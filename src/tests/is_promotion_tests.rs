#[cfg(test)]
/// all these tests are from chess.js test suite
///
/// @link https://github.com/jhlywa/chess.js/blob/master/__tests__/is-promotion.test.ts
pub mod is_promotion_tests {
    use crate::helpers::tsify::*;
    use crate::{WasmChess, helpers::tsify::MoveFromSquares};

    #[test]
    fn true_for_promotion() {
        let fen = "8/1PQ2pk1/3p2p1/3qp3/8/4P3/7p/2K5 w - - 0 56".to_string();
        let chess = WasmChess::new(Some(fen)).unwrap();

        pretty_assertions::assert_eq!(
            chess.is_promotion(MoveFromSquares {
                from: SquareStr::B7,
                to: SquareStr::B8
            }),
            true
        );
    }

    #[test]
    fn false_for_illegal_move() {
        let fen = "8/1PQ2pk1/3p2p1/3qp3/8/4P3/7p/2K5 w - - 0 56".to_string();
        let chess = WasmChess::new(Some(fen)).unwrap();

        pretty_assertions::assert_eq!(
            chess.is_promotion(MoveFromSquares {
                from: SquareStr::B7,
                to: SquareStr::C8
            }),
            false
        );
    }

    #[test]
    fn false_for_normal_pawn_move() {
        let fen = "r2qk2r/pR1nppbp/3p1np1/1BpP4/4PPb1/2N2N2/P1PB2PP/3QK2R w Kkq - 2 12".to_string();
        let chess = WasmChess::new(Some(fen)).unwrap();

        pretty_assertions::assert_eq!(
            chess.is_promotion(MoveFromSquares {
                from: SquareStr::E4,
                to: SquareStr::E5
            }),
            false
        );
    }

    #[test]
    fn false_for_non_pawn_move_to_eighth_rank() {
        let fen = "2r1k3/4bp2/p2p2p1/1p2p1P1/3qB3/5Q2/PPP2P2/1K5R w - - 0 25".to_string();
        let chess = WasmChess::new(Some(fen)).unwrap();

        pretty_assertions::assert_eq!(
            chess.is_promotion(MoveFromSquares {
                from: SquareStr::H1,
                to: SquareStr::H8
            }),
            false
        );
    }
}
