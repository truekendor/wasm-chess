use shakmaty::{Role, uci::UciMove};

use super::*;

#[wasm_bindgen]
impl WasmChess {
    pub fn fen(&self, force_en_passant_square: Option<bool>) -> String {
        let en_passant_mode = match force_en_passant_square {
            Some(true) => shakmaty::EnPassantMode::Always,
            Some(false) => shakmaty::EnPassantMode::Legal,
            None => shakmaty::EnPassantMode::Legal,
        };
        let fen = Fen::from_position(&self.chess, en_passant_mode);

        fen.to_string()
    }

    /// Returns a FEN string representing only the piece placement on the board.
    ///
    /// ## Returns
    /// A FEN string in the format `"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"`
    /// describing the current piece layout.
    ///
    /// ## Example
    /// ```
    /// let board_fen = chess.board_fen();
    /// assert_eq!(board_fen, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    /// ```
    #[wasm_bindgen(js_name = "boardFen")]
    pub fn board_fen(&self) -> String {
        self.chess.board().board_fen().to_string()
    }

    pub fn board(&self) -> BoardState {
        const RANK_STRINGS: [&str; 8] = ["1", "2", "3", "4", "5", "6", "7", "8"];
        let mut result: BoardMatrix = Vec::with_capacity(8);
        let mut square_str = String::with_capacity(2);

        for rank in (1..=8).rev() {
            let mut row: BoardObjRow = Vec::with_capacity(8);

            for file in 'a'..='h' {
                square_str.clear();
                square_str.push(file);
                square_str.push_str(RANK_STRINGS[rank - 1 as usize]);

                let square = square_str.parse::<SquareStr>().unwrap(); // Safe because format is correct

                let shakmaty_square: Square = (&square).into();
                let piece = self.chess.board().piece_at(shakmaty_square);

                let square_info = match piece {
                    Some(p) => {
                        let color = match p.color {
                            Color::White => ColorChar::W,
                            Color::Black => ColorChar::B,
                        };

                        Some(SquareInfo {
                            color,
                            square,
                            r#type: PieceSymbol::from(&p),
                        })
                    }
                    None => None,
                };

                row.push(square_info);
            }

            result.push(row);
        }

        BoardState(result)
    }

    pub fn turn(&self) -> ColorChar {
        match self.chess.turn() {
            Color::White => ColorChar::W,
            Color::Black => ColorChar::B,
        }
    }

    /// Checks if a move from the given squares would result in a promotion.
    ///
    /// # Returns
    /// `true` if the move would promote a pawn, `false` otherwise
    #[wasm_bindgen(js_name = "isPromotion")]
    pub fn is_promotion(&self, move_obj: MoveFromSquares) -> bool {
        let mut move_str = String::with_capacity(5);
        move_str.push_str(&move_obj.from.as_str());
        move_str.push_str(&move_obj.to.as_str());

        // # Note
        // Uses knight as a temporary promotion piece to validate the move.
        // This works because any promotion piece would indicate a valid promotion.
        move_str.push('n');

        parsing::str_to_move(&move_str, &self.chess)
            .map(|internal_move| internal_move.is_promotion())
            .unwrap_or(false)
    }

    /// Gets the piece at a specific square.
    ///
    /// # JavaScript Example
    /// ```js
    /// const piece = chess.get('e2');
    /// if (piece) {
    ///   console.log(piece.color, piece.kind); // 'w', 'pawn'
    /// }
    /// ```
    ///
    /// # Returns
    /// - `PieceObj` if a piece exists at the square
    /// - `null` | `undefined` if the square is empty
    pub fn get(&self, square: SquareStr) -> Option<PieceObj> {
        let square = (&square).into();

        let Some(piece) = self.chess.board().piece_at(square) else {
            return None;
        };

        let piece_obj = PieceObj::from(&piece);

        Some(piece_obj)
    }

