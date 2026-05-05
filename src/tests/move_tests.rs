#[cfg(test)]
pub mod move_tests {
    use crate::WasmChess;

    #[test]
    fn move_works_standart_algebraic_notation() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
        let fen_next = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1".to_string();

        let mut chess = WasmChess::new(Some(fen.clone())).unwrap();

        let e4_move = chess.make_move("e4").unwrap();

        pretty_assertions::assert_eq!(e4_move.captured, None);
        pretty_assertions::assert_eq!(e4_move.promotion, None);
        pretty_assertions::assert_eq!(e4_move.is_en_passant, false);
        pretty_assertions::assert_eq!(e4_move.is_castle, false);
        pretty_assertions::assert_eq!(e4_move.after, chess.fen(None));
        pretty_assertions::assert_eq!(chess.fen(None), fen_next);

        pretty_assertions::assert_eq!(
            chess.zobrist_hash(),
            WasmChess::new(Some(fen_next)).unwrap().zobrist_hash()
        );

        chess.undo();

        pretty_assertions::assert_eq!(
            chess.zobrist_hash(),
            WasmChess::new(Some(fen)).unwrap().zobrist_hash()
        );
    }
}
