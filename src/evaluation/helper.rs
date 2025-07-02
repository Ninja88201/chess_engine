use chess_lib::Piece;
use crate::evaluation::{PAWN_VALUE, KNIGHT_VALUE, BISHOP_VALUE, ROOK_VALUE, QUEEN_VALUE};

pub fn piece_value(piece: Piece) -> i32 {
    match piece {
        Piece::Pawn => PAWN_VALUE,
        Piece::Knight => KNIGHT_VALUE,
        Piece::Bishop => BISHOP_VALUE,
        Piece::Rook => ROOK_VALUE,
        Piece::Queen => QUEEN_VALUE,
        Piece::King => 0,
    }
}