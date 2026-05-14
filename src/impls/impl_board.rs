use super::*;

#[wasm_bindgen]
impl WasmChess {
    fn put(&mut self, piece_obj: PieceObj, square: SquareStr) -> bool {
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

        let editable = match self.editable.as_mut() {
            Some(val) => val,
            None => &mut EditablePosition {
                setup: Chess::to_setup(&self.chess, EnPassantMode::Legal),
                validated: None,
            },
        };

        let setup = &mut editable.setup;
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

        if let Some(validated) = pos.ok() {
            editable.validated = Some(validated.clone());
            // TODO:
            // why even bother with this if we immediately replace
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

        if let Some(p) = piece {
            return Some(PieceObj::from_shakmaty_piece(&p));
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
                setup: Chess::to_setup(&self.chess, EnPassantMode::Legal),
                validated: None,
            },
        };

        let empty_setup = Setup::empty();

        todo!()
    }

    fn set_turn(&mut self, color: ColorChar) -> bool {
        todo!()
    }

    fn set_castling_rights(&mut self, color: ColorChar, castling_obj: CastlingObj) -> bool {
        todo!()
    }

    fn set_en_passant_square() {
        //
    }
}
