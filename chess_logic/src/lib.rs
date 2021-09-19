use std::{collections::HashMap, convert::TryInto};

#[non_exhaustive]
struct TYPES;

impl TYPES {
    pub const NONE: u8 = 0;
    pub const PAWN: u8 = 1;
    pub const KNIGHT: u8 = 2;
    pub const BISHOP: u8 = 4;
    pub const ROOK: u8 = 8;
    pub const QUEEN: u8 = 16;
    pub const KING: u8 = 32;
}

pub fn is_black_king(piece: u8) -> bool {
    if (piece & TYPES::KING > 0) & (piece & COLORS::BLACK > 0) {
        return true;
    } else {
        false
    }
}
pub fn is_black_queen(piece: u8) -> bool {
    if (piece & TYPES::QUEEN > 0) & (piece & COLORS::BLACK > 0) {
        return true;
    } else {
        false
    }
}
pub fn is_black_rook(piece: u8) -> bool {
    if (piece & TYPES::ROOK > 0) & (piece & COLORS::BLACK > 0) {
        return true;
    } else {
        false
    }
}
pub fn is_black_bishop(piece: u8) -> bool {
    if (piece & TYPES::BISHOP > 0) & (piece & COLORS::BLACK > 0) {
        return true;
    } else {
        false
    }
}
pub fn is_black_knight(piece: u8) -> bool {
    if (piece & TYPES::KNIGHT > 0) & (piece & COLORS::BLACK > 0) {
        return true;
    } else {
        false
    }
}
pub fn is_black_pawn(piece: u8) -> bool {
    if (piece & TYPES::PAWN > 0) & (piece & COLORS::BLACK > 0) {
        return true;
    } else {
        false
    }
}
pub fn is_white_king(piece: u8) -> bool {
    if (piece & TYPES::KING > 0) & (piece & COLORS::WHITE > 0) {
        return true;
    } else {
        false
    }
}
pub fn is_white_queen(piece: u8) -> bool {
    if (piece & TYPES::QUEEN > 0) & (piece & COLORS::WHITE > 0) {
        return true;
    } else {
        false
    }
}
pub fn is_white_rook(piece: u8) -> bool {
    if (piece & TYPES::ROOK > 0) & (piece & COLORS::WHITE > 0) {
        return true;
    } else {
        false
    }
}
pub fn is_white_bishop(piece: u8) -> bool {
    if (piece & TYPES::BISHOP > 0) & (piece & COLORS::WHITE > 0) {
        return true;
    } else {
        false
    }
}
pub fn is_white_knight(piece: u8) -> bool {
    if (piece & TYPES::KNIGHT > 0) & (piece & COLORS::WHITE > 0) {
        return true;
    } else {
        false
    }
}
pub fn is_white_pawn(piece: u8) -> bool {
    if (piece & TYPES::PAWN > 0) & (piece & COLORS::WHITE > 0) {
        return true;
    } else {
        false
    }
}

#[non_exhaustive]
struct COLORS;

impl COLORS {
    pub const WHITE: u8 = 64;
    pub const BLACK: u8 = 128;
}

pub struct GAME {
    board: [u8; 64],
    turn: u8,
    moves: Vec<[u8; 2]>,
    check: bool,
    draw: bool,
    check_mate: bool,
}

impl GAME {
     fn generate_board_array() -> [u8; 64] {
        [0u8; 64]
    }

    pub fn get_board(&self) -> [u8; 64] {
        self.board
    }

    pub fn get_played_moves(&self) -> &Vec<[u8; 2]> {
        &self.moves
    }

    pub fn is_check(&self) -> bool {
        self.check
    }

    pub fn is_check_mate(&self) -> bool {
        self.check_mate
    }

    pub fn is_draw(&self) -> bool {
        self.draw
    }

    pub fn is_whites_turn(&self) -> bool {
        if self.turn & COLORS::WHITE > 0 {
            return true
        } else {
            false
        }
    }

    pub fn get_game_status(&self) -> (bool, bool, bool, bool) {
        (self.is_whites_turn(), self.is_check(), self.is_draw(), self.is_check_mate())
    }
}

pub fn algebraic_notation_to_memory_location(algebraic_notation: &str) -> usize {
    let mut alphabet_to_index = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    let mut rank: usize = 0;
    let mut file: usize = 0;
    for character in algebraic_notation.chars() {
        if character.is_alphabetic() {
            let lowercase_char = character.to_lowercase().collect::<Vec<_>>()[0];
            file = (alphabet_to_index.iter().position(|&r| r == lowercase_char).unwrap()) as usize;
        }
        if character.is_digit(10) {
            let int_rank = character.to_digit(10).unwrap() as i32;
            rank = (8 - int_rank) as usize;
        }
    }
    (rank * 8 + file) as usize
}

