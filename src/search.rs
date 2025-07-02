use std::i32;

use chess_lib::{Board, Move, MoveList, Piece};

use crate::evaluation::{consts, evaluate};
use crate::evaluation::helper;

pub fn search(board: &mut Board, depth: i32, mut alpha: i32, beta: i32) -> i32 {
    if depth == 0 {
        return if board.white_turn {
            evaluate(board)
        } else {
            -evaluate(board)
        };
    }

    let mut moves = MoveList::new();
    board.generate_legal_moves(board.white_turn, &mut moves);

    if moves.len() == 0 {
        if board.is_in_check(board.white_turn) {
            return consts::CHECKMATE_SCORE - depth;
        } else {
            return 0;
        }
    }

    let mut best = i32::MIN + 1;

    for &mv in moves.iter() {
        board.make_move_unchecked(mv);
        let eval = -search(board, depth - 1, -beta, -alpha);
        board.undo_move();

        best = best.max(eval);
        alpha = alpha.max(eval);
        if alpha >= beta {
            break;
        }
    }

    best
}
pub fn find_best_move(board: &mut Board, depth: i32) -> Option<Move> {
    let mut moves = MoveList::new();
    board.generate_legal_moves(board.white_turn, &mut moves);

    if moves.len() == 0 {
        return None;
    }

    let mut scored_moves: Vec<(Move, i32)> = moves.iter().map(|&mv| {
        let score = score_move(mv);
        (mv, score)
    }).collect();

    scored_moves.sort_by(|a, b| b.1.cmp(&a.1));

    let mut best_move = None;
    let mut best_score = i32::MIN + 1;
    let mut alpha = i32::MIN + 1;
    let beta = i32::MAX;

    for (mv, _) in scored_moves {
        board.make_move_unchecked(mv);
        let score = -search(board, depth - 1, -beta, -alpha);
        board.undo_move();

        if score > best_score {
            best_score = score;
            best_move = Some(mv);
            alpha = alpha.max(score);
        }
    }

    best_move
}
fn score_move(mv: Move) -> i32 {
    if let Some(captured) = mv.capture() {
        let victim_val = helper::piece_value(captured);
        let attacker_val = helper::piece_value(mv.piece());
        return 10_000 + (victim_val - attacker_val);
    }

    if let Some(promoted_piece) = mv.promoted_to() {
        let promo_bonus = match promoted_piece {
            Piece::Queen => consts::PROMOTION_BONUS_QUEEN,
            Piece::Rook => consts::PROMOTION_BONUS_ROOK,
            Piece::Bishop => consts::PROMOTION_BONUS_BISHOP,
            Piece::Knight => consts::PROMOTION_BONUS_KNIGHT,
            _ => 0,
        };
        return promo_bonus;
    }

    0
}