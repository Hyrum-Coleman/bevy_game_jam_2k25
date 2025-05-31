use crate::core::audio::AudioSettings;
use crate::core::audio::music_audio;
use crate::menu::Menu;
use crate::prelude::*;
use crate::screen::Screen;
use crate::screen::ScreenRoot;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Screen::Gameplay.on_enter(spawn_gameplay_screen));

    app.configure::<(GameplayAssets, GameplayAction)>();
}

fn spawn_gameplay_screen(
    mut commands: Commands,
    screen_root: Res<ScreenRoot>,
    audio_settings: Res<AudioSettings>,
    assets: Res<GameplayAssets>,
) {
    commands
        .entity(screen_root.ui)
        .with_child(widget::column_center(children![widget::label(
            "Gameplay goes here. Press P to pause!",
        )]));
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
        Player,
        DespawnOnExitState::<Screen>::Recursive,
    ));
}

#[derive(Component)]
struct Player;

#[derive(AssetCollection, Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct GameplayAssets {
    #[asset(path = "audio/music/summer.ogg")]
    music: Handle<AudioSource>,
    #[asset(path = "image/Player.png")]
    character_sprite: Handle<Image>,
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
    Sprint,
}

const WALKING_SPEED: f32 = 1.0;

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
                .with(Self::MoveRight, KeyCode::KeyD)
                .with(Self::Sprint, KeyCode::ShiftLeft),
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
                move_player_right
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(action_pressed(Self::MoveRight)),
                move_player_left
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(action_pressed(Self::MoveLeft)),
                move_player_up
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(action_pressed(Self::MoveUp)),
                move_player_down
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(action_pressed(Self::MoveDown)),
                
            )),
        );
    }
}

fn move_player_right(query: Query<&mut Transform, With<Player>>) {
    move_player(query, vec3(WALKING_SPEED, 0.0, 0.0));
}

fn move_player_left(query: Query<&mut Transform, With<Player>>) {
    move_player(query, vec3(-WALKING_SPEED, 0.0, 0.0));
}

fn move_player_up(query: Query<&mut Transform, With<Player>>) {
    move_player(query, vec3(0.0, WALKING_SPEED, 0.0));
}

fn move_player_down(query: Query<&mut Transform, With<Player>>) {
    move_player(query, vec3(0.0, -WALKING_SPEED, 0.0));
}

fn move_player(mut query: Query<&mut Transform, With<Player>>, direction: Vec3) {
    query.iter_mut().for_each(|mut transform| {
        transform.translation += direction;
    })
}

fn spawn_pause_overlay(mut commands: Commands) {
    commands.spawn((
        widget::blocking_overlay(1),
        ThemeColor::Overlay.set::<BackgroundColor>(),
        DespawnOnExitState::<Screen>::default(),
        DespawnOnDisableState::<Menu>::default(),
    ));
}
