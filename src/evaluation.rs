use chess_lib::{Board, Colour, Piece, Tile};

pub mod consts;
pub use consts::{CHECKMATE_SCORE, PAWN_VALUE, KNIGHT_VALUE, BISHOP_VALUE, ROOK_VALUE, QUEEN_VALUE};
pub use consts::{PAWN_PTT, KNIGHT_PTT, BISHOP_PTT, ROOK_PTT, QUEEN_PTT, KING_EARLY_PTT, KING_ENDGAME_PTT};
pub mod helper;

pub fn evaluate(board: &Board) -> i32 {
    let white_val = player_evaluation(board, Colour::White);
    let black_val = player_evaluation(board, Colour::Black);

    white_val - black_val
}
pub fn player_evaluation(board: &Board, colour: Colour) -> i32 {
    let mut eval = 0;
    let (player, _) = board.get_players(colour);
    let phase = game_phase(board);

    for &piece in Piece::ALL_PIECES.iter() {
        let bb = player.bb[piece as usize];

        for tile in bb {
            // Piece evaluation
            let base_val = piece_value(piece);

            // Piece Tile Tables
            let ptt_index = match colour {
                Colour::White => tile.to_usize(),
                Colour::Black => mirror_tile(tile).to_usize(),
            };

            let ptt_val = match piece {
                Piece::Pawn => PAWN_PTT[ptt_index],
                Piece::Knight => KNIGHT_PTT[ptt_index],
                Piece::Bishop => BISHOP_PTT[ptt_index],
                Piece::Rook => ROOK_PTT[ptt_index],
                Piece::Queen => QUEEN_PTT[ptt_index],
                Piece::King => {
                    let early = KING_EARLY_PTT[ptt_index];
                    let end = KING_ENDGAME_PTT[ptt_index];
                    blend_king_ptt(early, end, phase)
                }
            };

            eval += base_val + ptt_val;
        }
    }

    eval
}
fn game_phase(board: &Board) -> i32 {
    let mut phase = 0;

    for &colour in &[Colour::White, Colour::Black] {
        let (player, _) = board.get_players(colour);
        phase += player.bb[Piece::Knight as usize].count_ones() * 1;
        phase += player.bb[Piece::Bishop as usize].count_ones() * 1;
        phase += player.bb[Piece::Rook as usize].count_ones() * 2;
        phase += player.bb[Piece::Queen as usize].count_ones() * 4;
    }

    let max_phase = 24;

    let normalized_phase = (phase.min(max_phase) * 16 / max_phase) as i32;
    normalized_phase
}

fn blend_king_ptt(early: i32, end: i32, phase: i32) -> i32 {
    let adjusted_phase = phase.max(8);
    let early_weight = adjusted_phase;
    let end_weight = 16 - adjusted_phase;
    (early * early_weight + end * end_weight) / 16
}
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
pub fn mirror_tile(tile: Tile) -> Tile {
    let index = tile.to_u8();
    let mirrored_index = index ^ 56;
    Tile::new_unchecked(mirrored_index)
}