//
// MECHWARRIOR - THE UNKNOWN MERC
//
use bevy::prelude::*;
use bevy::app::AppExit;

// Modules can be included in TWO ways:
mod console {
    include!("console/entry_point.rs");
}
mod controls;   // Looks automatically for 'controls/mod.rs'

// Import public functions from local modules:
use console::*;

// Crate global constants
pub const APP_NAME: &str = "Mechwarrior - The Unknown Merch";


fn main() {
    // Here we go - the Bevy way:
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: APP_NAME.to_string(),
            ..default()
        }),
        ..default()
    }))
    .add_startup_system(print_start_message)
    // .add_system(bevy::window::close_on_esc)
    .add_system(quit)
    .run();

    // Nothing after the Bevy app seems to get executed
    println!("THIS END OF MAIN IS NEWER PRINTED");

}

fn quit(
    keys: Res<Input<KeyCode>>,
    mut exit: EventWriter<AppExit>
) {
    if (keys.pressed(KeyCode::LControl) || keys.pressed(KeyCode::RControl)) && keys.pressed(KeyCode::Q) {
        print_end_message();
        exit.send(AppExit);
    }
}
