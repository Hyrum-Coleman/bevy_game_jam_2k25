use core::f32;

use crate::core::audio::AudioSettings;
use crate::core::audio::music_audio;
use crate::core::camera::SmoothFollow;
use crate::menu::Menu;
use crate::prelude::*;
use crate::screen::Screen;
use crate::screen::ScreenRoot;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Screen::Gameplay.on_enter(spawn_gameplay_screen));

    app.configure::<(GameplayAssets, GameplayAction)>();
}

const TILE_SIZE: f32 = 32.0;
const ROOM_WIDTH_TILES: usize = 40;
const ROOM_HEIGHT_TILES: usize = 22;

fn spawn_gameplay_screen(
    mut commands: Commands,
    _screen_root: Res<ScreenRoot>,
    audio_settings: Res<AudioSettings>,
    assets: Res<GameplayAssets>,
) {
    commands.spawn((
        music_audio(&audio_settings, assets.music.clone()),
        DespawnOnExitState::<Screen>::Recursive,
    ));
    for x in 0..ROOM_WIDTH_TILES {
        for y in 0..ROOM_HEIGHT_TILES {
            let is_wall =
                x == 0 || y == 0 || x == ROOM_WIDTH_TILES - 1 || y == ROOM_HEIGHT_TILES - 1;
            if is_wall {
                let pos = Vec2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE);

                commands.spawn((
                    Sprite {
                        image: assets.wall_sprite.clone(),
                        custom_size: Some(Vec2::splat(TILE_SIZE)),
                        ..default()
                    },
                    Transform::from_xyz(pos.x, pos.y, 1.0),
                    Collider::rectangle(TILE_SIZE, TILE_SIZE),
                    RigidBody::Static,
                    DespawnOnExitState::<Screen>::Recursive,
                ));
            };
        }
    }
    commands.spawn((
        Sprite {
            image: assets.floor_sprite.clone(),
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
        Sprite {
            image: assets.character_sprite.clone(),
            ..default()
        },
        Transform::from_xyz(128., 128., 1.),
        Player {
            movement_direction: Vec3::default(),
        },
        RigidBody::Dynamic,
        Collider::rectangle(32.0, 64.0),
        DespawnOnExitState::<Screen>::Recursive,
        LockedAxes::ROTATION_LOCKED,
    ));
    commands.spawn((
        Sprite {
            image: assets.orc_sprite.clone(),
            ..default()
        },
        Orc,
        DespawnOnExitState::<Screen>::Recursive,
    ));
}

#[derive(Component)]
struct Player {
    movement_direction: Vec3,
}

#[derive(Component)]
struct Orc;

#[derive(AssetCollection, Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct GameplayAssets {
    #[asset(path = "audio/music/summer.ogg")]
    music: Handle<AudioSource>,
    #[asset(path = "image/tile.png")]
    floor_sprite: Handle<Image>,
    #[asset(path = "image/wall.png")]
    wall_sprite: Handle<Image>,
    #[asset(path = "image/Player.png")]
    character_sprite: Handle<Image>,
    #[asset(path = "image/Orc_Guy.png")]
    orc_sprite: Handle<Image>,
}

impl Configure for GameplayAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
}

#[derive(Actionlike, Reflect, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum GameplayAction {
    Pause,
    CloseMenu,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
}

// Walking Speed is in ft/s (1ft=12px)
const WALKING_SPEED_FEET_PER_SECOND: f32 = 7.0;

const WALKING_SPEED_PIXELS_PER_SECOND: f32 = 12.0 * WALKING_SPEED_FEET_PER_SECOND;

const SPRINT_MULTIPLIER: f32 = 2.0;

//ft/s^2
const DEACCELERATION_RATE_FEET: f32 = 50.0;
const DEACCELERATION_RATE_PIXELS: f32 = DEACCELERATION_RATE_FEET * 12.0;

