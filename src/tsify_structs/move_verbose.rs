use serde::{Deserialize, Serialize};

use crate::tsify_structs::{SquareStr, others::ColorChar};

#[derive(tsify::Tsify, Serialize, Deserialize, Debug, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct MoveVerbose {
    pub from: SquareStr,
    pub to: SquareStr,

    /// fen before move is played
    pub before: String,
    /// fen after move is played
    pub after: String,

    pub color: ColorChar,
    pub piece: String,
    pub captured: Option<String>,

    pub promotion: Option<String>,

    pub san: String,
    pub lan: String,

    pub is_en_passant: bool,
    // for now we do not distinguish between kingside and queenside castle
    pub is_castle: bool,
    // TODO: add `is_kingside_castle` and `is_queenside_castle`
}
