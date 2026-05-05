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
    /// returns true if the move is a two-square pawn move
    pub is_big_pawn: bool,

    pub is_castle: bool,
    pub is_kingside_castle: bool,
    pub is_queenside_castle: bool,
}
