#![windows_subsystem = "windows"] // hide terminal, also comment this out when debugging

use bevy::{prelude::*, winit::WinitSettings};
mod tic_tac_toe;

use tic_tac_toe::TicTacToePlugin;

fn main() {
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: String::from("Tic Tac Toe"),
            resize_constraints: WindowResizeConstraints { 
                max_width: 550.0,
                max_height: 600.0,
                ..default()
            },
            ..default()
        }),
        ..default()
        };

    App::new()
        .add_plugins((DefaultPlugins.set(window_plugin),TicTacToePlugin))
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .run();
}

