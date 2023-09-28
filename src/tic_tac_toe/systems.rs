use bevy::prelude::*;
use bevy_text_popup::{TextPopupEvent, TextPopupTimeout, TextPopupButton};

use super::components::*;
use super::game;
use super::game::*;

// Button Colors
pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn(Camera2dBundle::default());
    // spawn buttons for game 3x3 grid
    commands
        .spawn((NodeBundle {
            style: Style {
                display: Display::Grid,
                justify_content: JustifyContent::Center,
                align_self: AlignSelf::Center,
                margin: UiRect::left(Val::Vw(20.0)),
                grid_template_columns: RepeatedGridTrack::flex(3, 1.0),
                // Set the grid to have 4 rows all with sizes minmax(0, 1fr)
                // This creates 4 exactly evenly sized rows
                grid_template_rows: RepeatedGridTrack::flex(3, 1.0),
                aspect_ratio: Some(1.0),
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(1.0),
                column_gap: Val::Px(1.0),
                ..default()
            },
            ..default()
        },Game::default()))
        .with_children(|parent| {
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(110.0),
                        height: Val::Px(110.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },Marker1))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(110.0),
                        height: Val::Px(110.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },Marker2))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(110.0),
                        height: Val::Px(110.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },Marker3))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(110.0),
                        height: Val::Px(110.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },Marker4))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(110.0),
                        height: Val::Px(110.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },Marker5))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(110.0),
                        height: Val::Px(110.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },Marker6))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(110.0),
                        height: Val::Px(110.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },Marker7))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(110.0),
                        height: Val::Px(110.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },Marker8))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(110.0),
                        height: Val::Px(110.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },Marker9))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });

    // spawn button for ai move option
    commands
        .spawn(NodeBundle {
            style: Style { 
                display: Display::Flex,
                width: Val::Px(75.0),
                height: Val::Px(55.0),
                align_self: AlignSelf::Center,
                margin: UiRect::top(Val::Vh(70.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        width: Val::Px(75.0),
                        height: Val::Px(55.0),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: Color::PURPLE.into(),
                    ..default()
                }, AiMove))
                .with_children(|parent| {
                    parent
                        .spawn(TextBundle::from_section(
                            "Ai Move\nFirst",
                            TextStyle {
                                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                                font_size: 15.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            }
                        ));
                });
        });               
            
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
    mut board_query: Query<&mut Game>,
) {
    for (interaction, mut border_color, ai_button_check) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if ai_button_check.is_some() {
                    for mut board in &mut board_query {
                        let mut first_move_check = true;
                        for i in board.board {
                            for j in i {
                                if j != 0  { first_move_check = false; }
                            }
                        }
                        if !first_move_check {
                            break;
                        }
                        let ai = ai_move(AI, HUMAN, board.board);
                        player_move(AI, ai, &mut board.board);
                        mark_ai_move(ai, &button1_query, &button2_query, &button3_query, &button4_query, 
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
    mut board_query: Query<&mut Game>,
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
                        if player_move(b.turn, 1, &mut b.board) {
                            if is_draw(b.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if is_game_over(b.turn, b.board) {
                                send_won(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            {
                                let mut text = text_query.get_mut(children[0]).unwrap(); // put in its own scope so can reborrow
                                text.sections[0].value = HUMAN_SYM.to_string();
                            }
                            let ai = ai_move(AI, HUMAN, b.board);
                            player_move(AI, ai, &mut b.board);

                            mark_ai_move(ai, &button1_query, &button2_query, &button3_query, &button4_query, 
                                &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);

                            if is_draw(b.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if is_game_over(AI, b.board) {
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
                        if player_move(b.turn, 2, &mut b.board) {
                            if is_draw(b.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if is_game_over(b.turn, b.board) {
                                send_won(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            {
                                let mut text = text_query.get_mut(children[0]).unwrap(); // put in its own scope so can reborrow
                                text.sections[0].value = HUMAN_SYM.to_string();
                            }
                            let ai = ai_move(AI, HUMAN, b.board);
                            player_move(AI, ai, &mut b.board);

                            mark_ai_move(ai, &button1_query, &button2_query, &button3_query, &button4_query, 
                                &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);

                            if is_draw(b.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if is_game_over(AI, b.board) {
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
                        if player_move(b.turn, 3, &mut b.board) {
                            if is_draw(b.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if is_game_over(b.turn, b.board) {
                                send_won(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            {
                                let mut text = text_query.get_mut(children[0]).unwrap(); // put in its own scope so can reborrow
                                text.sections[0].value = HUMAN_SYM.to_string();
                            }
                            let ai = ai_move(AI, HUMAN, b.board);
                            player_move(AI, ai, &mut b.board);

                            mark_ai_move(ai, &button1_query, &button2_query, &button3_query, &button4_query, 
                                &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);

                            if is_draw(b.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if is_game_over(AI, b.board) {
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
                        if player_move(b.turn, 4, &mut b.board) {
                            if is_draw(b.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if is_game_over(b.turn, b.board) {
                                send_won(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            {
                                let mut text = text_query.get_mut(children[0]).unwrap(); // put in its own scope so can reborrow
                                text.sections[0].value = HUMAN_SYM.to_string();
                            }
                            let ai = ai_move(AI, HUMAN, b.board);
                            player_move(AI, ai, &mut b.board);

                            mark_ai_move(ai, &button1_query, &button2_query, &button3_query, &button4_query, 
                                &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);

                            if is_draw(b.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if is_game_over(AI, b.board) {
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
                        if player_move(b.turn, 5, &mut b.board) {
                            if is_draw(b.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if is_game_over(b.turn, b.board) {
                                send_won(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            {
                                let mut text = text_query.get_mut(children[0]).unwrap(); // put in its own scope so can reborrow
                                text.sections[0].value = HUMAN_SYM.to_string();
                            }
                            let ai = ai_move(AI, HUMAN, b.board);
                            player_move(AI, ai, &mut b.board);

                            mark_ai_move(ai, &button1_query, &button2_query, &button3_query, &button4_query, 
                                &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);

                            if is_draw(b.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if is_game_over(AI, b.board) {
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
                        if player_move(b.turn, 6, &mut b.board) {
                            if is_draw(b.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if is_game_over(b.turn, b.board) {
                                send_won(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            {
                                let mut text = text_query.get_mut(children[0]).unwrap(); // put in its own scope so can reborrow
                                text.sections[0].value = HUMAN_SYM.to_string();
                            }
                            let ai = ai_move(AI, HUMAN, b.board);
                            player_move(AI, ai, &mut b.board);

                            mark_ai_move(ai, &button1_query, &button2_query, &button3_query, &button4_query, 
                                &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);

                            if is_draw(b.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if is_game_over(AI, b.board) {
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
                        if player_move(b.turn, 7, &mut b.board) {
                            if is_draw(b.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if is_game_over(b.turn, b.board) {
                                send_won(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            {
                                let mut text = text_query.get_mut(children[0]).unwrap(); // put in its own scope so can reborrow
                                text.sections[0].value = HUMAN_SYM.to_string();
                            }
                            let ai = ai_move(AI, HUMAN, b.board);
                            player_move(AI, ai, &mut b.board);

                            mark_ai_move(ai, &button1_query, &button2_query, &button3_query, &button4_query, 
                                &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);

                            if is_draw(b.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if is_game_over(AI, b.board) {
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
                        if player_move(b.turn, 8, &mut b.board) {
                            if is_draw(b.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if is_game_over(b.turn, b.board) {
                                send_won(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            {
                                let mut text = text_query.get_mut(children[0]).unwrap(); // put in its own scope so can reborrow
                                text.sections[0].value = HUMAN_SYM.to_string();
                            }
                            let ai = ai_move(AI, HUMAN, b.board);
                            player_move(AI, ai, &mut b.board);

                            mark_ai_move(ai, &button1_query, &button2_query, &button3_query, &button4_query, 
                                &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);

                            if is_draw(b.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if is_game_over(AI, b.board) {
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
                        if player_move(b.turn, 9, &mut b.board) {
                            if is_draw(b.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if is_game_over(b.turn, b.board) {
                                send_won(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            {
                                let mut text = text_query.get_mut(children[0]).unwrap(); // put in its own scope so can reborrow
                                text.sections[0].value = HUMAN_SYM.to_string();
                            }
                            let ai = ai_move(AI, HUMAN, b.board);
                            player_move(AI, ai, &mut b.board);

                            mark_ai_move(ai, &button1_query, &button2_query, &button3_query, &button4_query, 
                                &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);

                            if is_draw(b.board) {
                                send_draw(&mut text_popup);
                                game::clear_board(&mut b.board);
                                clear_board(&button1_query, &button2_query, &button3_query, &button4_query, 
                                    &button5_query, &button6_query, &button7_query, &button8_query, &button9_query, &mut text_query);
                                break;
                            }
                            if is_game_over(AI, b.board) {
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
