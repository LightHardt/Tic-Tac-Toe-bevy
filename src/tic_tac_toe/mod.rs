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
            .add_systems(Update, board_system)
            .add_systems(Update, ai_move_button.after(board_system)); // use after to make sure board is updated first before AI move to ensure no double move
    }
}