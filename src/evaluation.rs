use chess_lib::{Board, Piece};

pub mod consts;
pub use consts::{CHECKMATE_SCORE, PAWN_VALUE, KNIGHT_VALUE, BISHOP_VALUE, ROOK_VALUE, QUEEN_VALUE};
pub mod helper;

pub fn evaluate(board: &Board) -> i32 {
    let white_val = material_eval(board, true);
    let black_val = material_eval(board, false);

    white_val - black_val
}
pub fn material_eval(board: &Board, white: bool) -> i32 {
    let mut eval = 0;
    let (player, _) = board.get_players(white);
    let pieces = Piece::ALL_PIECES.iter().filter(|&&p| p != Piece::King);
    for &piece in pieces {
        eval += player.bb[piece as usize].count_ones() as i32 * helper::piece_value(piece);
    }

    eval
}