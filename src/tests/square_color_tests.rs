#[cfg(test)]
pub mod test {
    use crate::WasmChess;
    use crate::tsify_structs::{others::*, *};

    #[test]
    fn square_color_parsing_ok() {
        let chess = WasmChess::new(None).unwrap();

        let sq_from_str_1 = "a5".parse();
        let sq_from_str_2 = "c5".parse();
        let sq_from_str_3 = "C5".parse::<SquareStr>();

        assert!(sq_from_str_1.is_ok());
        assert!(sq_from_str_2.is_ok());
        assert!(sq_from_str_3.is_err());

        let sq_from_str_1 = sq_from_str_1.unwrap();
        let sq_from_str_2 = sq_from_str_2.unwrap();

        let sq_color_1 = chess.square_color(SquareStr::A1);
        let sq_color_2 = chess.square_color(sq_from_str_1);
        let sq_color_3 = chess.square_color(sq_from_str_2);

        assert!(sq_color_1.is_some());
        assert!(sq_color_2.is_some());
        assert!(sq_color_3.is_some());
    }

    /// test cases from chess.js
    ///
    /// @link https://github.com/jhlywa/chess.js/blob/master/__tests__/square-color.test.ts
    #[test]
    fn square_color_light_for_light_squares() {
        let chess = WasmChess::new(None).unwrap();

        let square_1 = chess.square_color(SquareStr::A8).unwrap();
        let square_2 = chess.square_color(SquareStr::H1).unwrap();
        let square_3 = chess.square_color(SquareStr::E4).unwrap();

        pretty_assertions::assert_eq!(square_1, SquareColor::Light);
        pretty_assertions::assert_eq!(square_2, SquareColor::Light);
        pretty_assertions::assert_eq!(square_3, SquareColor::Light);
    }

    /// test cases from chess.js
    ///
    /// @link https://github.com/jhlywa/chess.js/blob/master/__tests__/square-color.test.ts
    #[test]
    fn square_color_dark_for_dark_squares() {
        let chess = WasmChess::new(None).unwrap();

        let square_1 = chess.square_color(SquareStr::A1).unwrap();
        let square_2 = chess.square_color(SquareStr::H8).unwrap();
        let square_3 = chess.square_color(SquareStr::D4).unwrap();

        pretty_assertions::assert_eq!(square_1, SquareColor::Dark);
        pretty_assertions::assert_eq!(square_2, SquareColor::Dark);
        pretty_assertions::assert_eq!(square_3, SquareColor::Dark);
    }
}
