use crate::game::GameLayer;
use crate::game::actor::ActorAssets;
use crate::game::actor::camera_cutie::{CameraCutieEvent, send_camera_follow_event};
use crate::game::actor::enemy::{get_enemy, get_enemy_aseprite};
use crate::game::actor::movement::spring::mass_spring_damper;
use crate::game::actor::player::get_player;
use crate::prelude::*;
use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(LevelAssets, Level)>();
}

#[derive(AssetCollection, Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[asset(path = "maps/Dungeon.world")]
    hub_assets: Handle<TiledWorld>,
    #[asset(path = "maps/World_H/World_H_Center.tmx")]
    x_assets: Handle<TiledMap>,
}

impl Configure for LevelAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_asset::<TiledMap>();
        #[cfg(feature = "web")]
        app.add_plugins(TiledMapPlugin(TiledMapPluginConfig {
            tiled_types_export_file: None,
        }));
        #[cfg(not(feature = "web"))]
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
        TiledWorldHandle(world_assets.hub_assets.clone()),
        TilemapAnchor::None,
        TiledMapLayerZOffset(0.),
        RigidBody::Static,
        CollisionLayers::new(GameLayer::Wall, LayerMask::ALL),
        DespawnOnExitState::<Level>::default(),
    ));

    let player_spawn_commands = commands.spawn((
        get_player(actor_assets.rat_handle.clone()),
        Transform::from_xyz(475., 330., 5.),
        DespawnOnExitState::<Level>::default(),
    ));

    send_camera_follow_event(player_spawn_commands.id(), set_camera_event);

    commands.spawn((
        get_enemy("Orc", actor_assets.orc_image.clone()),
        mass_spring_damper(500., 1_000_000., 20_000., Vec2::new(212., 463.)),
        Transform::from_xyz(212., 463., 5.),
        DespawnOnExitState::<Screen>::Recursive,
    ));

    commands.spawn((
        get_enemy_aseprite(
            "Blob Cannon",
            actor_assets.cannon.clone(),
            "Idle",
            100.,
            32.,
        ),
        Transform::from_xyz(212., 420., 5.),
        DespawnOnExitState::<Screen>::Recursive,
    ));

    commands.spawn((
        get_enemy_aseprite(
            "Mouse Boss",
            actor_assets.mouse_boss.clone(),
            "Move Down",
            400.,
            96.,
        ),
        Transform::from_xyz(200., 600., 5.),
        DespawnOnExitState::<Screen>::Recursive,
    ));

    commands.spawn((
        get_enemy_aseprite("Mouse", actor_assets.mouse.clone(), "move down", 25., 32.),
        Transform::from_xyz(200., 100., 5.),
        DespawnOnExitState::<Screen>::Recursive,
    ));

    commands.spawn((
        get_enemy_aseprite(
            "Cheese Item",
            actor_assets.exp_cheese.clone(),
            "Frame",
            15.,
            16.,
        ),
        Transform::from_xyz(400., 463., 5.),
        DespawnOnExitState::<Screen>::Recursive,
    ));
}
pub fn despawn() {}
