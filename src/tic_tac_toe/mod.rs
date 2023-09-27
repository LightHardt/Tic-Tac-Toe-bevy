use bevy::prelude::*;

mod game;
mod systems;
mod components;

use systems::*;

pub struct TicTacToePlugin;

impl Plugin for TicTacToePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, button_system);
    }
}