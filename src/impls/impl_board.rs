use super::*;

#[wasm_bindgen]
impl WasmChess {
    /// Places a piece on the board at the given square.
    ///
    /// # JavaScript Example
    /// ```js
    /// const success = chess.put({ kind: 'queen', color: 'white' }, 'e2');
    /// ```
    ///
    /// # Returns
    /// - `true` if the position is valid after placement
    /// - `false` if the placement would create an invalid position (e.g., illegal castling rights)
    ///
    /// # Notes
    /// - En passant square is cleared when placing pieces manually
    /// - Invalid positions are rejected and the board remains unchanged
    pub fn put(&mut self, piece_obj: PieceObj, square: SquareStr) -> bool {
        let piece = piece_obj.to_shakmaty_piece();
        let square = square.to_shakmaty_sq();

        let editable = self.editable.get_or_insert_with(|| EditablePosition {
            setup: Chess::to_setup(&self.chess, EnPassantMode::Legal),
            validated: None,
        });

        let setup = &mut editable.setup;
        setup.ep_square = None;

        let _ = &setup.board.set_piece_at(square, piece);

        let pos = setup
            .clone()
            .position::<Chess>(shakmaty::CastlingMode::Chess960);

        let pos = match pos {
            Ok(val) => Some(val),
            Err(err) => {
                let result = err
                    .ignore_too_much_material()
                    .or_else(|err| err.ignore_invalid_castling_rights())
                    .or_else(|err| err.ignore_invalid_ep_square());

                result.ok()
            }
        };

        if let Some(validated) = pos {
            editable.validated = Some(validated.clone());
            self.chess = validated;
            return true;
        }

        editable.validated = None;

        return false;
    }

    /// Changes which player moves next.
    ///
    /// # JavaScript Example
    /// ```js
    /// try {
    ///   const changed = chess.setTurn('b'); // Switch to black
    ///   console.log(changed ? "Turn changed" : "Already black's turn");
    /// } catch (err) {
    ///   console.error("Invalid position after turn change:", err);
    /// }
    /// ```
    ///
    /// # Parameters
    /// - `color`: `'w'` for white, `'b'` for black
    ///
    /// # Returns
    /// - `Ok(true)` - Turn was successfully changed
    /// - `Ok(false)` - Already that player's turn (no change)
    /// - `Err(string)` - Position became invalid after turn change (contains error details)
    #[wasm_bindgen(js_name = "setTurn")]
    pub fn set_turn(&mut self, color: ColorChar) -> Result<bool, String> {
        let turn = self.turn();

        if color == turn {
            return Ok(false);
        }

        let editable = self.editable.get_or_insert_with(|| EditablePosition {
            setup: Chess::to_setup(&self.chess, EnPassantMode::Legal),
            validated: None,
        });
        let setup = &mut editable.setup;
        setup.swap_turn();

        let pos = setup
            .clone()
            .position::<Chess>(shakmaty::CastlingMode::Chess960);

        let pos = match pos {
            Ok(val) => Ok(val),
            Err(err) => {
                let result = err
                    .ignore_too_much_material()
                    .or_else(|err| err.ignore_invalid_castling_rights())
                    .or_else(|err| err.ignore_invalid_ep_square());

                let res = result.map_err(|err| format!("{:#?}", err.kinds()));

                res
            }
        };

        match pos {
            Ok(val) => {
                editable.validated = Some(val.clone());
                self.chess = val;
                return Ok(true);
            }
            Err(err) => {
                return Err(format!("{:#?}", err));
            }
        }
    }

    /// Updates castling rights for a specific color.
    ///
    /// # JavaScript Example
    /// ```js
    /// // Disable black's kingside castling only
    /// chess.setCastlingRights('b', { king: false });
    ///
    /// // Enable both kingside and queenside for white
    /// chess.setCastlingRights('w', { king: true, queen: true });
    ///
    /// // Leave queenside unchanged, remove kingside
    /// chess.setCastlingRights('w', { king: false });
    /// ```
    ///
    /// # Parameters
    /// - `color`: `'w'` or `'b'`
    /// - `castling_obj`: Object with optional boolean fields
    ///   - `king` - `true` to enable, `false` to disable, `undefined` to leave unchanged
    ///   - `queen` - Same as above
    ///
    /// # Returns
    /// - `true` if the requested rights were successfully applied
    /// - `false` if the operation didn't take effect (e.g., invalid position rejected)
    ///
    /// # Notes
    /// - Invalid castling configurations (e.g., rights on empty files) are automatically filtered out
    /// - The method returns `false` if the final rights don't match the request
    #[wasm_bindgen(js_name = "setCastlingRights")]
    pub fn set_castling_rights(&mut self, color: ColorChar, castling_obj: CastlingObj) -> bool {
        let editable = match self.editable.as_mut() {
            Some(val) => val,
            None => &mut EditablePosition {
                setup: Chess::to_setup(&self.chess, EnPassantMode::Legal),
                validated: None,
            },
        };

        let setup = &mut editable.setup;

        let (kingside_sq, queenside_sq) = match color {
            ColorChar::W => (Square::H1, Square::A1),
            ColorChar::B => (Square::H8, Square::A8),
        };

        if castling_obj.king == Some(true) {
            setup.castling_rights.add(kingside_sq);
        } else if castling_obj.king == Some(false) {
            let _ = setup.castling_rights.remove(kingside_sq);
        }

        if castling_obj.queen == Some(true) {
            setup.castling_rights.add(queenside_sq);
        } else if castling_obj.queen == Some(false) {
            let _ = setup.castling_rights.remove(queenside_sq);
        }

        let chess_temp = Chess::from_setup(setup.clone(), shakmaty::CastlingMode::Chess960);

        let pos: Option<Chess> = match chess_temp {
            Ok(val) => Some(val),
            Err(err) => {
                let result = err.ignore_invalid_castling_rights();

                result.ok()
            }
        };

        if let Some(validated) = pos {
            editable.validated = Some(validated.clone());
            self.chess = validated;
        };

        let rights_final = self.get_castling_rights(color);

        let successfully_set = (castling_obj.king.is_none()
            || rights_final.king == castling_obj.king)
            && (castling_obj.queen.is_none() || rights_final.queen == castling_obj.queen);

        return successfully_set;
    }
}
