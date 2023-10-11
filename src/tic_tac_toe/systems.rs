use bevy::prelude::*;
use bevy_text_popup::{TextPopupEvent, TextPopupTimeout, TextPopupButton};
use rand::Rng;

use super::components::*;
use super::game;
use super::game::*;
use super::styles::*;

/**
 * A function that will use the queries from the board_system
 * function and remove the text on them to show the board is in a fresh
 * state
 */
fn clear_board(
    button1_query: &Query<&Children,With<Marker1>>,
    button2_query: &Query<&Children,With<Marker2>>,
    button3_query: &Query<&Children,With<Marker3>>,
    button4_query: &Query<&Children,With<Marker4>>,
    button5_query: &Query<&Children,With<Marker5>>,
    button6_query: &Query<&Children,With<Marker6>>,
    button7_query: &Query<&Children,With<Marker7>>,
    button8_query: &Query<&Children,With<Marker8>>,
    button9_query: &Query<&Children,With<Marker9>>,
    text_query: &mut Query<&mut Text>,

) {
    for button in button1_query{
        let mut text = text_query.get_mut(button[0]).unwrap();
        text.sections[0].value = "".to_string();
    }
    for button in button2_query{
        let mut text = text_query.get_mut(button[0]).unwrap();
        text.sections[0].value = "".to_string();
    }
    for button in button3_query{
        let mut text = text_query.get_mut(button[0]).unwrap();
        text.sections[0].value = "".to_string();
    }
    for button in button4_query{
        let mut text = text_query.get_mut(button[0]).unwrap();
        text.sections[0].value = "".to_string();
    }
    for button in button5_query{
        let mut text = text_query.get_mut(button[0]).unwrap();
        text.sections[0].value = "".to_string();
    }
    for button in button6_query{
        let mut text = text_query.get_mut(button[0]).unwrap();
        text.sections[0].value = "".to_string();
    }
    for button in button7_query{
        let mut text = text_query.get_mut(button[0]).unwrap();
        text.sections[0].value = "".to_string();
    }
    for button in button8_query{
        let mut text = text_query.get_mut(button[0]).unwrap();
        text.sections[0].value = "".to_string();
    }
    for button in button9_query{
        let mut text = text_query.get_mut(button[0]).unwrap();
        text.sections[0].value = "".to_string();
    }
}

/**
 * This will send a Draw popup to the window
 */
fn send_draw(text_popup: &mut EventWriter<TextPopupEvent>) {
    text_popup.send(TextPopupEvent {
        content: "Draw!".to_string(),
        modal: Some(Color::YELLOW),
        timeout: TextPopupTimeout::Seconds(5),
        dismiss_button: Some(TextPopupButton {
            text: "Close".to_string(),
            ..Default::default()
        }),
        ..default()
    });
}

/**
 * This will send a you won popup to the window
 */
fn send_won(text_popup: &mut EventWriter<TextPopupEvent>) {
    text_popup.send(TextPopupEvent {
        content: "You Won!".to_string(),
        modal: Some(Color::GREEN),
        timeout: TextPopupTimeout::Seconds(5),
        dismiss_button: Some(TextPopupButton {
            text: "Close".to_string(),
            ..Default::default()
        }),
        ..default()
    });
}

/**
 * This will send a you lost popup to the window
 */
fn send_lost(text_popup: &mut EventWriter<TextPopupEvent>) {
    text_popup.send(TextPopupEvent {
        content: "You Lost!".to_string(),
        modal: Some(Color::RED),
        timeout: TextPopupTimeout::Seconds(5),
        dismiss_button: Some(TextPopupButton {
            text: "Close".to_string(),
            ..Default::default()
        }),
        ..default()
    });
}

/**
 * Mark on the appropriate button where the ai has decided to move on
 * the board
 */
