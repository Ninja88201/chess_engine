use chess_lib::{Board, Piece};

mod piece_values;

pub fn evaluate(board: &Board) -> i32 {
    let white_val = material_eval(board, true);
    let black_val = material_eval(board, false);

    white_val - black_val
}
pub fn material_eval(board: &Board, white: bool) -> i32 {
    let mut eval = 0;
    let (player, _) = board.get_players(white);
    eval += player.bb[Piece::Pawn as usize].count_ones() as i32 * piece_values::pawnValue;
    eval += player.bb[Piece::Knight as usize].count_ones() as i32 * piece_values::knightValue;
    eval += player.bb[Piece::Bishop as usize].count_ones() as i32 * piece_values::bishopValue;
    eval += player.bb[Piece::Rook as usize].count_ones() as i32 * piece_values::rookValue;
    eval += player.bb[Piece::Queen as usize].count_ones() as i32 * piece_values::queenValue;
    eval
}