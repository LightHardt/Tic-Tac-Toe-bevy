use bevy::prelude::*;
use bevy_text_popup::{TextPopupEvent, TextPopupTimeout, TextPopupButton};
pub mod game;

// Button Colors
pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);

// Use markers to mark buttons
#[derive(Component)]
pub struct Marker1;
#[derive(Component)]
pub struct Marker2;
#[derive(Component)]
pub struct Marker3;
#[derive(Component)]
pub struct Marker4;
#[derive(Component)]
pub struct Marker5;
#[derive(Component)]
pub struct Marker6;
#[derive(Component)]
pub struct Marker7;
#[derive(Component)]
pub struct Marker8;
#[derive(Component)]
pub struct Marker9;

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
                text.sections[0].value = game::AI_SYM.to_string();
            }
        },
        2 => {
            for button in button2_query {
                let mut text = text_query.get_mut(button[0]).unwrap();
                text.sections[0].value = game::AI_SYM.to_string();
            }
        },
        3 => {
            for button in button3_query {
                let mut text = text_query.get_mut(button[0]).unwrap();
                text.sections[0].value = game::AI_SYM.to_string();
            }
        },
        4 => {
            for button in button4_query {
                let mut text = text_query.get_mut(button[0]).unwrap();
                text.sections[0].value = game::AI_SYM.to_string();
            }
        },
        5 => {
            for button in button5_query {
                let mut text = text_query.get_mut(button[0]).unwrap();
                text.sections[0].value = game::AI_SYM.to_string();
            }
        },
        6 => {
            for button in button6_query {
                let mut text = text_query.get_mut(button[0]).unwrap();
                text.sections[0].value = game::AI_SYM.to_string();
            }
        },
        7 => {
            for button in button7_query {
                let mut text = text_query.get_mut(button[0]).unwrap();
                text.sections[0].value = game::AI_SYM.to_string();
            }
        },
        8 => {
            for button in button8_query {
                let mut text = text_query.get_mut(button[0]).unwrap();
                text.sections[0].value = game::AI_SYM.to_string();
            }
        },
        9 => {
            for button in button9_query {
                let mut text = text_query.get_mut(button[0]).unwrap();
                text.sections[0].value = game::AI_SYM.to_string();
            }
        },
        _ => panic!("AI move error")
    }
}