fn mark_ai_move(
    ai: i8,
    button1_query: &Query<&Children,With<Marker1>>,
    button2_query: &Query<&Children,With<Marker2>>,
    button3_query: &Query<&Children,With<Marker3>>,
    button4_query: &Query<&Children,With<Marker4>>,
    button5_query: &Query<&Children,With<Marker5>>,
    button6_query: &Query<&Children,With<Marker6>>,
    button7_query: &Query<&Children,With<Marker7>>,
    button8_query: &Query<&Children,With<Marker8>>,
    button9_query: &Query<&Children,With<Marker9>>,
    text_query: &mut Query<&mut Text>,
) {
    match ai {
        1 => {
            for button in button1_query {
                let mut text = text_query.get_mut(button[0]).unwrap();
                text.sections[0].value = AI_SYM.to_string();
            }
        },
        2 => {
            for button in button2_query {
                let mut text = text_query.get_mut(button[0]).unwrap();
                text.sections[0].value = AI_SYM.to_string();
            }
        },
        3 => {
            for button in button3_query {
                let mut text = text_query.get_mut(button[0]).unwrap();
                text.sections[0].value = AI_SYM.to_string();
            }
        },
        4 => {
            for button in button4_query {
                let mut text = text_query.get_mut(button[0]).unwrap();
                text.sections[0].value = AI_SYM.to_string();
            }
        },
        5 => {
            for button in button5_query {
                let mut text = text_query.get_mut(button[0]).unwrap();
                text.sections[0].value = AI_SYM.to_string();
            }
        },
        6 => {
            for button in button6_query {
                let mut text = text_query.get_mut(button[0]).unwrap();
                text.sections[0].value = AI_SYM.to_string();
            }
        },
        7 => {
            for button in button7_query {
                let mut text = text_query.get_mut(button[0]).unwrap();
                text.sections[0].value = AI_SYM.to_string();
            }
        },
        8 => {
            for button in button8_query {
                let mut text = text_query.get_mut(button[0]).unwrap();
                text.sections[0].value = AI_SYM.to_string();
            }
        },
        9 => {
            for button in button9_query {
                let mut text = text_query.get_mut(button[0]).unwrap();
                text.sections[0].value = AI_SYM.to_string();
            }
        },
        _ => panic!("AI move error")
    }
}

/**
 * This system will handle the AI moving first by checking if the board
 * is empty and if so make a move
 */
pub fn ai_move_button(
    mut interaction_query: Query<(
        &Interaction,
        &mut BorderColor,
        Option<&AiMove>,
    ),
    (Changed<Interaction>, With<Button>),
    >,
    button1_query: Query<&Children,With<Marker1>>,
    button2_query: Query<&Children,With<Marker2>>,
    button3_query: Query<&Children,With<Marker3>>,
    button4_query: Query<&Children,With<Marker4>>,
    button5_query: Query<&Children,With<Marker5>>,
    button6_query: Query<&Children,With<Marker6>>,
    button7_query: Query<&Children,With<Marker7>>,
    button8_query: Query<&Children,With<Marker8>>,
    button9_query: Query<&Children,With<Marker9>>,
    mut text_query: Query<&mut Text>,
    mut game_query: Query<&mut Game>,
) {
    for (interaction, mut border_color, ai_button_check) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if ai_button_check.is_some() {
                    for mut game_component in &mut game_query {
                        let mut first_move_check = true;
                        for i in game_component.board {
                            for j in i {
                                if j != 0  { first_move_check = false; }
                            }
                        }
                        if !first_move_check {
                            break;
                        }
                        let ai_choice = rand::thread_rng().gen_range(1..10);
                        player_move(AI, ai_choice, &mut game_component.board);
                        mark_ai_move(ai_choice, &button1_query, &button2_query, &button3_query, &button4_query, 
                            &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                    }
                }
            },
            Interaction::Hovered => {
                border_color.0 = Color::WHITE; // dont need to check as this behavior is same for all buttons
            },
            Interaction::None => {
                if ai_button_check.is_some() {
                    border_color.0 = Color::BLACK;
                }
            },
        }
    }
}

/**
 * Board system handles the TicTacToe game loop by handling the interaction with the
 * 3x3 grid of buttons and calling the appropriate functions for the game
 */
