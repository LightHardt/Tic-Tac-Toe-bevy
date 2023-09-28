use bevy::prelude::*;
use bevy_text_popup::TextPopupPlugin;

mod game;
mod systems;
mod components;

use systems::*;

/**
 * TicTacToePlugin has the needed systems to run the TicTacToe game
 */
pub struct TicTacToePlugin;

impl Plugin for TicTacToePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(TextPopupPlugin)
            .add_systems(Startup, setup)
            .add_systems(Update, board_system)
            .add_systems(Update, ai_move_button.after(board_system)); // use after to make sure board is updated first before AI move to ensure no double move
    }
}