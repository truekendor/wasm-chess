use shakmaty::{Chess, fen::Fen};

pub fn pos_from_fen_with_recovery(fen: &Fen) -> Result<Chess, String> {
    match fen.clone().into_position(shakmaty::CastlingMode::Chess960) {
        Ok(val) => Ok(val),
        Err(err) => {
            let result = err
                .ignore_too_much_material()
                .or_else(|err| err.ignore_invalid_castling_rights())
                .or_else(|err| err.ignore_invalid_ep_square())
                .map_err(|err| {
                    return format!("Invalid position: {:#?}\nFEN: {fen}", err);
                })?;

            Ok(result)
        }
    }
}
