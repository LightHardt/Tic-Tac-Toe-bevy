use bevy::prelude::*;
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
) {
    for (interaction, mut color, mut border_color, children, 
        (marker1, marker2,marker3, marker4,marker5, 
            marker6,marker7, marker8, marker9)) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if marker1.is_some() {
                    for mut b in &mut board_query {
                        if game::player_move(b.turn, 1, &mut b.board) {
                            if game::is_draw(b.board) { // Maybe turn this into a system and schedule it before is_game_over maybe
                                println!("Game is Draw");
                            }
                            if game::is_game_over(b.turn, b.board) {
                                println!("Game Over {} wins", b.turn);
                            }
                            {
                                let mut text = text_query.get_mut(children[0]).unwrap(); // put in its own scope so can reborrow
                                text.sections[0].value = game::HUMAN_SYM.to_string();
                            }
                            let ai = game::ai_move(game::AI, game::HUMAN, b.board);
                            game::player_move(game::AI, ai, &mut b.board);

                            match ai {
                                1 => {
                                    for button in &button1_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                2 => {
                                    for button in &button2_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                3 => {
                                    for button in &button3_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                4 => {
                                    for button in &button4_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                5 => {
                                    for button in &button5_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                6 => {
                                    for button in &button6_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                7 => {
                                    for button in &button7_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                8 => {
                                    for button in &button8_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                9 => {
                                    for button in &button9_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                _ => panic!("AI move error")
                            }
                        }
                    }
                }
                else if marker2.is_some() {
                    for mut b in &mut board_query {
                        if game::player_move(b.turn, 2, &mut b.board) {
                            if game::is_draw(b.board) { // Maybe turn this into a system and schedule it before is_game_over maybe
                                println!("Game is Draw");
                            }
                            if game::is_game_over(b.turn, b.board) {
                                println!("Game Over {} wins", b.turn);
                            }
                            {
                                let mut text = text_query.get_mut(children[0]).unwrap(); // put in its own scope so can reborrow
                                text.sections[0].value = game::HUMAN_SYM.to_string();
                            }
                            let ai = game::ai_move(game::AI, game::HUMAN, b.board);
                            game::player_move(game::AI, ai, &mut b.board);

                            match ai {
                                1 => {
                                    for button in &button1_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                2 => {
                                    for button in &button2_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                3 => {
                                    for button in &button3_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                4 => {
                                    for button in &button4_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                5 => {
                                    for button in &button5_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                6 => {
                                    for button in &button6_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                7 => {
                                    for button in &button7_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                8 => {
                                    for button in &button8_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                9 => {
                                    for button in &button9_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                _ => panic!("AI move error")
                            }
                        }
                    }
                }
                else if marker3.is_some() {
                    for mut b in &mut board_query {
                        if game::player_move(b.turn, 3, &mut b.board) {
                            if game::is_draw(b.board) { // Maybe turn this into a system and schedule it before is_game_over maybe
                                println!("Game is Draw");
                            }
                            if game::is_game_over(b.turn, b.board) {
                                println!("Game Over {} wins", b.turn);
                            }
                            {
                                let mut text = text_query.get_mut(children[0]).unwrap(); // put in its own scope so can reborrow
                                text.sections[0].value = game::HUMAN_SYM.to_string();
                            }
                            let ai = game::ai_move(game::AI, game::HUMAN, b.board);
                            game::player_move(game::AI, ai, &mut b.board);

                            match ai {
                                1 => {
                                    for button in &button1_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                2 => {
                                    for button in &button2_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                3 => {
                                    for button in &button3_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                4 => {
                                    for button in &button4_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                5 => {
                                    for button in &button5_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                6 => {
                                    for button in &button6_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                7 => {
                                    for button in &button7_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                8 => {
                                    for button in &button8_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                9 => {
                                    for button in &button9_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                _ => panic!("AI move error")
                            }
                        }
                    }
                }
                else if marker4.is_some() {
                    for mut b in &mut board_query {
                        if game::player_move(b.turn, 4, &mut b.board) {
                            if game::is_draw(b.board) { // Maybe turn this into a system and schedule it before is_game_over maybe
                                println!("Game is Draw");
                            }
                            if game::is_game_over(b.turn, b.board) {
                                println!("Game Over {} wins", b.turn);
                            }
                            {
                                let mut text = text_query.get_mut(children[0]).unwrap(); // put in its own scope so can reborrow
                                text.sections[0].value = game::HUMAN_SYM.to_string();
                            }
                            let ai = game::ai_move(game::AI, game::HUMAN, b.board);
                            game::player_move(game::AI, ai, &mut b.board);

                            match ai {
                                1 => {
                                    for button in &button1_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                2 => {
                                    for button in &button2_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                3 => {
                                    for button in &button3_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                4 => {
                                    for button in &button4_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                5 => {
                                    for button in &button5_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                6 => {
                                    for button in &button6_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                7 => {
                                    for button in &button7_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                8 => {
                                    for button in &button8_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                9 => {
                                    for button in &button9_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                _ => panic!("AI move error")
                            }
                        }
                    }
                }
                else if marker5.is_some() {
                    for mut b in &mut board_query {
                        if game::player_move(b.turn, 5, &mut b.board) {
                            if game::is_draw(b.board) { // Maybe turn this into a system and schedule it before is_game_over maybe
                                println!("Game is Draw");
                            }
                            if game::is_game_over(b.turn, b.board) {
                                println!("Game Over {} wins", b.turn);
                            }
                            {
                                let mut text = text_query.get_mut(children[0]).unwrap(); // put in its own scope so can reborrow
                                text.sections[0].value = game::HUMAN_SYM.to_string();
                            }
                            let ai = game::ai_move(game::AI, game::HUMAN, b.board);
                            game::player_move(game::AI, ai, &mut b.board);

                            match ai {
                                1 => {
                                    for button in &button1_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                2 => {
                                    for button in &button2_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                3 => {
                                    for button in &button3_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                4 => {
                                    for button in &button4_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                5 => {
                                    for button in &button5_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                6 => {
                                    for button in &button6_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                7 => {
                                    for button in &button7_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                8 => {
                                    for button in &button8_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                9 => {
                                    for button in &button9_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                _ => panic!("AI move error")
                            }
                        }
                    }
                }
                else if marker6.is_some() {
                    for mut b in &mut board_query {
                        if game::player_move(b.turn, 6, &mut b.board) {
                            if game::is_draw(b.board) { // Maybe turn this into a system and schedule it before is_game_over maybe
                                println!("Game is Draw");
                            }
                            if game::is_game_over(b.turn, b.board) {
                                println!("Game Over {} wins", b.turn);
                            }
                            {
                                let mut text = text_query.get_mut(children[0]).unwrap(); // put in its own scope so can reborrow
                                text.sections[0].value = game::HUMAN_SYM.to_string();
                            }
                            let ai = game::ai_move(game::AI, game::HUMAN, b.board);
                            game::player_move(game::AI, ai, &mut b.board);

                            match ai {
                                1 => {
                                    for button in &button1_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                2 => {
                                    for button in &button2_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                3 => {
                                    for button in &button3_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                4 => {
                                    for button in &button4_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                5 => {
                                    for button in &button5_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                6 => {
                                    for button in &button6_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                7 => {
                                    for button in &button7_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                8 => {
                                    for button in &button8_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                9 => {
                                    for button in &button9_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                _ => panic!("AI move error")
                            }
                        }
                    }
                }
                else if marker7.is_some() {
                    for mut b in &mut board_query {
                        if game::player_move(b.turn, 7, &mut b.board) {
                            if game::is_draw(b.board) { // Maybe turn this into a system and schedule it before is_game_over maybe
                                println!("Game is Draw");
                            }
                            if game::is_game_over(b.turn, b.board) {
                                println!("Game Over {} wins", b.turn);
                            }
                            {
                                let mut text = text_query.get_mut(children[0]).unwrap(); // put in its own scope so can reborrow
                                text.sections[0].value = game::HUMAN_SYM.to_string();
                            }
                            let ai = game::ai_move(game::AI, game::HUMAN, b.board);
                            game::player_move(game::AI, ai, &mut b.board);

                            match ai {
                                1 => {
                                    for button in &button1_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                2 => {
                                    for button in &button2_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                3 => {
                                    for button in &button3_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                4 => {
                                    for button in &button4_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                5 => {
                                    for button in &button5_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                6 => {
                                    for button in &button6_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                7 => {
                                    for button in &button7_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                8 => {
                                    for button in &button8_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                9 => {
                                    for button in &button9_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                _ => panic!("AI move error")
                            }
                        }
                    }
                }
                else if marker8.is_some() {
                    for mut b in &mut board_query {
                        if game::player_move(b.turn, 8, &mut b.board) {
                            if game::is_draw(b.board) { // Maybe turn this into a system and schedule it before is_game_over maybe
                                println!("Game is Draw");
                            }
                            if game::is_game_over(b.turn, b.board) {
                                println!("Game Over {} wins", b.turn);
                            }
                            {
                                let mut text = text_query.get_mut(children[0]).unwrap(); // put in its own scope so can reborrow
                                text.sections[0].value = game::HUMAN_SYM.to_string();
                            }
                            let ai = game::ai_move(game::AI, game::HUMAN, b.board);
                            game::player_move(game::AI, ai, &mut b.board);

                            match ai {
                                1 => {
                                    for button in &button1_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                2 => {
                                    for button in &button2_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                3 => {
                                    for button in &button3_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                4 => {
                                    for button in &button4_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                5 => {
                                    for button in &button5_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                6 => {
                                    for button in &button6_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                7 => {
                                    for button in &button7_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                8 => {
                                    for button in &button8_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                9 => {
                                    for button in &button9_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                _ => panic!("AI move error")
                            }
                        }
                    }
                }
                else if marker9.is_some() {
                    for mut b in &mut board_query {
                        if game::player_move(b.turn, 9, &mut b.board) {
                            if game::is_draw(b.board) { // Maybe turn this into a system and schedule it before is_game_over maybe
                                println!("Game is Draw");
                            }
                            if game::is_game_over(b.turn, b.board) {
                                println!("Game Over {} wins", b.turn);
                            }
                            {
                                let mut text = text_query.get_mut(children[0]).unwrap(); // put in its own scope so can reborrow
                                text.sections[0].value = game::HUMAN_SYM.to_string();
                            }
                            let ai = game::ai_move(game::AI, game::HUMAN, b.board);
                            game::player_move(game::AI, ai, &mut b.board);

                            match ai {
                                1 => {
                                    for button in &button1_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                2 => {
                                    for button in &button2_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                3 => {
                                    for button in &button3_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                4 => {
                                    for button in &button4_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                5 => {
                                    for button in &button5_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                6 => {
                                    for button in &button6_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                7 => {
                                    for button in &button7_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                8 => {
                                    for button in &button8_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                9 => {
                                    for button in &button9_query {
                                        let mut text = text_query.get_mut(button[0]).unwrap();
                                        text.sections[0].value = game::AI_SYM.to_string();
                                    }
                                },
                                _ => panic!("AI move error")
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

// pub fn marker1_test(
//     button1_query: Query<&Children,With<Marker1>>,
//     button2_query: Query<&Children,With<Marker2>>,
//     mut text_query: Query<&mut Text>
// ) {
//     for button in &button1_query {
//         let mut text = text_query.get_mut(button[0]).unwrap();
//         text.sections[0].value = "I work".to_string();
//     }
// }