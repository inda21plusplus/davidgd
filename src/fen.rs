
pub fn set_starting_position(board: [[u8; 8]; 8]) -> [[u8; 8]; 8] {
    let starting_fen_string: &str = "rnbqkbnr/pppppppp/8/8/8/8PPPPPPPP/RNBQKBNR";

    println!("{:?}", board);

    for (i, c) in starting_fen_string.chars().enumerate() {
        if c.is_lowercase() {
            // go through every white 
            println!("{}", c);
        } else if c.is_uppercase() {
            // go through every black piece
            println!("{}", c);
        } else if c.is_numeric() {
            // skip as many cols
            println!("{}", c);
        } else if c == '/' {
            // skip to next row
            println!("{}", "/");
        } else {
            return board
        }
    }

    board
}