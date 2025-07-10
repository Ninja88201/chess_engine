use std::i32;

use chess_lib::{Board, Colour, Move, MoveList, Piece};
use crate::evaluation::{consts, evaluate};
use crate::evaluation::helper;
use crate::transposition::{TTEntry, TTFlag, TranspositionTable};

const NEG_INFINITY: i32 = i32::MIN + 1;
const POS_INFINITY: i32 = i32::MAX;

pub fn search(board: &mut Board, depth: i32, mut alpha: i32, beta: i32, tt: &mut TranspositionTable) -> i32 {
    if depth == 0 {
        return quiescence(board, alpha, beta);
    }

    let hash = board.zobrist_hash;
    let original_alpha = alpha;

    if let Some(entry) = tt.get(hash) {
        if entry.depth >= depth {
            match entry.flag {
                TTFlag::Exact => return entry.score,
                TTFlag::LowerBound => alpha = alpha.max(entry.score),
                TTFlag::UpperBound => {
                    if entry.score <= alpha {
                        return entry.score;
                    }
                }
                
            }
            if alpha >= beta {
                return alpha;
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

    let mut scored_moves: Vec<(Move, i32)> = moves.iter()
        .map(|&mv| (mv, score_move(mv)))
        .collect();

    scored_moves.sort_by(|a, b| b.1.cmp(&a.1));

    let mut best_score = NEG_INFINITY;
    let mut best_move = None;

    for (mv, _) in scored_moves {
        board.make_move_unchecked(mv);
        let score = -search(board, depth - 1, -beta, -alpha, tt);
        board.undo_move();

        if score > best_score {
            best_score = score;
            best_move = Some(mv)
        }
        if best_score > alpha {
            alpha = best_score;
        }
        if alpha >= beta {
            break;  // Beta cutoff
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
        best_move: best_move,
    });

    best_score
}


pub fn find_best_move(board: &mut Board, depth: i32) -> Option<Move> {
    let mut tt = TranspositionTable::new();
    let mut moves = MoveList::new();
    board.generate_legal_moves(board.turn, &mut moves);

    if moves.is_empty() {
        return None;
    }

    let mut best_move = None;
    let mut best_score = NEG_INFINITY;
    let mut alpha = NEG_INFINITY;
    let beta = POS_INFINITY;

    for &mv in moves.iter() {
        board.make_move_unchecked(mv);
        let score = -search(board, depth - 1, -beta, -alpha, &mut tt);
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
fn quiescence(board: &mut Board, mut alpha: i32, beta: i32) -> i32 {
    let mut eval = evaluate(board);
    if board.turn == Colour::Black {
        eval = -eval;
    }

    if eval >= beta {
        return beta;
    }
    if alpha < eval {
        alpha = eval;
    }

    let mut moves = MoveList::new();
    board.generate_legal_captures(board.turn, &mut moves);

    let mut scored_moves: Vec<(Move, i32)> = moves.iter()
        .map(|&mv| (mv, score_move(mv)))
        .collect();

    scored_moves.sort_by(|a, b| b.1.cmp(&a.1));

    for (mv, _) in scored_moves.iter() {
        board.make_move_unchecked(*mv);
        let score = -quiescence(board, -beta, -alpha);
        board.undo_move();

        if score >= beta {
            return beta;
        }
        if score > alpha {
            alpha = score;
        }
    }

    alpha
}