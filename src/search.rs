use std::i32;

use chess_lib::{Board, Colour, Move, MoveList, Piece};
use crate::evaluation::{consts, evaluate};
use crate::evaluation::helper;
use crate::transposition::{TTEntry, TTFlag, TranspositionTable};

const NEG_INFINITY: i32 = i32::MIN + 1;
const POS_INFINITY: i32 = i32::MAX;

pub fn search(board: &mut Board, depth: i32, mut alpha: i32, beta: i32, tt: &mut TranspositionTable) -> i32 {
    if depth == 0 {
        return cap_search(board, alpha, beta);
    }

    let hash = board.zobrist_hash;
    let original_alpha = alpha;

    let tt_entry = tt.get(hash);

    if let Some(entry) = tt_entry {
        if entry.depth >= depth {
            match entry.flag {
                TTFlag::Exact => return entry.score,
                TTFlag::LowerBound if entry.score >= beta => return entry.score,
                TTFlag::UpperBound if entry.score <= alpha => return entry.score,
                _ => {}
            }
        }
    }

    let mut moves = MoveList::new();
    board.generate_legal_moves(board.turn, &mut moves);

    if moves.is_empty() {
        return if board.is_in_check(board.turn) {
            consts::CHECKMATE_SCORE - depth
        } else {
            0
        };
    }

    let tt_best_move = tt_entry.and_then(|entry| entry.best_move);
    let ordered_move_indices = ordered_move_indices(&moves, tt_best_move);

    let mut best_score = NEG_INFINITY;
    let mut best_move = None;

    for index in ordered_move_indices {
        let mv = moves[index];
        board.make_move_unchecked(mv);
        let score = -search(board, depth - 1, -beta, -alpha, tt);
        board.undo_move();

        if score > best_score {
            best_score = score;
            best_move = Some(mv);
        }

        alpha = alpha.max(score);
        if alpha >= beta {
            break;
        }
    }

    let flag = if best_score <= original_alpha {
        TTFlag::UpperBound
    } else if best_score >= beta {
        TTFlag::LowerBound
    } else {
        TTFlag::Exact
    };

    tt.insert(hash, TTEntry {
        depth,
        flag,
        score: best_score,
        best_move,
    });

    best_score
}
fn ordered_move_indices(moves: &MoveList, tt_best_move: Option<Move>) -> [usize; 256] {
    let mut indexed_scores = [(0usize, 0i32); 256];
    let len = moves.len();

    for i in 0..len {
        let mv = moves[i];
        let score = if Some(mv) == tt_best_move {
            i32::MAX
        } else {
            score_move(mv)
        };
        indexed_scores[i] = (i, score);
    }

    indexed_scores[..len].sort_unstable_by(|a, b| b.1.cmp(&a.1));

    let mut sorted_indices = [0usize; 256];
    for i in 0..len {
        sorted_indices[i] = indexed_scores[i].0;
    }

    sorted_indices
}
fn score_move(mv: Move) -> i32 {
    if let Some(captured) = mv.capture() {
        let victim_val = helper::piece_value(captured);
        let attacker_val = helper::piece_value(mv.piece());
        return 10_000 + (victim_val - attacker_val);
    }

    if let Some(promoted_piece) = mv.promoted_to() {
        return match promoted_piece {
            Piece::Queen => consts::PROMOTION_BONUS_QUEEN,
            Piece::Rook => consts::PROMOTION_BONUS_ROOK,
            Piece::Bishop => consts::PROMOTION_BONUS_BISHOP,
            Piece::Knight => consts::PROMOTION_BONUS_KNIGHT,
            _ => 0,
        };
    }

    0
}
fn cap_search(board: &mut Board, mut alpha: i32, beta: i32) -> i32 {
    let mut eval = evaluate(board);
    if board.turn == Colour::Black {
        eval = -eval;
    }

    if eval >= beta {
        return beta;
    }
    alpha = alpha.max(eval);

    let mut moves = MoveList::new();
    board.generate_legal_captures(board.turn, &mut moves);

    let mut scored_moves: Vec<(Move, i32)> = moves.iter()
        .map(|&mv| (mv, score_move(mv)))
        .collect();

    scored_moves.sort_by(|a, b| b.1.cmp(&a.1));

    for (mv, _) in scored_moves {
        board.make_move_unchecked(mv);
        let score = -cap_search(board, -beta, -alpha);
        board.undo_move();

        if score >= beta {
            return beta;
        }
        alpha = alpha.max(score);
    }

    alpha
}
pub fn find_best_move(board: &mut Board, max_depth: i32) -> Option<Move> {
    let mut tt = TranspositionTable::new();
    let mut best_move = None;

    for depth in 1..=max_depth {
        let mut moves = MoveList::new();
        board.generate_legal_moves(board.turn, &mut moves);

        if moves.is_empty() {
            return None;
        }

        let mut best_score = NEG_INFINITY;
        let mut alpha = NEG_INFINITY;
        let beta = POS_INFINITY;

        let mut scored_moves: Vec<(Move, i32)> = moves.iter()
            .map(|&mv| (mv, score_move(mv)))
            .collect();

        scored_moves.sort_by(|a, b| b.1.cmp(&a.1));

        for (mv, _) in scored_moves {
            board.make_move_unchecked(mv);
            let score = -search(board, depth - 1, -beta, -alpha, &mut tt);
            board.undo_move();

            if score > best_score {
                best_score = score;
                best_move = Some(mv);
            }

            alpha = alpha.max(score);
        }
    }

    best_move
}