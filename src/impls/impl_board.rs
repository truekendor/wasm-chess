use super::*;

#[wasm_bindgen]
impl WasmChess {
    // TODO: check when it returns true
    // it can be on valid position after put() call or on successful put
    pub fn put(&mut self, piece_obj: PieceObj, square: SquareStr) -> bool {
        let piece = piece_obj.to_shakmaty_piece();
        let square = square.to_shakmaty_sq();

        // TODO: rules:
        // no more than TWO kings, but can be less

        // TODO: change to behavior below
        // right not we create editable setup even on only half-valid put() call
        // but we should only do that if the put action was valid all the way through OR
        // if the setup already exist

        // TODO: we can do logic by passing board chain of methods on board, but without
        // knowing which one of the boards it is ?
        // OR we just do  `if _ {} else {}`

        let editable = self.editable.get_or_insert_with(|| EditablePosition {
            setup: Chess::to_setup(&self.chess, EnPassantMode::Legal),
            validated: None,
        });

        let setup = &mut editable.setup;

        // TODO: check if this is always the case
        setup.ep_square = None;

        println!("EP SQUARE IS SOME {:?}", setup.ep_square.is_some());

        let _ = &setup.board.set_piece_at(square, piece);

        let pos = setup
            .clone()
            .position::<Chess>(shakmaty::CastlingMode::Chess960);

        // TODO:
        // update castling rights
        // update setup
        // update ep square

        // TODO:
        // immediatelly try to replace current pos with new one

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
            // TODO: consider changes
            // why even bother with editable setup
            // if we immediately replace self.chess in case of validation?
            editable.validated = Some(validated.clone());
            // TODO:
            self.chess = validated;
            return true;
        }

        editable.validated = None;

        return false;
    }

    fn remove(&mut self, square: SquareStr) -> Option<PieceObj> {
        let square = square.to_shakmaty_sq();

        let editable = match self.editable.as_mut() {
            Some(val) => val,
            None => &mut EditablePosition {
                setup: Chess::to_setup(&self.chess, EnPassantMode::Legal),
                validated: None,
            },
        };

        let setup = &mut editable.setup;
        let piece = setup.board.remove_piece_at(square);

        let pos = setup
            .clone()
            .position::<Chess>(shakmaty::CastlingMode::Chess960);

        editable.validated = pos.ok();

        if let Some(piece) = piece {
            return Some(PieceObj::from_shakmaty_piece(&piece));
        }

        // TODO:
        // update castling rights
        // update setup
        // update ep square

        return None;
    }

    fn clear(&mut self, preserve_headers: Option<PreserveHeaders>) -> () {
        let editable = match self.editable.as_mut() {
            Some(val) => val,
            None => &mut EditablePosition {
                setup: Setup::empty(),
                validated: None,
            },
        };

        editable.setup = Setup::empty();
        editable.validated = None;
    }

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

    // TODO: move to impl_moves ???
    pub(crate) fn make_null_move(&mut self) -> Result<(), String> {
        let editable = self.editable.get_or_insert_with(|| EditablePosition {
            setup: Chess::to_setup(&self.chess, EnPassantMode::Legal),
            validated: None,
        });

        let setup = &mut editable.setup;

        if setup.turn == Color::Black {
            setup.fullmoves = setup
                .fullmoves
                .checked_add(1)
                // Safe: u32::MAX is ~4B moves, far beyond any possible chess game length
                .expect("Increment caused an overflow");
        }

        setup.halfmoves += 1;
        setup.swap_turn();

        let pos = setup
            .clone()
            .position::<Chess>(shakmaty::CastlingMode::Chess960);

        let pos = match pos {
            Ok(val) => Ok(val),
            Err(err) => {
                let result = err
                    .ignore_too_much_material()
                    .or_else(|err| err.ignore_impossible_check())
                    .or_else(|err| err.ignore_invalid_ep_square());

                // result.ok()
                match result {
                    Ok(val) => Ok(val),
                    Err(err) => Err(err.kinds()),
                }
            }
        };

        match pos {
            Ok(val) => {
                editable.validated = Some(val.clone());
                self.chess = val;
                return Ok(());
            }
            Err(err) => {
                return Err(format!("{:#?}", err));
            }
        }
    }

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
            // TODO:
            // why even bother with this if we immediately replace
            self.chess = validated;
        };

        let rights_final = self.get_castling_rights(color);

        let successfully_set = (castling_obj.king.is_none()
            || rights_final.king == castling_obj.king)
            && (castling_obj.queen.is_none() || rights_final.queen == castling_obj.queen);

        return successfully_set;
    }
}
