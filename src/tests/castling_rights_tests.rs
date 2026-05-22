/// tests taken from chess.js test suite for set/get castlingRights with few exceptions
///
/// @link https://github.com/jhlywa/chess.js/blob/master/__tests__/castling-rights.test.ts

#[cfg(test)]
pub mod castling_rights_tests {
    use crate::{ColorChar, WasmChess, models::utils::CastlingObj};

    #[test]
    fn correct_rights_from_set_fen() {
        let no_castling_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w Aa - 0 1";

        let chess = WasmChess::new(Some(no_castling_fen.to_string())).unwrap();

        pretty_assertions::assert_eq!(
            chess.get_castling_rights(ColorChar::W),
            CastlingObj {
                king: Some(false),
                queen: Some(true)
            }
        );

        pretty_assertions::assert_eq!(
            chess.get_castling_rights(ColorChar::B),
            CastlingObj {
                king: Some(false),
                queen: Some(true)
            }
        );
    }

    #[test]
    fn no_castling_after_rook_move() {
        let no_castling_fen = "rnbqkbnr/8/8/8/8/8/8/RNBQKBNR w AHah - 0 1";

        let mut chess = WasmChess::new(Some(no_castling_fen.to_string())).unwrap();

        pretty_assertions::assert_eq!(
            chess.get_castling_rights(ColorChar::W),
            CastlingObj {
                king: Some(true),
                queen: Some(true)
            }
        );

        pretty_assertions::assert_eq!(
            chess.get_castling_rights(ColorChar::B),
            CastlingObj {
                king: Some(true),
                queen: Some(true)
            }
        );

        chess.play_move("Ra2").unwrap();
        chess.play_move("Rh7").unwrap();

        pretty_assertions::assert_eq!(
            chess.get_castling_rights(ColorChar::W),
            CastlingObj {
                king: Some(true),
                queen: Some(false)
            }
        );
        pretty_assertions::assert_eq!(
            chess.get_castling_rights(ColorChar::B),
            CastlingObj {
                king: Some(false),
                queen: Some(true)
            }
        );
    }

    #[test]
    fn no_castling_after_king_move() {
        let no_castling_fen = "rnbqkbnr/8/8/8/8/8/8/RNBQKBNR w AHah - 0 1";

        let mut chess = WasmChess::new(Some(no_castling_fen.to_string())).unwrap();

        pretty_assertions::assert_eq!(
            chess.get_castling_rights(ColorChar::W),
            CastlingObj {
                king: Some(true),
                queen: Some(true)
            }
        );

        chess.play_move("Ke2").unwrap();

        pretty_assertions::assert_eq!(
            chess.get_castling_rights(ColorChar::W),
            CastlingObj {
                king: Some(false),
                queen: Some(false)
            }
        );
    }

    #[test]
    fn set_castling_clear_white_kingside() {
        let mut chess = WasmChess::new(None).unwrap();

        let ok = chess.set_castling_rights(
            ColorChar::W,
            CastlingObj {
                king: Some(false),
                queen: None,
            },
        );

        pretty_assertions::assert_eq!(ok, true);

        let castling_rights = chess.get_castling_rights(ColorChar::W);

        pretty_assertions::assert_eq!(castling_rights.king, Some(false));

        let ok = chess.set_castling_rights(
            ColorChar::W,
            CastlingObj {
                king: Some(true),
                queen: None,
            },
        );

        pretty_assertions::assert_eq!(ok, true);

        let castling_rights = chess.get_castling_rights(ColorChar::W);
        pretty_assertions::assert_eq!(castling_rights.king, Some(true));
    }

    #[test]
    fn set_castling_clear_white_queenside() {
        let mut chess = WasmChess::new(None).unwrap();

        let ok = chess.set_castling_rights(
            ColorChar::W,
            CastlingObj {
                king: None,
                queen: Some(false),
            },
        );

        pretty_assertions::assert_eq!(ok, true);

        let castling_rights = chess.get_castling_rights(ColorChar::W);

        pretty_assertions::assert_eq!(castling_rights.queen, Some(false));
    }

    #[test]
    fn set_castling_clear_black_kingside() {
        let mut chess = WasmChess::new(None).unwrap();

        let ok = chess.set_castling_rights(
            ColorChar::B,
            CastlingObj {
                king: Some(false),
                queen: None,
            },
        );
        pretty_assertions::assert_eq!(ok, true);

        let castling_rights = chess.get_castling_rights(ColorChar::B);

        pretty_assertions::assert_eq!(castling_rights.king, Some(false));
    }

