

pub fn generate_array_of_all_pieces() -> [u8; 12] {
    [pawn() + white(),
    rook() + white(),
    kight() + white(),
    bishop() + white(),
    queen() + white(),
    king() + white(),
    pawn() + black(),
    rook() + black(),
    kight() + black(),
    bishop() + black(),
    queen() + black(),
    king() + black()
    ]
}

pub fn is_black(piece: u8) -> bool {
    if (piece & black()) > 0 {
        true
    } else {
        false
    }
}

pub fn get_white_pawn() -> u8 {
    pawn() + white()
}
pub fn get_white_rook() -> u8 {
    rook() + white()
}
pub fn get_white_knight() -> u8 {
    kight() + white()
}
pub fn get_white_bishop() -> u8 {
    bishop() + white()
}
pub fn get_white_queen() -> u8 {
    queen() + white()
}
pub fn get_white_king() -> u8 {
    king() + white()
}

pub fn get_black_pawn() -> u8 {
    pawn() + black()
}
pub fn get_black_rook() -> u8 {
    rook() + black()
}
pub fn get_black_knight() -> u8 {
    kight() + black()
}
pub fn get_black_bishop() -> u8 {
    bishop() + black()
}
pub fn get_black_queen() -> u8 {
    queen() + black()
}
pub fn get_black_king() -> u8 {
    king() + black()
}

pub fn pawn() -> u8 {
    1
}
pub fn rook() -> u8 {
    2
}
pub fn kight() -> u8 {
    4
}
pub fn bishop() -> u8 {
    8
}
pub fn queen() -> u8 {
    16
}
pub fn king() -> u8 {
    32
}

pub fn black() -> u8 {
    64
}
pub fn white() -> u8 {
    128
}