    /// Finds all squares containing a piece identified by its single-character notation.
    ///
    ///
    /// # JavaScript Example
    /// ```js
    /// // Find all white knights
    /// const knightSquares = chess.findPiece('N');
    ///
    /// // Find all black pawns
    /// const blackPawnSquares = chess.findPiece('p');
    /// ```
    ///
    /// # Parameters
    /// - `piece`: Single character piece notation:
    ///   - Uppercase: `K` (king), `Q` (queen), `R` (rook), `B` (bishop), `N` (knight), `P` (pawn) = white
    ///   - Lowercase: same letters = black
    ///
    /// # Returns
    /// Array of square strings (e.g., `['g1', 'b1']`)
    ///
    /// # Throws
    /// If the piece string is invalid (wrong length or unknown piece character)
    #[wasm_bindgen(js_name = "findPiece")]
    pub fn find_piece_from_str(&self, piece: &str) -> Result<Vec<SquareStr>, String> {
        let piece = piece.trim();

        if piece.len() != 1 {
            return Err(format!(
                "Error: unexpected piece length: {}\nPiece: {}",
                piece.len(),
                piece
            ));
        }

        let piece_char = piece
            .chars()
            .next()
            .ok_or_else(|| "Empty piece string".to_string())?;

        let piece_type = Piece::from_char(piece_char).ok_or_else(|| {
            format!(
                "Error parsing piece char: \"{}\" into a valid piece type",
                piece
            )
        })?;

        let mut squares_with_piece = Vec::new();

        for (sq, p) in self.chess.board().iter() {
            if p == piece_type {
                let square = SquareStr::from(&sq);
                squares_with_piece.push(square);
            }
        }

        Ok(squares_with_piece)
    }

    #[wasm_bindgen(js_name = "findPieceByType")]
    pub fn find_piece_from_obj(&self, piece: PieceObj) -> Vec<SquareStr> {
        let piece_type = Piece::from(&piece);

        let mut squares_with_piece: Vec<SquareStr> = vec![];

        for (sq, p) in self.chess.board().iter() {
            if p == piece_type {
                let square = SquareStr::from(&sq);

                squares_with_piece.push(square);
            }
        }

        squares_with_piece
    }

    #[wasm_bindgen(js_name = "squareColor")]
    pub fn square_color(&self, square: SquareStr) -> Option<SquareColor> {
        let square: Square = (&square).into();

        Some(if square.is_light() {
            SquareColor::Light
        } else {
            SquareColor::Dark
        })
    }

    #[wasm_bindgen(js_name = "isAttacked")]
    pub fn is_attacked(&self, square: SquareStr, attacked_by_side: Option<ColorChar>) -> bool {
        let square = (&square).into();

        let get_attackers = |color: Color| -> Vec<Square> {
            self.chess
                .board()
                .attacks_to(square, color, self.chess.board().by_color(color))
                .into_iter()
                .collect()
        };

        match attacked_by_side {
            Some(ColorChar::W) => !get_attackers(Color::White).is_empty(),
            Some(ColorChar::B) => !get_attackers(Color::Black).is_empty(),
            None => {
                let turn = self.chess.turn();
                !get_attackers(turn).is_empty()
            }
        }
    }

    pub fn attackers(
        &self,
        square: SquareStr,
        attacked_by_side: Option<ColorChar>,
    ) -> Vec<SquareStr> {
        let square = (&square).into();

        let get_attackers = |color: Color| -> Vec<Square> {
            self.chess
                .board()
                .attacks_to(
                    square,
                    color,
                    self.chess.board().occupied(), // .without(square)
                )
                .into_iter()
                .collect()
        };

        let w_attackers = get_attackers(Color::White);
        let b_attackers = get_attackers(Color::Black);

        let squares = match attacked_by_side {
            None => match self.chess.turn() {
                Color::White => w_attackers,
                Color::Black => b_attackers,
            },
            Some(ColorChar::W) => w_attackers,
            Some(ColorChar::B) => b_attackers,
        };

        squares.into_iter().map(|sq| SquareStr::from(&sq)).collect()
    }

    #[wasm_bindgen(js_name = "legalMovesUci")]
    pub fn legal_moves_uci(&self, filter_options: Option<LegalMovesFilterOptions>) -> Vec<String> {
        let filter_options = unwrap_filter_options(&filter_options);

        let legal_moves: Vec<String> = self
            .chess
            .legal_moves()
            .iter()
            .filter_map(|mov| {
                let move_filtered = filter_move(mov, &filter_options);
                if move_filtered {
                    return None;
                }

                let uci_move = UciMove::from_move(*mov, shakmaty::CastlingMode::Chess960);
                return Some(uci_move.to_string());
            })
            .collect();

        legal_moves
    }

