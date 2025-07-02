pub mod evaluation;
pub mod search;

#[cfg(test)]
mod tests
{
    use chess_lib::{Board, Piece, Tile};

    use crate::search::find_best_move;

    #[test]
    fn test_1() {
        let mut board = Board::new_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w - - 0 1").unwrap();
        board.try_move_piece(Tile::E2, Tile::A6, None);
        println!("Current board: {}", board);
        let best = find_best_move(&mut board, 4);

        if let Some(m) = best {
            println!("Best move: {}", m.to_string());
        }

    }
    #[test]
    fn mate_in_one() {
        let mut board = Board::new_from_fen("rnbqkbnr/1ppp1ppp/p7/4p2Q/2B1P3/8/PPPP1PPP/RNB1K1NR w KQkq - 0 1").unwrap();
        let best = find_best_move(&mut board, 4);
        assert_eq!(best, Some(board.create_move(Tile::H5, Tile::F7, Piece::Queen, Some(Piece::Pawn), None)))
    }

}