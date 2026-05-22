use super::*;

#[wasm_bindgen]
impl WasmChess {
    #[wasm_bindgen(js_name = "historySan")]
    pub fn history_san(&self) -> Vec<String> {
        self.history
            .iter()
            .map(|history| {
                let san_move = San::from_move(&history.position_before, history.raw_move);

                san_to_san_plus(&san_move, &history.position_after)
            })
            .collect()
    }

    #[wasm_bindgen(js_name = "historyUci")]
    pub fn history_uci(&self) -> Vec<String> {
        self.history
            .iter()
            .map(|h| {
                let uci_move = h.raw_move.to_uci(shakmaty::CastlingMode::Chess960);

                uci_move.to_string()
            })
            .collect()
    }

    #[wasm_bindgen(js_name = "historyVerbose")]
    pub fn history_verbose(&self) -> Vec<MoveVerbose> {
        let moves_verbose: Vec<MoveVerbose> = self
            .history
            .iter()
            .map(|history_entry| {
                let internal_move = history_entry.raw_move;

                let move_verbose = parsing::verbose_move_from_raw_move(
                    internal_move,
                    &history_entry.position_before,
                );

                move_verbose
            })
            .collect();

        moves_verbose
    }

    /// Returns the FEN string at a specific move index.
    ///
    /// ## Parameters
    /// * `index` - The move index (0-based):
    ///   - `0` - Starting position (before any moves)
    ///   - `1` - Position after first move
    ///   - `2` - Position after second move, etc.
    ///
    /// ## Returns
    /// The FEN string at the requested position:
    ///   - For valid indices (0 through history length), returns the position at that index
    ///   - For indices beyond the history length, returns the last available position
    ///
    /// ## Note
    /// This method never fails - out-of-bounds indices return the final position.
    /// This is useful for UCI-style interfaces where move indices might exceed the game length.
    ///
    /// ## Example
    /// ```
    /// assert_eq!(chess.fen_at(0), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    ///
    /// // After 1.e4
    /// assert_eq!(chess.fen_at(1), "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1");
    ///
    /// // Index beyond game length returns the final position
    /// assert_eq!(chess.fen_at(999), final_position_fen);
    /// ```
    #[wasm_bindgen(js_name = "fenAt")]
    pub fn fen_at(&self, index: usize) -> String {
        if index == 0 {
            if self.history.is_empty() {
                self.fen(None)
            } else {
                self.history[0].fen_before.to_string()
            }
        } else if index <= self.history.len() {
            self.history[index - 1].fen_after.to_string()
        } else {
            // Index beyond history - return last position
            self.history
                .last()
                .map(|last| last.fen_after.to_string())
                .unwrap_or_else(|| self.fen(None))
        }
    }

    // TODO: write test for it
    /// Returns the move at a specific index.
    ///
    /// # Parameters
    /// * `index` - The move index (0-based):
    ///   - `0` - Returns `None` (no move at starting position)
    ///   - `1` - First move played
    ///   - `2` - Second move played, etc.
    ///
    /// # Returns
    /// * `Some(MoveObject)` - The move at the requested index
    /// * `None` - If `index` is 0 or exceeds total moves played
    #[wasm_bindgen(js_name = "moveAt")]
    pub fn move_at(&self, index: usize) -> Option<MoveObject> {
        match index {
            0 => None,
            idx if idx <= self.history.len() => {
                let history_entry = &self.history[idx - 1];
                let internal_move = history_entry.raw_move;
                let promotion = internal_move
                    .promotion()
                    .map(|role| PieceSymbol::from_shakmaty_piece_role(&role));

                let from = internal_move.from()?;
                let to = internal_move.to();

                let from = SquareStr::from_shakmaty_sq(&from);
                let to = SquareStr::from_shakmaty_sq(&to);

                Some(MoveObject {
                    from,
                    to,
                    promotion,
                })
            }
            _ => None,
        }
    }

    /// Returns which side to move at a specific index.
    ///
    /// # Parameters
    /// * `index` - The position index (0-based):
    ///   - `0` - Starting position (White's turn for default starting position)
    ///   - `1` - Turn after first move (Black's turn for default starting position)
    ///   - `2` - Turn after second move, etc.
    ///
    /// # Behavior
    /// * If `index` is within history bounds, returns the side to move at that position
    /// * If `index` overflows the history (exceeds available moves), returns the current side to move
    #[wasm_bindgen(js_name = "sideToMoveAt")]
    pub fn side_to_move_at(&self, index: usize) -> ColorChar {
        if index == 0 {
            let turn = if self.history.is_empty() {
                self.chess.turn()
            } else {
                self.history[0].turn
            };
            return ColorChar::from(&turn);
        }

        if index <= self.history.len() {
            let turn = self.history[index - 1].turn;
            match turn {
                Color::White => ColorChar::B,
                Color::Black => ColorChar::W,
            }
        } else {
            // Index overflows history - return current side to move
            ColorChar::from(&self.chess.turn())
        }
    }

    #[wasm_bindgen(js_name = "moveNumber")]
    pub fn move_number(&self) -> u32 {
        return self.chess.fullmoves().get();
    }

    pub fn length(&self) -> u32 {
        return self.history.len() as u32;
    }
}
