#[cfg(test)]
mod set_turn_tests {
    use crate::{WasmChess, models::utils::ColorChar};

    #[test]
    fn set_turn_for_current_color_returns_false() {
        let fen = "2r5/1p2k3/2n1p2p/2P1B1p1/1P2K3/3R1PP1/7P/8 w - - 1 39".to_string();
        let mut chess = WasmChess::new(Some(fen)).unwrap();

        pretty_assertions::assert_eq!(chess.set_turn(ColorChar::W), Ok(false));
        pretty_assertions::assert_eq!(chess.turn(), ColorChar::W);
    }

    #[test]
    fn set_turn_for_opposite_color_returns_false() {
        let fen = "2r5/1p2k3/2n1p2p/2P1B1p1/1P2K3/3R1PP1/7P/8 w - - 1 39".to_string();
        let mut chess = WasmChess::new(Some(fen)).unwrap();

        pretty_assertions::assert_eq!(chess.set_turn(ColorChar::B), Ok(true));
        pretty_assertions::assert_eq!(chess.turn(), ColorChar::B);
    }

    #[test]
    fn set_turn_throws_if_change_color_in_check() {
        let fen = "2r5/1p2k3/2nBp2p/2P3p1/1P2K3/3R1PP1/7P/8 b - - 2 39".to_string();
        let mut chess = WasmChess::new(Some(fen)).unwrap();

        let result = chess.set_turn(ColorChar::W);

        assert!(result.is_err());
        pretty_assertions::assert_eq!(chess.turn(), ColorChar::B);
    }
}
