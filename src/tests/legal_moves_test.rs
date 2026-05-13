#[cfg(test)]
pub mod legal_moves_test {
    use crate::{
        WasmChess,
        tsify_structs::{PieceSymbol, SquareStr, others::LegalMovesFilterOptions},
    };

    #[test]
    fn moves() {
        let chess = WasmChess::new(None).unwrap();
        let moves = "a3 a4 b3 b4 c3 c4 d3 d4 e3 e4 f3 f4 g3 g4 h3 h4 Na3 Nc3 Nf3 Nh3"
            .split_whitespace()
            .collect::<Vec<&str>>();

        let legal_moves = chess.legal_moves_san(None);

        pretty_assertions::assert_eq!(legal_moves.len(), moves.len());

        legal_moves.iter().for_each(|san_move| {
            let m = moves
                .iter()
                .find(|m| *m == san_move)
                .expect("Should always be Some");

            pretty_assertions::assert_eq!(m.len(), san_move.len());
        });
    }

    #[test]
    fn single_square() {
        let chess = WasmChess::new(None).unwrap();
        let moves = "e3 e4".split_whitespace().collect::<Vec<&str>>();

        let legal_moves = chess.legal_moves_san(Some(LegalMovesFilterOptions {
            from_square: Some(SquareStr::E2),
            piece: None,
        }));

        pretty_assertions::assert_eq!(legal_moves.len(), moves.len());

        legal_moves.iter().for_each(|san_move| {
            let m = moves
                .iter()
                .find(|m| *m == san_move)
                .expect("Should always be Some");

            pretty_assertions::assert_eq!(m.len(), san_move.len());
        });
    }

    #[test]
    fn single_square_pinned_piece() {
        let fen = "rnbqk1nr/pppp1ppp/4p3/8/1b1P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 2 3".to_string();
        let chess = WasmChess::new(Some(fen)).unwrap();

        let legal_moves = chess.legal_moves_san(Some(LegalMovesFilterOptions {
            from_square: Some(SquareStr::C3),
            piece: None,
        }));

        pretty_assertions::assert_eq!(legal_moves.len(), 0);
    }

    #[test]
    fn single_square_promotion() {
        let fen = "8/k7/8/8/8/8/7p/K7 b - - 0 1".to_string();
        let chess = WasmChess::new(Some(fen)).unwrap();

        let moves = "h1=N h1=B h1=R+ h1=Q+"
            .split_whitespace()
            .collect::<Vec<&str>>();

        let legal_moves = chess.legal_moves_san(Some(LegalMovesFilterOptions {
            from_square: Some(SquareStr::H2),
            piece: None,
        }));
        pretty_assertions::assert_eq!(legal_moves.len(), moves.len());

        legal_moves.iter().for_each(|san_move| {
            let m = moves
                .iter()
                .find(|m| *m == san_move)
                .expect("Should always be Some");

            pretty_assertions::assert_eq!(m.len(), san_move.len());
        });
    }

    #[test]
    fn single_square_castling() {
        let fen =
            "r1bq1rk1/1pp2ppp/p1np1n2/2b1p3/2B1P3/2NP1N2/PPPBQPPP/R3K2R w KQ - 0 8".to_string();
        let chess = WasmChess::new(Some(fen)).unwrap();

        let moves = "Kf1 Kd1 O-O O-O-O"
            .split_whitespace()
            .collect::<Vec<&str>>();

        let legal_moves = chess.legal_moves_san(Some(LegalMovesFilterOptions {
            from_square: Some(SquareStr::E1),
            piece: None,
        }));
        pretty_assertions::assert_eq!(legal_moves.len(), moves.len());

        legal_moves.iter().for_each(|san_move| {
            let m = moves
                .iter()
                .find(|m| *m == san_move)
                .expect("Should always be Some");

            pretty_assertions::assert_eq!(m.len(), san_move.len());
        });
    }

    #[test]
    fn single_square_no_castling() {
        let fen =
            "r1bq1rk1/1pp2ppp/p1np1n2/2b1p3/2B1P3/2NP1N2/PPPBQPPP/R3K2R w - - 0 8".to_string();
        let chess = WasmChess::new(Some(fen)).unwrap();

        let moves = "Kf1 Kd1".split_whitespace().collect::<Vec<&str>>();

        let legal_moves = chess.legal_moves_san(Some(LegalMovesFilterOptions {
            from_square: Some(SquareStr::E1),
            piece: None,
        }));
        pretty_assertions::assert_eq!(legal_moves.len(), moves.len());

        legal_moves.iter().for_each(|san_move| {
            let m = moves
                .iter()
                .find(|m| *m == san_move)
                .expect("Should always be Some");

            pretty_assertions::assert_eq!(m.len(), san_move.len());
        });
    }