impl Configure for GameplayAction {
    fn configure(app: &mut App) {
        app.init_resource::<ActionState<Self>>();
        app.insert_resource(
            InputMap::default()
                .with(Self::Pause, GamepadButton::Start)
                .with(Self::Pause, KeyCode::Escape)
                .with(Self::Pause, KeyCode::KeyP)
                .with(Self::CloseMenu, KeyCode::KeyP)
                .with(Self::MoveUp, KeyCode::KeyW)
                .with(Self::MoveLeft, KeyCode::KeyA)
                .with(Self::MoveDown, KeyCode::KeyS)
                .with(Self::MoveRight, KeyCode::KeyD),
        );
        app.add_plugins(InputManagerPlugin::<Self>::default());
        app.add_systems(
            Update,
            Screen::Gameplay.on_update((
                (spawn_pause_overlay, Menu::Pause.enter())
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(Menu::is_disabled.and(action_just_pressed(Self::Pause))),
                Menu::clear
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(Menu::is_enabled.and(action_just_pressed(Self::CloseMenu))),
                prep_move_right
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(action_pressed(Self::MoveRight)),
                prep_move_left
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(action_pressed(Self::MoveLeft)),
                prep_move_up
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(action_pressed(Self::MoveUp)),
                prep_move_down
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(action_pressed(Self::MoveDown)),
                move_player.in_set(UpdateSystems::Update),
                camera_follow_player.in_set(UpdateSystems::SyncEarly),
                slow_speed.in_set(UpdateSystems::Update).run_if(
                    action_pressed(Self::MoveDown).nand(action_pressed(Self::MoveUp).nand(
                        action_pressed(Self::MoveLeft).nand(action_pressed(Self::MoveRight)),
                    )),
                ),
            )),
        );
    }
}

fn prep_move_right(query: Query<&mut Player>) {
    prep_move(query, Vec3::X * 1.0);
}

fn prep_move_left(query: Query<&mut Player>) {
    prep_move(query, Vec3::X * -1.0);
}

fn prep_move_up(query: Query<&mut Player>) {
    prep_move(query, Vec3::Y * 1.0);
}

fn prep_move_down(query: Query<&mut Player>) {
    prep_move(query, Vec3::Y * -1.0);
}

fn prep_move(mut query: Query<&mut Player>, direction: Vec3) {
    if let Ok(mut player) = query.single_mut() {
        player.movement_direction += direction;
    } else {
        warn!("Player not found");
    }
}

fn move_player(
    mut player_query: Query<&mut Player>,
    mut transform_query: Query<(&mut LinearVelocity, &mut Sprite), With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    transform_query
        .iter_mut()
        .for_each(|(mut velocity, mut sprite)| {
            let mut player = rq!(player_query.single_mut());
            let direction = player.movement_direction.normalize_or_zero() * calculate_speed(&keys);

            if direction == Vec3::ZERO {
                return;
            }

            velocity.x = direction.x;
            velocity.y = direction.y;
            let flip = match direction.x.partial_cmp(&0.0).unwrap() {
                std::cmp::Ordering::Less => false,
                std::cmp::Ordering::Equal => false,
                std::cmp::Ordering::Greater => true,
            };
            sprite.flip_x = flip;
            player.movement_direction = Vec3::ZERO;
        })
}

fn slow_speed(mut transform_query: Query<&mut LinearVelocity, With<Player>>, time: Res<Time>) {
    transform_query.iter_mut().for_each(|mut velocity| {
        if velocity.x != 0.0 {
            let sign_x = velocity.x.signum();
            velocity.x -= DEACCELERATION_RATE_PIXELS * time.delta_secs() * sign_x;
            if velocity.x.signum() != sign_x {
                velocity.x = 0.0
            }
        }
        if velocity.y != 0.0 {
            let sign_y = velocity.y.signum();
            velocity.y -= DEACCELERATION_RATE_PIXELS * time.delta_secs() * sign_y;
            if velocity.y.signum() != sign_y {
                velocity.y = 0.0
            }
        }
    });
}

fn calculate_speed(keys: &Res<ButtonInput<KeyCode>>) -> f32 {
    if keys.pressed(KeyCode::ShiftLeft) {
        WALKING_SPEED_PIXELS_PER_SECOND * SPRINT_MULTIPLIER
    } else {
        WALKING_SPEED_PIXELS_PER_SECOND
    }
}

fn spawn_pause_overlay(mut commands: Commands) {
    commands.spawn((
        widget::blocking_overlay(1),
        ThemeColor::Overlay.set::<BackgroundColor>(),
        DespawnOnExitState::<Screen>::default(),
        DespawnOnDisableState::<Menu>::default(),
    ));
}

fn camera_follow_player(
    mut camera_query: Query<&mut SmoothFollow>,
    player_query: Query<Entity, With<Player>>,
) {
    if let Ok(player) = player_query.single() {
        if let Ok(mut follow) = camera_query.single_mut() {
            follow.target = player;
        }
    }
}
