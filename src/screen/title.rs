use crate::core::audio::AudioSettings;
use crate::core::audio::music_audio;
use crate::menu::Menu;
use crate::prelude::*;
use crate::screen::Screen;
use crate::screen::gameplay::load_collections;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(load_collections(LoadingState::new(Screen::Title.bevy())));
    app.add_systems(
        StateFlush,
        Screen::Title.on_enter((
            (Menu::Main.enter(), Menu::acquire).chain(),
            spawn_title_screen,
        )),
    );

    app.configure::<TitleAssets>();
}

fn spawn_title_screen(
    mut commands: Commands,
    audio_settings: Res<AudioSettings>,
    assets: Res<TitleAssets>,
) {
    commands.spawn((
        music_audio(&audio_settings, assets.music.clone()),
        DespawnOnExitState::<Screen>::Recursive,
    ));
}

#[derive(AssetCollection, Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct TitleAssets {
    #[asset(path = "audio/music/Marimba_Duet.ogg")]
    music: Handle<AudioSource>,
}

impl Configure for TitleAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
}