pub fn button_system(
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
    button9_query: Query<&Children,With<Marker9>>, // Add all buttons to this query as will make AI move after human move
    mut board_query: Query<&mut game::Game>,
    mut text_query: Query<&mut Text>,
    mut text_popup: EventWriter<TextPopupEvent>,
) {
    for (interaction, mut color, mut border_color, children, 
        (marker1, marker2,marker3, marker4,marker5, 
            marker6,marker7, marker8, marker9)) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if marker1.is_some() {
                    for mut b in &mut board_query {
                        if game::player_move(b.turn, 1, &mut b.board) {
                            if game::is_draw(b.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if game::is_game_over(b.turn, b.board) {
                                send_won(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            {
                                let mut text = text_query.get_mut(children[0]).unwrap(); // put in its own scope so can reborrow
                                text.sections[0].value = game::HUMAN_SYM.to_string();
                            }
                            let ai = game::ai_move(game::AI, game::HUMAN, b.board);
                            game::player_move(game::AI, ai, &mut b.board);

                            mark_ai_move(ai, &button1_query, &button2_query, &button3_query, &button4_query, 
                                &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);

                            if game::is_draw(b.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if game::is_game_over(game::AI, b.board) {
                                send_lost(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                        }
                    }
                }
                else if marker2.is_some() {
                    for mut b in &mut board_query {
                        if game::player_move(b.turn, 2, &mut b.board) {
                            if game::is_draw(b.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if game::is_game_over(b.turn, b.board) {
                                send_won(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            {
                                let mut text = text_query.get_mut(children[0]).unwrap(); // put in its own scope so can reborrow
                                text.sections[0].value = game::HUMAN_SYM.to_string();
                            }
                            let ai = game::ai_move(game::AI, game::HUMAN, b.board);
                            game::player_move(game::AI, ai, &mut b.board);

                            mark_ai_move(ai, &button1_query, &button2_query, &button3_query, &button4_query, 
                                &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);

                            if game::is_draw(b.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if game::is_game_over(game::AI, b.board) {
                                send_lost(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                        }
                    }
                }
                else if marker3.is_some() {
                    for mut b in &mut board_query {
                        if game::player_move(b.turn, 3, &mut b.board) {
                            if game::is_draw(b.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if game::is_game_over(b.turn, b.board) {
                                send_won(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            {
                                let mut text = text_query.get_mut(children[0]).unwrap(); // put in its own scope so can reborrow
                                text.sections[0].value = game::HUMAN_SYM.to_string();
                            }
                            let ai = game::ai_move(game::AI, game::HUMAN, b.board);
                            game::player_move(game::AI, ai, &mut b.board);

                            mark_ai_move(ai, &button1_query, &button2_query, &button3_query, &button4_query, 
                                &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);

                            if game::is_draw(b.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if game::is_game_over(game::AI, b.board) {
                                send_lost(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                        }
                    }
                }
                else if marker4.is_some() {
                    for mut b in &mut board_query {
                        if game::player_move(b.turn, 4, &mut b.board) {
                            if game::is_draw(b.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if game::is_game_over(b.turn, b.board) {
                                send_won(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            {
                                let mut text = text_query.get_mut(children[0]).unwrap(); // put in its own scope so can reborrow
                                text.sections[0].value = game::HUMAN_SYM.to_string();
                            }
                            let ai = game::ai_move(game::AI, game::HUMAN, b.board);
                            game::player_move(game::AI, ai, &mut b.board);

                            mark_ai_move(ai, &button1_query, &button2_query, &button3_query, &button4_query, 
                                &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);

                            if game::is_draw(b.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if game::is_game_over(game::AI, b.board) {
                                send_lost(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                        }
                    }
                }
                else if marker5.is_some() {
                    for mut b in &mut board_query {
                        if game::player_move(b.turn, 5, &mut b.board) {
                            if game::is_draw(b.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if game::is_game_over(b.turn, b.board) {
                                send_won(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            {
                                let mut text = text_query.get_mut(children[0]).unwrap(); // put in its own scope so can reborrow
                                text.sections[0].value = game::HUMAN_SYM.to_string();
                            }
                            let ai = game::ai_move(game::AI, game::HUMAN, b.board);
                            game::player_move(game::AI, ai, &mut b.board);

                            mark_ai_move(ai, &button1_query, &button2_query, &button3_query, &button4_query, 
                                &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);

                            if game::is_draw(b.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if game::is_game_over(game::AI, b.board) {
                                send_lost(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                        }
                    }
                }
                else if marker6.is_some() {
                    for mut b in &mut board_query {
                        if game::player_move(b.turn, 6, &mut b.board) {
                            if game::is_draw(b.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if game::is_game_over(b.turn, b.board) {
                                send_won(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            {
                                let mut text = text_query.get_mut(children[0]).unwrap(); // put in its own scope so can reborrow
                                text.sections[0].value = game::HUMAN_SYM.to_string();
                            }
                            let ai = game::ai_move(game::AI, game::HUMAN, b.board);
                            game::player_move(game::AI, ai, &mut b.board);

                            mark_ai_move(ai, &button1_query, &button2_query, &button3_query, &button4_query, 
                                &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);

                            if game::is_draw(b.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if game::is_game_over(game::AI, b.board) {
                                send_lost(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                        }
                    }
                }
                else if marker7.is_some() {
                    for mut b in &mut board_query {
                        if game::player_move(b.turn, 7, &mut b.board) {
                            if game::is_draw(b.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if game::is_game_over(b.turn, b.board) {
                                send_won(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            {
                                let mut text = text_query.get_mut(children[0]).unwrap(); // put in its own scope so can reborrow
                                text.sections[0].value = game::HUMAN_SYM.to_string();
                            }
                            let ai = game::ai_move(game::AI, game::HUMAN, b.board);
                            game::player_move(game::AI, ai, &mut b.board);

                            mark_ai_move(ai, &button1_query, &button2_query, &button3_query, &button4_query, 
                                &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);

                            if game::is_draw(b.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if game::is_game_over(game::AI, b.board) {
                                send_lost(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                        }
                    }
                }
                else if marker8.is_some() {
                    for mut b in &mut board_query {
                        if game::player_move(b.turn, 8, &mut b.board) {
                            if game::is_draw(b.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if game::is_game_over(b.turn, b.board) {
                                send_won(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            {
                                let mut text = text_query.get_mut(children[0]).unwrap(); // put in its own scope so can reborrow
                                text.sections[0].value = game::HUMAN_SYM.to_string();
                            }
                            let ai = game::ai_move(game::AI, game::HUMAN, b.board);
                            game::player_move(game::AI, ai, &mut b.board);

                            mark_ai_move(ai, &button1_query, &button2_query, &button3_query, &button4_query, 
                                &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);

                            if game::is_draw(b.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if game::is_game_over(game::AI, b.board) {
                                send_lost(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                        }
                    }
                }
                else if marker9.is_some() {
                    for mut b in &mut board_query {
                        if game::player_move(b.turn, 9, &mut b.board) {
                            if game::is_draw(b.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if game::is_game_over(b.turn, b.board) {
                                send_won(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            {
                                let mut text = text_query.get_mut(children[0]).unwrap(); // put in its own scope so can reborrow
                                text.sections[0].value = game::HUMAN_SYM.to_string();
                            }
                            let ai = game::ai_move(game::AI, game::HUMAN, b.board);
                            game::player_move(game::AI, ai, &mut b.board);

                            mark_ai_move(ai, &button1_query, &button2_query, &button3_query, &button4_query, 
                                &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);

                            if game::is_draw(b.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if game::is_game_over(game::AI, b.board) {
                                send_lost(&mut text_popup);
                                game::clear_board(&mut b.board);
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