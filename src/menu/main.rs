use crate::menu::Menu;
use crate::menu::MenuRoot;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Menu::Main.on_enter(spawn_main_menu));
}

#[cfg_attr(feature = "native_dev", hot)]
fn spawn_main_menu(mut commands: Commands, menu_root: Res<MenuRoot>) {
    commands
        .entity(menu_root.ui)
        .with_child(widget::body(children![
            widget::header("[b]Bevy Game Jam 2k25"),
            widget::column_of_buttons(children![
                widget::big_button("Play", open_intro),
                widget::big_button("Settings", open_settings),
                (
                    widget::big_button("Quit", quit_to_desktop),
                    #[cfg(feature = "web")]
                    InteractionDisabled(true),
                ),
            ]),
        ]));
}

fn open_intro(_: Trigger<Pointer<Click>>, mut menu: ResMut<NextStateStack<Menu>>) {
    menu.push(Menu::Intro);
}

fn open_settings(_: Trigger<Pointer<Click>>, mut menu: ResMut<NextStateStack<Menu>>) {
    menu.push(Menu::Settings);
}

fn quit_to_desktop(_: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>) {
    if cfg!(not(feature = "web")) {
        app_exit.write(AppExit::Success);
    }
}
