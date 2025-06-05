use crate::game::GameLayer;
use crate::game::actor::ActorAssets;
use crate::game::actor::camera_cutie::{CameraCutieEvent, send_camera_follow_event};
use crate::game::actor::enemy::get_enemy;
use crate::game::actor::player::get_player;
use crate::prelude::*;
use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(LevelAssets, Level)>();
}

#[derive(AssetCollection, Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[asset(path = "maps/World_H_Map.tmx")]
    map_assets: Handle<TiledMap>,
}

impl Configure for LevelAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_asset::<TiledMap>();
        app.add_plugins(TiledMapPlugin::default());
        app.add_plugins(TiledPhysicsPlugin::<TiledPhysicsAvianBackend>::default());
        app.init_collection::<Self>();
    }
}

#[derive(State, Reflect, Copy, Clone, Default, Eq, PartialEq, Debug)]
#[state(log_flush, react)]
#[reflect(Resource)]
pub struct Level(pub usize);

impl Configure for Level {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_state::<Self>();
        app.add_systems(StateFlush, Level::ANY.on_edge(despawn, spawn_world));
    }
}

pub fn spawn_world(
    mut commands: Commands,
    _world: NextRef<Level>,
    world_assets: Res<LevelAssets>,
    actor_assets: Res<ActorAssets>,
    set_camera_event: EventWriter<CameraCutieEvent>,
) {
    commands.spawn((
        TiledMapHandle(world_assets.map_assets.clone()),
        TilemapAnchor::Center,
        RigidBody::Static,
        CollisionLayers::new(GameLayer::Wall, LayerMask::ALL),
        DespawnOnExitState::<Level>::default(),
    ));

    let mut player_spawn_commands = commands.spawn((
        get_player(actor_assets.rat_handle.clone()),
        Transform::from_xyz(64., 0., 10.),
        DespawnOnExitState::<Level>::default(),
    ));
    player_spawn_commands.with_children(|children| {
        children.spawn((
            CollisionLayers::new(GameLayer::Player, LayerMask::ALL),
            Collider::rectangle(32., 16.),
            Transform::from_xyz(0.0, -24.0, 0.0),
            ColliderDensity(5.0),
        ));
    });

    send_camera_follow_event(player_spawn_commands.id(), set_camera_event);

    commands.spawn((
        get_enemy("Orc", actor_assets.orc_image.clone()),
        Transform::from_xyz(0., 0., 2.),
        DespawnOnExitState::<Screen>::Recursive,
    ));
}

pub fn despawn() {}