pub fn board_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            (Option<&Marker1>,Option<&Marker2>,Option<&Marker3>,
                Option<&Marker4>,Option<&Marker5>,Option<&Marker6>,
                Option<&Marker7>,Option<&Marker8>,Option<&Marker9>),
        ),
        (Changed<Interaction>, With<Button>, Without<AiMove>),
    >,
    button1_query: Query<&Children,With<Marker1>>,
    button2_query: Query<&Children,With<Marker2>>,
    button3_query: Query<&Children,With<Marker3>>,
    button4_query: Query<&Children,With<Marker4>>,
    button5_query: Query<&Children,With<Marker5>>,
    button6_query: Query<&Children,With<Marker6>>,
    button7_query: Query<&Children,With<Marker7>>,
    button8_query: Query<&Children,With<Marker8>>,
    button9_query: Query<&Children,With<Marker9>>, // Add all buttons to this query as will make AI move after human move
    mut game_query: Query<&mut Game>,
    mut text_query: Query<&mut Text>,
    mut text_popup: EventWriter<TextPopupEvent>,
) {
    for (interaction, mut color, mut border_color, children, 
        (marker1, marker2,marker3, marker4,marker5, 
            marker6,marker7, marker8, marker9)) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if marker1.is_some() { // Use the markers to identify what button in the grid is being clicked
                    for mut game_component in &mut game_query {
                        if player_move(game_component.turn, 1, &mut game_component.board) { // Makes sure player move is valid and if makes it and continues
                            if is_draw(game_component.board) { // After all moves must check if the game is in a draw state
                                send_draw(&mut text_popup); // if draw send the appropriate popup
                                game::clear_board(&mut game_component.board); // clear the game board so can continue playing
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query,  // Clear the 3x3 grid text to show a clean board
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if is_game_over(game_component.turn, game_component.board) { // After all moves must check if the player who made the move won
                                send_won(&mut text_popup);
                                game::clear_board(&mut game_component.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            {
                                let mut text = text_query.get_mut(children[0]).unwrap(); // put in its own scope so can reborrow
                                text.sections[0].value = HUMAN_SYM.to_string();
                            }
                            let ai_choice = ai_move(AI, HUMAN, game_component.board); // After all that it is time for the ai to make its move
                            player_move(AI, ai_choice, &mut game_component.board); // No need to check as Ai can only make valid moves or panics

                            mark_ai_move(ai_choice, &button1_query, &button2_query, &button3_query, &button4_query, // Mark on 3x3 grid where the ai moved
                                &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);

                            if is_draw(game_component.board) { // Check if the ai move put the game in a draw state
                                send_draw(&mut text_popup);
                                game::clear_board(&mut game_component.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if is_game_over(AI, game_component.board) { // Check if the Ai made a winning move
                                send_lost(&mut text_popup);
                                game::clear_board(&mut game_component.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                        }
                    }
                }
                else if marker2.is_some() { // For the rest of the buttons the logic is the same as marked in button 1 above
                    for mut game_component in &mut game_query {
                        if player_move(game_component.turn, 2, &mut game_component.board) {
                            if is_game_over(game_component.turn, game_component.board) {
                                send_lost(&mut text_popup);
                                game::clear_board(&mut game_component.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if is_draw(game_component.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut game_component.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            {
                                let mut text = text_query.get_mut(children[0]).unwrap(); // put in its own scope so can reborrow
                                text.sections[0].value = HUMAN_SYM.to_string();
                            }
                            let ai_choice = ai_move(AI, HUMAN, game_component.board);
                            player_move(AI, ai_choice, &mut game_component.board);

                            mark_ai_move(ai_choice, &button1_query, &button2_query, &button3_query, &button4_query, 
                                &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);

                            if is_game_over(AI, game_component.board) {
                                send_lost(&mut text_popup);
                                game::clear_board(&mut game_component.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if is_draw(game_component.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut game_component.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                        }
                    }
                }
                else if marker3.is_some() {
                    for mut game_component in &mut game_query {
                        if player_move(game_component.turn, 3, &mut game_component.board) {
                            if is_game_over(game_component.turn, game_component.board) {
                                send_won(&mut text_popup);
                                game::clear_board(&mut game_component.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if is_draw(game_component.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut game_component.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            {
                                let mut text = text_query.get_mut(children[0]).unwrap(); // put in its own scope so can reborrow
                                text.sections[0].value = HUMAN_SYM.to_string();
                            }
                            let ai_choice = ai_move(AI, HUMAN, game_component.board);
                            player_move(AI, ai_choice, &mut game_component.board);

                            mark_ai_move(ai_choice, &button1_query, &button2_query, &button3_query, &button4_query, 
                                &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);

                            if is_game_over(AI, game_component.board) {
                                send_lost(&mut text_popup);
                                game::clear_board(&mut game_component.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if is_draw(game_component.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut game_component.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                        }
                    }
                }
                else if marker4.is_some() {
                    for mut game_component in &mut game_query {
                        if player_move(game_component.turn, 4, &mut game_component.board) {
                            if is_game_over(game_component.turn, game_component.board) {
                                send_won(&mut text_popup);
                                game::clear_board(&mut game_component.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if is_draw(game_component.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut game_component.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            {
                                let mut text = text_query.get_mut(children[0]).unwrap(); // put in its own scope so can reborrow
                                text.sections[0].value = HUMAN_SYM.to_string();
                            }
                            let ai_choice = ai_move(AI, HUMAN, game_component.board);
                            player_move(AI, ai_choice, &mut game_component.board);

                            mark_ai_move(ai_choice, &button1_query, &button2_query, &button3_query, &button4_query, 
                                &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);

                            if is_game_over(AI, game_component.board) {
                                send_lost(&mut text_popup);
                                game::clear_board(&mut game_component.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if is_draw(game_component.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut game_component.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                        }
                    }
                }
                else if marker5.is_some() {
                    for mut game_component in &mut game_query {
                        if player_move(game_component.turn, 5, &mut game_component.board) {
                            if is_game_over(game_component.turn, game_component.board) {
                                send_won(&mut text_popup);
                                game::clear_board(&mut game_component.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if is_draw(game_component.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut game_component.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            {
                                let mut text = text_query.get_mut(children[0]).unwrap(); // put in its own scope so can reborrow
                                text.sections[0].value = HUMAN_SYM.to_string();
                            }
                            let ai_choice = ai_move(AI, HUMAN, game_component.board);
                            player_move(AI, ai_choice, &mut game_component.board);

                            mark_ai_move(ai_choice, &button1_query, &button2_query, &button3_query, &button4_query, 
                                &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);

                            if is_game_over(AI, game_component.board) {
                                send_lost(&mut text_popup);
                                game::clear_board(&mut game_component.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if is_draw(game_component.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut game_component.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                        }
                    }
                }
                else if marker6.is_some() {
                    for mut game_component in &mut game_query {
                        if player_move(game_component.turn, 6, &mut game_component.board) {
                            if is_game_over(game_component.turn, game_component.board) {
                                send_won(&mut text_popup);
                                game::clear_board(&mut game_component.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if is_draw(game_component.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut game_component.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            {
                                let mut text = text_query.get_mut(children[0]).unwrap(); // put in its own scope so can reborrow
                                text.sections[0].value = HUMAN_SYM.to_string();
                            }
                            let ai_choice = ai_move(AI, HUMAN, game_component.board);
                            player_move(AI, ai_choice, &mut game_component.board);

                            mark_ai_move(ai_choice, &button1_query, &button2_query, &button3_query, &button4_query, 
                                &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);

                            if is_game_over(AI, game_component.board) {
                                send_lost(&mut text_popup);
                                game::clear_board(&mut game_component.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if is_draw(game_component.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut game_component.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                        }
                    }
                }
                else if marker7.is_some() {
                    for mut game_component in &mut game_query {
                        if player_move(game_component.turn, 7, &mut game_component.board) {
                            if is_game_over(game_component.turn, game_component.board) {
                                send_won(&mut text_popup);
                                game::clear_board(&mut game_component.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if is_draw(game_component.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut game_component.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            {
                                let mut text = text_query.get_mut(children[0]).unwrap(); // put in its own scope so can reborrow
                                text.sections[0].value = HUMAN_SYM.to_string();
                            }
                            let ai_choice = ai_move(AI, HUMAN, game_component.board);
                            player_move(AI, ai_choice, &mut game_component.board);

                            mark_ai_move(ai_choice, &button1_query, &button2_query, &button3_query, &button4_query, 
                                &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                            
                            if is_game_over(AI, game_component.board) {
                                send_lost(&mut text_popup);
                                game::clear_board(&mut game_component.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if is_draw(game_component.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut game_component.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                        }
                    }
                }
                else if marker8.is_some() {
                    for mut game_component in &mut game_query {
                        if player_move(game_component.turn, 8, &mut game_component.board) {
                            if is_game_over(game_component.turn, game_component.board) {
                                send_won(&mut text_popup);
                                game::clear_board(&mut game_component.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if is_draw(game_component.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut game_component.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            {
                                let mut text = text_query.get_mut(children[0]).unwrap(); // put in its own scope so can reborrow
                                text.sections[0].value = HUMAN_SYM.to_string();
                            }
                            let ai_choice = ai_move(AI, HUMAN, game_component.board);
                            player_move(AI, ai_choice, &mut game_component.board);

                            mark_ai_move(ai_choice, &button1_query, &button2_query, &button3_query, &button4_query, 
                                &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);

                            if is_game_over(AI, game_component.board) {
                                send_lost(&mut text_popup);
                                game::clear_board(&mut game_component.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if is_draw(game_component.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut game_component.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                        }
                    }
                }
                else if marker9.is_some() {
                    for mut game_component in &mut game_query {
                        if player_move(game_component.turn, 9, &mut game_component.board) {
                            if is_game_over(game_component.turn, game_component.board) {
                                send_won(&mut text_popup);
                                game::clear_board(&mut game_component.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if is_draw(game_component.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut game_component.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            {
                                let mut text = text_query.get_mut(children[0]).unwrap(); // put in its own scope so can reborrow
                                text.sections[0].value = HUMAN_SYM.to_string();
                            }
                            let ai_choice = ai_move(AI, HUMAN, game_component.board);
                            player_move(AI, ai_choice, &mut game_component.board);

                            mark_ai_move(ai_choice, &button1_query, &button2_query, &button3_query, &button4_query, 
                                &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);

                            if is_game_over(AI, game_component.board) {
                                send_lost(&mut text_popup);
                                game::clear_board(&mut game_component.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if is_draw(game_component.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut game_component.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                        }
                    }
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}
