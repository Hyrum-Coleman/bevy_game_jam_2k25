use crate::core::audio::AudioSettings;
use crate::core::audio::music_audio;
use crate::menu::Menu;
use crate::prelude::*;
use crate::screen::Screen;
use crate::screen::ScreenRoot;
use crate::core::camera::SmoothFollow;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Screen::Gameplay.on_enter(spawn_gameplay_screen));

    app.configure::<(GameplayAssets, GameplayAction)>();
}

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
    commands.spawn((
        Sprite {
            image: assets.character_sprite.clone(),
            texture_atlas: None,
            color: Default::default(),
            flip_x: false,
            flip_y: false,
            custom_size: None,
            rect: None,
            anchor: Default::default(),
            image_mode: Default::default(),
        },
        Transform::from_xyz(0., 0., 1.),
        Player {
            movement_direction: Vec3::default(),
        },
        DespawnOnExitState::<Screen>::Recursive,
    ));
    commands.spawn((
        Sprite {
            image: assets.orc_sprite.clone(),
            texture_atlas: None,
            color: Default::default(),
            flip_x: false,
            flip_y: false,
            custom_size: None,
            rect: None,
            anchor: Default::default(),
            image_mode: Default::default(),
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

const WALKING_SPEED_PIXELS_PER_SECOND: f32 = 12.0*WALKING_SPEED_FEET_PER_SECOND;


const SPRINT_MULTIPLIER: f32 = 2.0;

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
                camera_follow_player.in_set(UpdateSystems::SyncEarly)
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
    time: Res<Time>,
    mut player_query: Query<&mut Player>,
    mut transform_query: Query<&mut Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    transform_query.iter_mut().for_each(|mut transform| {
        if let Ok(mut player) = player_query.single_mut() {
            let direction = player.movement_direction.normalize_or_zero() * calculate_speed(&keys) * time.delta_secs();

            if direction == Vec3::ZERO {
                return;
            }

            transform.translation += direction;
            info!("Moved player to {}", transform.translation);
            player.movement_direction = Vec3::ZERO;
        }
    })
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
