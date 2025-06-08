use crate::game::GameLayer;
use crate::game::actor::ActorAssets;
use crate::game::actor::camera_cutie::{CameraCutieEvent, send_camera_follow_event};
use crate::game::actor::enemy::get_enemy;
use crate::game::actor::movement::spring::mass_spring_damper;
use crate::game::actor::player::get_player;
use crate::game::world::level_gen::Map;
use crate::prelude::*;
use crate::screen::Screen;

pub mod level_gen;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(LevelAssets, Level)>();
    app.add_plugins(level_gen::plugin);
}

#[derive(AssetCollection, Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[asset(path = "maps/World_H.world")]
    hub_assets: Handle<TiledWorld>,
    #[asset(path = "maps/World_X.world")]
    x_assets: Handle<TiledWorld>,
    #[asset(path = "maps/Dungeon.world")]
    dungeon_assets: Handle<TiledWorld>,
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
    let map = Map::shaped(50);
    map.save_world_file();
    let y_offset = map.y_offset();
    commands.spawn((
        TiledWorldHandle(world_assets.dungeon_assets.clone()),
        TilemapAnchor::None,
        TiledMapLayerZOffset(0.),
        RigidBody::Static,
        CollisionLayers::new(GameLayer::Wall, LayerMask::ALL),
        DespawnOnExitState::<Level>::default(),
        Transform::from_xyz(0., y_offset, 0.),
    ));

    let player_spawn_commands = commands.spawn((
        get_player(actor_assets.rat_handle.clone()),
        Transform::from_xyz(480., 320., 5.),
        DespawnOnExitState::<Level>::default(),
    ));

    send_camera_follow_event(player_spawn_commands.id(), set_camera_event);

    commands.spawn((
        get_enemy("Orc", actor_assets.orc_image.clone()),
        mass_spring_damper(100., 40_000., 4_000., Vec2::new(-256., -128.)),
        Transform::from_xyz(-256., -128., -2.),
        DespawnOnExitState::<Screen>::Recursive,
    ));
}
pub fn despawn() {}
