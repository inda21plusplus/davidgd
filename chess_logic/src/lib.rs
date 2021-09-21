use std::{collections::HashMap, convert::TryInto};
use std::cmp;


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
    computed_distances: [[u8; 8]; 64],
    board: [u8; 64],
    turn: u8,
    moves: Vec<[u8; 2]>,
    check: bool,
    draw: bool,
    check_mate: bool,
}

impl GAME {
    fn tiles_to_the_edge() -> [[u8; 8]; 64] {
        let mut precomputed_distances = [[0u8; 8]; 64];
    
        for file in 0..8 {
            for rank in 0..8 {
    
                let tiles_north: u8 = rank;
                let tiles_south: u8 = 7 - rank;
                let tiles_west: u8 = file;
                let tiles_east: u8 = 7 - file;
    
                let tile_index: usize = (rank * 8 + file) as usize;
    
                precomputed_distances[tile_index] = [tiles_north as u8,
                                    tiles_south as u8,
                                    tiles_west as u8,
                                    tiles_east as u8,
                                    cmp::min(tiles_north, tiles_west),
                                    cmp::min(tiles_south, tiles_east),
                                    cmp::min(tiles_north, tiles_east),
                                    cmp::min(tiles_south, tiles_west)];
            }
        }
        precomputed_distances
    }
    
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