    #[test]
    fn set_castling_clear_black_queenside() {
        let mut chess = WasmChess::new(None).unwrap();

        let ok = chess.set_castling_rights(
            ColorChar::B,
            CastlingObj {
                king: None,
                queen: Some(false),
            },
        );

        pretty_assertions::assert_eq!(ok, true);

        let castling_rights = chess.get_castling_rights(ColorChar::B);

        pretty_assertions::assert_eq!(castling_rights.queen, Some(false));
    }

    #[test]
    fn set_castling_rights_set_white_kingside() {
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R w - - 0 1".to_string();
        let mut chess = WasmChess::new(Some(fen)).unwrap();

        let ok = chess.set_castling_rights(
            ColorChar::W,
            CastlingObj {
                king: Some(true),
                queen: None,
            },
        );

        pretty_assertions::assert_eq!(ok, true);

        let castling_rights = chess.get_castling_rights(ColorChar::W);

        pretty_assertions::assert_eq!(castling_rights.king, Some(true));
    }

    #[test]
    fn set_castling_rights_set_white_queenside() {
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R w - - 0 1".to_string();
        let mut chess = WasmChess::new(Some(fen)).unwrap();

        let ok = chess.set_castling_rights(
            ColorChar::W,
            CastlingObj {
                king: None,
                queen: Some(true),
            },
        );

        pretty_assertions::assert_eq!(ok, true);

        let castling_rights = chess.get_castling_rights(ColorChar::W);

        pretty_assertions::assert_eq!(castling_rights.queen, Some(true));
    }

    #[test]
    fn set_castling_rights_set_black_kingside() {
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R w - - 0 1".to_string();
        let mut chess = WasmChess::new(Some(fen)).unwrap();

        let ok = chess.set_castling_rights(
            ColorChar::B,
            CastlingObj {
                king: Some(true),
                queen: None,
            },
        );

        pretty_assertions::assert_eq!(ok, true);

        let castling_rights = chess.get_castling_rights(ColorChar::B);

        pretty_assertions::assert_eq!(castling_rights.king, Some(true));
    }

    #[test]
    fn set_castling_rights_set_black_queenside() {
        let fen = "r3k2r/8/8/8/8/8/8/R3K2R w - - 0 1".to_string();
        let mut chess = WasmChess::new(Some(fen)).unwrap();

        let ok = chess.set_castling_rights(
            ColorChar::B,
            CastlingObj {
                king: None,
                queen: Some(true),
            },
        );

        pretty_assertions::assert_eq!(ok, true);

        let castling_rights = chess.get_castling_rights(ColorChar::B);

        pretty_assertions::assert_eq!(castling_rights.queen, Some(true));
    }

    #[test]
    fn recovers_from_impossible_castling_set_attempt_white() {
        // king on a second row
        let fen = "2k4r/1q6/8/8/1P3P2/P3P1P1/3K4/R6R w - - 0 1".to_string();
        let mut chess = WasmChess::new(Some(fen)).unwrap();

        let ok = chess.set_castling_rights(
            ColorChar::W,
            CastlingObj {
                king: Some(true),
                queen: Some(true),
            },
        );

        pretty_assertions::assert_eq!(ok, false);

        let castling_rights = chess.get_castling_rights(ColorChar::W);

        pretty_assertions::assert_eq!(castling_rights.king, Some(false));
        pretty_assertions::assert_eq!(castling_rights.queen, Some(false));
    }

    #[test]
    fn recovers_from_impossible_castling_set_attempt_black() {
        // king moved (ok! might be a chess960 game),
        // no rook on a queenside
        let fen = "2k4r/1q6/8/8/1P3P2/P3P1P1/3K4/R6R b - - 0 1".to_string();
        let mut chess = WasmChess::new(Some(fen)).unwrap();

        let ok = chess.set_castling_rights(
            ColorChar::B,
            CastlingObj {
                king: Some(true),
                queen: Some(true),
            },
        );

        pretty_assertions::assert_eq!(ok, false);

        let castling_rights = chess.get_castling_rights(ColorChar::B);

        pretty_assertions::assert_eq!(castling_rights.king, Some(true));
        pretty_assertions::assert_eq!(castling_rights.queen, Some(false));
    }

    // TODO:
    // add eventually ?
    // #[test]
    fn fail_to_set_white_kingside() {
        // chess.clear()
    }
}