    #[wasm_bindgen(js_name = "legalMovesSan")]
    pub fn legal_moves_san(&self, filter_options: Option<LegalMovesFilterOptions>) -> Vec<String> {
        let filter_options = unwrap_filter_options(&filter_options);

        let legal_moves: Vec<String> = self
            .chess
            .legal_moves()
            .iter()
            .filter_map(|mov| {
                let mut chess_clone = self.chess.clone();

                let move_filtered = filter_move(mov, &filter_options);
                if move_filtered {
                    return None;
                }

                let san_move = San::from_move(&self.chess, *mov);
                chess_clone.play_unchecked(*mov);

                let san_str = san_to_san_plus(&san_move, &chess_clone);

                return Some(san_str);
            })
            .collect();

        legal_moves
    }

    /// # API Discrepancy: chess.js Compatibility
    ///
    /// This implementation differs from chess.js in how it handles the
    /// en passant square in verbose move objects.
    ///
    /// |      Aspect       |           chess.js             |      This Implementation      |
    /// |-------------------|--------------------------------|-------------------------------|
    /// | En passant square | Always included when available | Only included for legal moves |
    #[wasm_bindgen(js_name = "legalMovesVerbose")]
    pub fn legal_moves_verbose(
        &self,
        filter_options: Option<LegalMovesFilterOptions>,
    ) -> Vec<MoveVerbose> {
        let filter_options = unwrap_filter_options(&filter_options);

        let legal_moves: Vec<MoveVerbose> = self
            .chess
            .legal_moves()
            .iter()
            .filter_map(|raw_move| {
                let move_filtered = filter_move(raw_move, &filter_options);
                if move_filtered {
                    return None;
                }

                let move_verbose = verbose_move_from_raw_move(*raw_move, &self.chess);

                Some(move_verbose)
            })
            .collect();

        legal_moves
    }

    #[wasm_bindgen(js_name = "getCastlingRights")]
    pub fn get_castling_rights(&self, color_char: ColorChar) -> CastlingObj {
        let castles_bitboard = &self.chess.castles().castling_rights();

        match color_char {
            ColorChar::W => {
                let queenside = castles_bitboard.contains(Square::A1);
                let kingside = castles_bitboard.contains(Square::H1);

                return CastlingObj {
                    king: Some(kingside),
                    queen: Some(queenside),
                };
            }
            ColorChar::B => {
                let queenside = castles_bitboard.contains(Square::A8);
                let kingside = castles_bitboard.contains(Square::H8);

                return CastlingObj {
                    king: Some(kingside),
                    queen: Some(queenside),
                };
            }
        };
    }

    #[wasm_bindgen(js_name = "zobristHash")]
    pub fn zobrist_hash(&self) -> u64 {
        return self.hash.0;
    }
}

struct FilterOptions {
    pub square: Option<Square>,
    pub piece: Option<Role>,
}

fn unwrap_filter_options(options: &Option<LegalMovesFilterOptions>) -> FilterOptions {
    let filter_square_option: Option<Square> = match options.as_ref() {
        Some(val) => {
            if let Some(square) = &val.from_square {
                Some(square.into())
            } else {
                None
            }
        }
        None => None,
    };

    let filter_piece_option: Option<Role> = match options.as_ref() {
        Some(val) => {
            let p = val.piece.as_ref();

            if let Some(piece_symbol) = p {
                Some(piece_symbol.into())
            } else {
                None
            }
        }
        None => None,
    };

    FilterOptions {
        square: filter_square_option,
        piece: filter_piece_option,
    }
}

fn filter_move(mov: &Move, options: &FilterOptions) -> bool {
    if let Some(filter_square) = options.square {
        let from = mov.from();

        let Some(from) = from else {
            return true;
        };

        if filter_square != from {
            return true;
        }
    }

    if let Some(filter_piece) = options.piece {
        if mov.role() != filter_piece {
            return true;
        }
    }

    false
}