    #[test]
    fn single_square_trapped_king() {
        let fen = "8/7K/8/8/1R6/k7/1R1p4/8 b - - 0 1".to_string();
        let chess = WasmChess::new(Some(fen)).unwrap();

        let legal_moves = chess.legal_moves_san(Some(LegalMovesFilterOptions {
            from_square: Some(SquareStr::A3),
            piece: None,
        }));

        pretty_assertions::assert_eq!(legal_moves.len(), 0);
    }

    // TODO: implement
    fn single_square_verbose() {
        //
    }

    #[test]
    fn moves_piece_filter() {
        let chess = WasmChess::new(None).unwrap();
        let moves = "Na3 Nc3 Nf3 Nh3".split_whitespace().collect::<Vec<&str>>();

        let legal_moves = chess.legal_moves_san(Some(LegalMovesFilterOptions {
            from_square: None,
            piece: Some(PieceSymbol::N),
        }));
        pretty_assertions::assert_eq!(legal_moves.len(), moves.len());

        legal_moves.iter().for_each(|san_move| {
            let m = moves
                .iter()
                .find(|m| *m == san_move)
                .expect("Should always be Some");

            pretty_assertions::assert_eq!(m.len(), san_move.len());
        });
    }

    #[test]
    fn moves_piece_filter_en_passant() {
        let fen =
            "rnbq1rk1/4bpp1/p2p1n1p/Ppp1p3/2B1P3/2NP1N1P/1PP2PP1/R1BQ1RK1 w - b6 0 10".to_string();
        let chess = WasmChess::new(Some(fen)).unwrap();

        let moves = "axb6 b3 b4 d4 g3 g4 h4"
            .split_whitespace()
            .collect::<Vec<&str>>();

        let legal_moves = chess.legal_moves_san(Some(LegalMovesFilterOptions {
            from_square: None,
            piece: Some(PieceSymbol::P),
        }));
        pretty_assertions::assert_eq!(legal_moves.len(), moves.len());

        legal_moves.iter().for_each(|san_move| {
            let m = moves
                .iter()
                .find(|m| *m == san_move)
                .expect("Should always be Some");

            pretty_assertions::assert_eq!(m.len(), san_move.len());
        });
    }

    #[test]
    fn moves_piece_filter_no_such_piece() {
        let fen = "r1bq1rk1/1pp2ppp/p1np1n2/2b1p3/4P3/2NP1N2/PPP1QPPP/R3K2R w KQ - 0 8".to_string();
        let chess = WasmChess::new(Some(fen)).unwrap();

        let legal_moves = chess.legal_moves_san(Some(LegalMovesFilterOptions {
            from_square: None,
            piece: Some(PieceSymbol::B),
        }));
        pretty_assertions::assert_eq!(legal_moves.len(), 0);
    }

    // #[test]
    fn moves_verbose_piece_filter() {
        // TODO:
    }

    #[test]
    fn moves_square_and_piece_filter() {
        let fen = "5rk1/1p3rp1/p1n1p3/2p1p2p/2PpP1qP/P2P2P1/1P2QP1K/3R1R2 w - - 0 23".to_string();
        let chess = WasmChess::new(Some(fen)).unwrap();

        let moves = "Qd2 Qc2 Qe1 Qe3 Qf3 Qxg4"
            .split_whitespace()
            .collect::<Vec<&str>>();

        let legal_moves = chess.legal_moves_san(Some(LegalMovesFilterOptions {
            from_square: Some(SquareStr::E2),
            piece: Some(PieceSymbol::Q),
        }));
        pretty_assertions::assert_eq!(legal_moves.len(), moves.len());

        legal_moves.iter().for_each(|san_move| {
            let m = moves
                .iter()
                .find(|m| *m == san_move)
                .expect("Should always be Some");

            pretty_assertions::assert_eq!(m.len(), san_move.len());
        });
    }

    fn moves_no_kings() {
        // TODO:
    }
}