    let available_moves_for_piece = available_moves_for_piece(piece_to_move, from_tile, game); // XOR the two arrays to get array with all allowed moves
    if available_moves_for_piece[to_tile] & if_valid_move {
        if_valid_move = true;
    } else {
        if_valid_move = false;
    }

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

pub fn available_moves_for_piece(piece: u8, tile: usize, game: &mut GAME) -> [bool; 64] {
    let mut moves = [true; 64];
    if (piece & TYPES::KING) > 0 {
        moves = king_movement_from_tile(game.board, piece, tile, game.computed_distances);
    } else if (piece & TYPES::QUEEN) > 0 {
        moves = queen_movement_from_tile(game.board, piece, tile, game.computed_distances);
    } else if (piece & TYPES::ROOK) > 0 {
        moves = rook_movement_from_tile(game.board, piece, tile, game.computed_distances);
    } else if (piece & TYPES::BISHOP) > 0 {
        moves = bishop_movement_from_tile(game.board, piece, tile, game.computed_distances);
    } else if (piece & TYPES::KNIGHT) > 0 {
        moves = knight_movement_from_tile(game.board, piece, tile, game.computed_distances);
    } else if (piece & TYPES::PAWN) > 0 {
        // let moves = pawn_movement_from_tile(tile);
    }
    moves
}

fn king_movement_from_tile(board: [u8; 64], piece: u8, tile: usize, precomputed_distances: [[u8; 8]; 64]) -> [bool; 64] {
    let mut available_moves_board = [false; 64];
    let piece_color: u8;
    if piece & COLORS::WHITE > 0 {
        piece_color = COLORS::WHITE;
    } else {
        piece_color = COLORS::BLACK;
    }
    let offsets: [i8; 8] = [-8, 8, -1, 1, -9, 9, -7, 7];
    for (index, offset) in offsets.iter().enumerate() {
        let mut target_tile = tile as i8 + offset;
        let distances_to_edge = precomputed_distances[tile];
        if distances_to_edge[index] > 0 {
            if board[target_tile as usize] & piece_color > 0 {
                continue;
            } else {
                available_moves_board[target_tile as usize] = true;
            }
        }
    }
    draw_movement_board(available_moves_board);
    return available_moves_board
}

fn queen_movement_from_tile(board: [u8; 64], piece: u8, tile: usize, precomputed_distances: [[u8; 8]; 64]) -> [bool; 64] {
    let mut available_moves_board = [false; 64];
    let piece_color: u8;
    let enemy_piece_color: u8;
    if piece & COLORS::WHITE > 0 {
        piece_color = COLORS::WHITE;
        enemy_piece_color = COLORS::BLACK;
    } else {
        piece_color = COLORS::BLACK;
        enemy_piece_color = COLORS::WHITE;
    }
    let offsets: [i8; 8] = [-8, 8, -1, 1, -9, 9, -7, 7];
    for (index, offset) in offsets.iter().enumerate() {
        let mut target_tile: i8;
        let distances_to_edge = precomputed_distances[tile];
        for sliding_factor in 1..distances_to_edge[index] + 1 {
            target_tile = tile as i8 + offset * sliding_factor as i8;
            if board[target_tile as usize] & piece_color > 0 {
                break;
            } else if board[target_tile as usize] & enemy_piece_color > 0 {
                available_moves_board[target_tile as usize] = true;
                break;
            } else {
                available_moves_board[target_tile as usize] = true;
            }
        }
    }
    draw_movement_board(available_moves_board);
    return available_moves_board
}

fn rook_movement_from_tile(board: [u8; 64], piece: u8, tile: usize, precomputed_distances: [[u8; 8]; 64]) -> [bool; 64] {
    let mut available_moves_board = [false; 64];
    let piece_color: u8;
    let enemy_piece_color: u8;
    if piece & COLORS::WHITE > 0 {
        piece_color = COLORS::WHITE;
        enemy_piece_color = COLORS::BLACK;
    } else {
        piece_color = COLORS::BLACK;
        enemy_piece_color = COLORS::WHITE;
    }
    let offsets: [i8; 8] = [-8, 8, -1, 1, -9, 9, -7, 7];
    for (index, offset) in offsets[0..4].iter().enumerate() {
        let mut target_tile: i8;
        let distances_to_edge = precomputed_distances[tile];
        for sliding_factor in 1..distances_to_edge[index] + 1 {
            target_tile = tile as i8 + offset * sliding_factor as i8;
            if board[target_tile as usize] & piece_color > 0 {
                break;
            } else if board[target_tile as usize] & enemy_piece_color > 0 {
                available_moves_board[target_tile as usize] = true;
                break;
            } else {
                available_moves_board[target_tile as usize] = true;
            }
        }
    }
    draw_movement_board(available_moves_board);
    return available_moves_board
}

fn bishop_movement_from_tile(board: [u8; 64], piece: u8, tile: usize, precomputed_distances: [[u8; 8]; 64]) -> [bool; 64] {
    let mut available_moves_board = [false; 64];
    let piece_color: u8;
    let enemy_piece_color: u8;
    if piece & COLORS::WHITE > 0 {
        piece_color = COLORS::WHITE;
        enemy_piece_color = COLORS::BLACK;
    } else {
        piece_color = COLORS::BLACK;
        enemy_piece_color = COLORS::WHITE;
    }
    let offsets: [i8; 8] = [-8, 8, -1, 1, -9, 9, -7, 7];
    for (index, offset) in offsets[4..8].iter().enumerate() {
        let mut target_tile: i8;
        let distances_to_edge = precomputed_distances[tile];
        for sliding_factor in 1..distances_to_edge[index + 4] + 1 {
            target_tile = tile as i8 + offset * sliding_factor as i8;
            if board[target_tile as usize] & piece_color > 0 {
                break;
            } else if board[target_tile as usize] & enemy_piece_color > 0 {
                available_moves_board[target_tile as usize] = true;
                break;
            } else {
                available_moves_board[target_tile as usize] = true;
            }
        }
    }
    draw_movement_board(available_moves_board);
    return available_moves_board
}

fn knight_movement_from_tile(board: [u8; 64], piece: u8, tile: usize, precomputed_distances: [[u8; 8]; 64]) -> [bool; 64] {
    let mut available_moves_board = [false; 64];
    let piece_color: u8;
    if piece & COLORS::WHITE > 0 {
        piece_color = COLORS::WHITE;
    } else {
        piece_color = COLORS::BLACK;
    }
    let offsets: [i8; 8] = [-15, -6, 10, 17, 15, 6, -10, -17];
    let precomputed_distances_to_edge = [precomputed_distances[tile][0], 
                                                precomputed_distances[tile][3], 
                                                precomputed_distances[tile][1], 
                                                precomputed_distances[tile][2]];
    let mut distance_to_edge: u8 = 0;
    for (index, offset) in offsets.iter().enumerate() {
        let mut target_tile = tile as i8 + offset;
        if index == 0 {
            if precomputed_distances_to_edge[0] > 1 && precomputed_distances_to_edge[1] > 0 {
                if board[target_tile as usize] & piece_color > 0 {
                    continue;
                } else {
                    available_moves_board[target_tile as usize] = true;
                }
            }
        } else if index == 1 {
            if precomputed_distances_to_edge[0] > 0 && precomputed_distances_to_edge[1] > 1 {
                if board[target_tile as usize] & piece_color > 0 {
                    continue;
                } else {
                    available_moves_board[target_tile as usize] = true;
                }
            }
        } else if index == 2 {
            if precomputed_distances_to_edge[1] > 1 && precomputed_distances_to_edge[2] > 0 {
                if board[target_tile as usize] & piece_color > 0 {
                    continue;
                } else {
                    available_moves_board[target_tile as usize] = true;
                }
            }
        } else if index == 3 {
            if precomputed_distances_to_edge[1] > 0 && precomputed_distances_to_edge[2] > 1 {
                if board[target_tile as usize] & piece_color > 0 {
                    continue;
                } else {
                    available_moves_board[target_tile as usize] = true;
                }
            }
        } else if index == 4 {
            if precomputed_distances_to_edge[2] > 1 && precomputed_distances_to_edge[3] > 0 {
                if board[target_tile as usize] & piece_color > 0 {
                    continue;
                } else {
                    available_moves_board[target_tile as usize] = true;
                }
            }
        } else if index == 5 {
            if precomputed_distances_to_edge[2] > 0 && precomputed_distances_to_edge[3] > 1 {
                if board[target_tile as usize] & piece_color > 0 {
                    continue;
                } else {
                    available_moves_board[target_tile as usize] = true;
                }
            }
        } else if index == 6 {
            if precomputed_distances_to_edge[3] > 1 && precomputed_distances_to_edge[0] > 0 {
                if board[target_tile as usize] & piece_color > 0 {
                    continue;
                } else {
                    available_moves_board[target_tile as usize] = true;
                }
            }
        } else if index == 7 {
            if precomputed_distances_to_edge[3] > 0 && precomputed_distances_to_edge[0] > 1 {
                if board[target_tile as usize] & piece_color > 0 {
                    continue;
                } else {
                    available_moves_board[target_tile as usize] = true;
                }
            }
        }
    }
    draw_movement_board(available_moves_board);
    return available_moves_board
}

fn pawn_movement_from_tile(tile: usize, precomputed_distances: [[u8; 8]; 64]) -> [bool; 64] {
    let mut board = [false; 64];
    let offsets: [i8; 8] = [-8, 8, -1, 1, -9, 9, -7, 7];
    for (index, offset) in offsets.iter().enumerate() {
        let distances_to_edge = precomputed_distances[tile];
        if distances_to_edge[index] > 0 {
            board[(tile as i8 + offset) as usize] = true;
        }
    }
    draw_movement_board(board);
    return board
}

fn draw_movement_board(board: [bool; 64]) {
    let mut rank  = 1;
    for available in board {
        if available {
            print!("|{} ", available);
        } else if !available {
            print!("|{}", available);
        }
        if rank % 8 == 0 {
            print!("|{}", "\n");
        }
        rank += 1;
    }
}

pub fn init_game() -> GAME {
    let mut game = GAME {
        computed_distances: GAME::tiles_to_the_edge(),
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