pub fn move_piece_from_to(from_tile: &str, to_tile: &str, game: &mut GAME) -> bool {
    let from_tile = algebraic_notation_to_memory_location(from_tile);
    let to_tile = algebraic_notation_to_memory_location(to_tile);
    let piece_to_move = game.board[from_tile];

    let mut if_valid_move = true;
    
    if ((game.turn & COLORS::WHITE) > 0) & ((piece_to_move & COLORS::WHITE) > 0) {
        if_valid_move = true;
    } else if ((game.turn & COLORS::BLACK) > 0) & ((piece_to_move & COLORS::BLACK) > 0) {
        if_valid_move = true;
    } else {
        if_valid_move = false;
    }

    // check if move is legal

    if if_valid_move {
        game.board[from_tile] = TYPES::NONE;
        game.board[to_tile] = piece_to_move;

        if game.turn == COLORS::WHITE {
            game.turn = COLORS::BLACK;
        } else {
            game.turn = COLORS::WHITE;
        }
    }
    
    if_valid_move
}

pub fn available_moves(tile: &str) -> [bool; 64] {

    [true; 64]
}

pub fn init_game() -> GAME {
    let mut game = GAME {
        board: GAME::generate_board_array(),
        turn: COLORS::WHITE,
        moves: Vec::new(),
        check: false,
        draw: false,
        check_mate: false,
    };
    let mut piece_type_from_symbol = HashMap::new();

    piece_type_from_symbol.insert('k', TYPES::KING);
    piece_type_from_symbol.insert('p', TYPES::PAWN);
    piece_type_from_symbol.insert('n', TYPES::KNIGHT);
    piece_type_from_symbol.insert('b', TYPES::BISHOP);
    piece_type_from_symbol.insert('r', TYPES::ROOK);
    piece_type_from_symbol.insert('q', TYPES::QUEEN);

    game.board = load_position_from_fen(STARTINGFEN, game.board, &mut piece_type_from_symbol);

    game
}

const STARTINGFEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";


/// # Testing FEN algorithm
///```
/// use chess_logic::*;
/// use std::{collections::HashMap, convert::TryInto};
/// let mut piece_type_from_symbol = HashMap::new();
/// piece_type_from_symbol.insert('k', 32);
/// piece_type_from_symbol.insert('p', 1);
/// piece_type_from_symbol.insert('n', 2);
/// piece_type_from_symbol.insert('b', 4);
/// piece_type_from_symbol.insert('r', 8);
/// piece_type_from_symbol.insert('q', 16);
/// let empty_board = [0u8; 64];
/// let STARTINGFEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
/// let board = chess_logic::load_position_from_fen(STARTINGFEN, empty_board, &mut piece_type_from_symbol);
/// let expected_output = [136, 130, 132, 144, 160, 132, 130, 136, 129, 129, 129, 129, 129, 129, 129, 129, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 65, 65, 65, 65, 65, 65, 65, 65, 72, 66, 68, 80, 96, 68, 66, 72];
/// assert_eq!(board, expected_output);
///```

pub fn load_position_from_fen(fen: &str, mut board: [u8; 64], piece_type_from_symbol: &mut HashMap<char, u8>) -> [u8; 64] {

    let mut fen_parts = fen.split_whitespace();

    let mut empty = false;
    let mut parts_index = 0;

    let mut positions: &str = "";
    let mut turn: &str = "";
    let mut casteling_ability: &str = "";
    let mut moved_on_to_by_un_passant: &str = "";
    let mut halfmove: &str = "";
    let mut fullmove: &str = "";

    while !empty {
        let part = fen_parts.next();
        if part == None {
            empty = true;
        } else {
            if parts_index == 0 {
                positions = part.unwrap();
            } else if parts_index == 1 {
                turn = part.unwrap();
            } else if parts_index == 2 {
                casteling_ability = part.unwrap();
            } else if parts_index == 3 {
                moved_on_to_by_un_passant = part.unwrap();
            } else if parts_index == 4 {
                halfmove = part.unwrap();
            } else {
                fullmove = part.unwrap();
            }
        }
        parts_index += 1;
    }

    let mut file = 0;
    let mut rank = 0;

    for character in positions.chars() {
        if character == '/'{
            file = 0;
            rank += 1;
        } else {
            if character.is_digit(10) {
                file += character.to_digit(10).unwrap();
            } else {
                let piece_color = if character.is_uppercase() { COLORS::WHITE } else { COLORS::BLACK};
                let lowercase_char = &character.to_lowercase().collect::<Vec<_>>()[0];
                let piece_type = piece_type_from_symbol.get(lowercase_char);
                let square: usize = (rank * 8 + file).try_into().unwrap();
                board[square] = piece_type.unwrap() + piece_color;
                file += 1;
            }
        }
    }
    board
}