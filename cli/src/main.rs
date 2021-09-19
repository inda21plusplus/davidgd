
use chess_logic::*;

use std::{collections::HashMap};
use std::io::{self, BufRead};

fn main() {
    let mut symbol_to_piece = HashMap::new();
    symbol_to_piece.insert("bk", "\u{2654}");
    symbol_to_piece.insert("bq", "\u{2655}");
    symbol_to_piece.insert("br", "\u{2656}");
    symbol_to_piece.insert("bb", "\u{2657}");
    symbol_to_piece.insert("bn", "\u{2658}");
    symbol_to_piece.insert("bp", "\u{2659}");

    symbol_to_piece.insert("wk", "\u{265A}");
    symbol_to_piece.insert("wq", "\u{265B}");
    symbol_to_piece.insert("wr", "\u{265C}");
    symbol_to_piece.insert("wb", "\u{265D}");
    symbol_to_piece.insert("wn", "\u{265E}");
    symbol_to_piece.insert("wp", "\u{265F}");

    let game = init_game();    // array with starting position

    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let mut running = true;
    let mut command: String;

    while running {
        let board = game.get_board();
        draw_chess_board(board, &mut symbol_to_piece);              // todo - test for fen, move pieces, limit color to move

        
        command = iterator.next().unwrap().unwrap();
        
        println!("{}", command);

        if command == "quit" || command == "exit" {
            running = false;
        }
    }

    // if game.get_turn.is_whites_turn() {
    //     println("{}", 12)
    // }
    
    // get_location(pieces);

    // let status = get_game_status();

    // for event in status:

    //     event.is_whites_turn();

    //     event.is_in_check();

    //     event.is_checkmate();

    //     valid = move_to_if_valid(piece, tile);
}

pub fn draw_chess_board(board: [u8; 64], symbol_to_piece: &mut HashMap<&str, &str>) {
    let mut rank  = 1;
    let mut file = 8;
    println!("{}", "   A  B  C  D  E  F  G  H");
    print!(" {}", file);
    for piece in board {
        if piece == 0 {
            print!("|{} ", " ");
        } else if is_black_king(piece) {
            print!("|{} ", symbol_to_piece.get("bk").unwrap());
        } else if is_black_queen(piece) {
            print!("|{} ", symbol_to_piece.get("bq").unwrap());
        } else if is_black_rook(piece) {
            print!("|{} ", symbol_to_piece.get("br").unwrap());
        } else if is_black_bishop(piece) {
            print!("|{} ", symbol_to_piece.get("bb").unwrap());
        } else if is_black_knight(piece) {
            print!("|{} ", symbol_to_piece.get("bn").unwrap());
        } else if is_black_pawn(piece) {
            print!("|{} ", symbol_to_piece.get("bp").unwrap());
        } else if is_white_king(piece) {
            print!("|{} ", symbol_to_piece.get("wk").unwrap());
        } else if is_white_queen(piece) {
            print!("|{} ", symbol_to_piece.get("wq").unwrap());
        } else if is_white_rook(piece) {
            print!("|{} ", symbol_to_piece.get("wr").unwrap());
        } else if is_white_bishop(piece) {
            print!("|{} ", symbol_to_piece.get("wb").unwrap());
        } else if is_white_knight(piece) {
            print!("|{} ", symbol_to_piece.get("wn").unwrap());
        } else if is_white_pawn(piece) {
            print!("|{} ", symbol_to_piece.get("wp").unwrap());
        }
        if rank % 8 == 0 {
            file -= 1;
            print!("|{}", "\n");
            if file > 0 {
            print!(" {}", file);
            }
        }
        rank += 1;
    }
}
