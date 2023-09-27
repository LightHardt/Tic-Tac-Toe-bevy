use bevy::prelude::Component;

type Board = [[i8;3]; 3];
pub const HUMAN: i8 = 1;
pub const AI: i8 = 2;
pub const HUMAN_SYM: &str = "X";
pub const AI_SYM: &str = "O";

/**
 * Will hold key functions for Tic Tac Toe game
 */
#[derive(Clone, Copy, Component)]
pub struct Game {
    pub board: Board,
    pub turn: i8,
}
impl Default for Game {
    fn default() -> Game {
        Game { board:  [[0;3];3], turn: 1}
    }
}
/**
 * Checks to see if a given move is valid and if so makes it
 */
pub fn player_move(player: i8, position: i8, board: &mut Board) -> bool{
    match position {
        1 => if board[0][0] != 0 {return false;} else {board[0][0] = player; return true;},
        2 => if board[0][1] != 0 {return false;} else {board[0][1] = player; return true;},
        3 => if board[0][2] != 0 {return false;} else {board[0][2] = player; return true;},
        4 => if board[1][0] != 0 {return false;} else {board[1][0] = player; return true;},
        5 => if board[1][1] != 0 {return false;} else {board[1][1] = player; return true;},
        6 => if board[1][2] != 0 {return false;} else {board[1][2] = player; return true;},
        7 => if board[2][0] != 0 {return false;} else {board[2][0] = player; return true;},
        8 => if board[2][1] != 0 {return false;} else {board[2][1] = player; return true;},
        9 => if board[2][2] != 0 {return false;} else {board[2][2] = player; return true;},
        _ => false,
    }
}
pub fn remove_move(position: i8,board: &mut Board) {
    match position {
        1 => board[0][0] = 0,
        2 => board[0][1] = 0,
        3 => board[0][2] = 0,
        4 => board[1][0] = 0,
        5 => board[1][1] = 0,
        6 => board[1][2] = 0,
        7 => board[2][0] = 0,
        8 => board[2][1] = 0,
        9 => board[2][2] = 0,
        _ => panic!("Error in remove move function"),
    }
}
/**
 * Function checks if win condition has been met for
 * given player
 */
pub fn is_game_over(player: i8, board: Board) -> bool {
    let n = 3;
    let mut in_a_row: i8 = 0;
    // Diagnol check
    for x in 0..n {
        if board[x][x] == player {
            in_a_row+=1;
        }
    }
    if in_a_row == 3 { return true; }
    in_a_row = 0;
    // Second diagnol check
    let mut reverse = n;
    for x in 0..n {
        reverse-=1; 
        if board[x][reverse] == player {
            in_a_row+=1;
        }    
    }
    if in_a_row == 3 { return true; }
    in_a_row = 0;
    // Rows
    for x in 0..n {
        for y in 0..n {
            if board[x][y] == player {
                in_a_row+=1;
            }
        }
        if in_a_row == 3 { return true; }
        in_a_row = 0;
    }
    // Columns
    for x in 0..n {
        for y in 0..n {
            if board[y][x] == player {
                in_a_row+=1;
            }
        }
        if in_a_row == 3 { return true; }
        in_a_row = 0;
    }
    false
}

pub fn is_draw(board: Board) -> bool {
    let mut filled_slots = 0;
    for x in 0..3 {
        for y in 0..3 {
            if board[x][y] != 0 {
                filled_slots+=1;
            }
        }
    }
    if filled_slots == 9 {
        return true;
    }
    false
}
/**
 * The evaluate function will determine how good a move is based on
 * how many possible ways the given player can win in the current
 * board state
 */
fn evaluate(board: Board, player: i8) -> i32 {
    if is_game_over(player, board) {
        return i32::max_value();
    }
    if is_draw(board) {
        return 0;
    }
    let mut ways_to_win = 0;
    let mut ctr = 0;
    let n = 3;
            // Diagnol check
            for x in 0..n {
                if board[x][x] == player || board[x][x] == 0 {
                    ctr+=1;
                }
            }
    
            if ctr == 3 { ways_to_win+=1; }
            ctr = 0;
    
            // Second diagnol check
            let mut reverse = n;
            for x in 0..n {
                reverse-=1; 
                if board[x][reverse] == player || board[x][reverse] == 0 {
                    ctr+=1;
                }    
            }
    
            if ctr == 3 { ways_to_win+=1; }
            ctr = 0;
    
            // Rows
            for x in 0..n {
                for y in 0..n {
                    if board[x][y] == player || board[x][y] == 0 {
                        ctr+=1;
                    }
                }
                if ctr == 3 { ways_to_win+=1; }
                ctr = 0;
            }
    
            // Columns
            for x in 0..n {
                for y in 0..n {
                    if board[y][x] == player || board[y][x] == 0 {
                        ctr+=1;
                    }
                }
                if ctr == 3 { ways_to_win+=1; }
                ctr = 0;
            }
            ways_to_win
}
/**
 * Minimax algorithm so the ai can make intelligent moves
 */
fn minimax(ai: i8, human: i8,  depth: i8, mut board: Board, maximize: bool) -> i32{
    if depth == 0 || is_draw(board) || is_game_over(ai, board) || is_game_over(human,board) {
        let ai_eval: i32 = evaluate(board, ai);
        let human_eval: i32 = evaluate(board, human);
        return ai_eval - human_eval;
    }
    if maximize {
        let mut max_eval: i32 = i32::min_value();
        for position in 1..10 {
            
            if player_move(ai, position,&mut board) {
                let eval = minimax(ai, human, depth - 1, board, false);
                remove_move(position, &mut board);
                max_eval = i32::max(max_eval, eval);
            }
            
        }
        return max_eval;
    }
    else {
        let mut min_eval = i32::max_value();
        for position in 1..10 {
            if player_move(human, position, &mut board) {
                let eval = minimax(ai, human, depth - 1, board, true);
                remove_move(position, &mut board);
                min_eval = i32::min(min_eval, eval);
            }
        }
        return min_eval;
    }
}
/**
 * Function makes a random move for AI based on 
 * open slots randomly
 */
pub fn ai_move(ai: i8, human: i8, board: Board) -> i8 {
    let mut best = i32::min_value();
    let mut ai_position = -1;
    let depth = 10; // At four you could get it to lose easily this makes it harder
    let mut board = board.clone();
    for position in 1..10 {
        if player_move(ai, position, &mut board) {
            let eval = minimax(ai, human, depth, board, false);
            remove_move(position, &mut board);
            if eval >= best {
                best = eval;
                ai_position = position;
            }
        }
    }
    ai_position
}

pub fn clear_board(board: &mut Board) {
    for i in 0..3 {
        for j in 0..3 {
            board[i][j] = 0;
        }
    }
}