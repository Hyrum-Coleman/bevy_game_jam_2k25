use crate::game::actor::ActorAssets;
use crate::game::actor::player::get_player;
use crate::prelude::*;
use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(WorldAssets, Level)>();
}

#[derive(AssetCollection, Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct WorldAssets {
    #[asset(path = "image/tile.png")]
    floor_sprite: Handle<Image>,
    #[asset(path = "image/wall.png")]
    wall_sprite: Handle<Image>,
}

impl Configure for WorldAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
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
    world: NextRef<Level>,
    world_assets: Res<WorldAssets>,
    actor_assets: Res<ActorAssets>,
) {
    commands.spawn((
        Sprite {
            image: world_assets.floor_sprite.clone(),
            custom_size: Some(Vec2::new(2560., 1440.)),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: 1.0,
            },
            ..default()
        },
        Transform::from_xyz(0., 0., -5.),
        DespawnOnExitState::<Screen>::Recursive,
    ));

    commands.spawn((
        get_player(actor_assets.player_image.clone()),
        DespawnOnExitState::<Level>::default(),
        Transform::from_xyz(64., 64., 2.),
    ));

    commands.spawn((
        Sprite {
            image: actor_assets.orc_image.clone(),
            ..default()
        },
        DespawnOnExitState::<Screen>::Recursive,
    ));
}

pub fn despawn() {}
