use ordermap::OrderMap;
use serde::{Deserialize, Serialize};
use strum::Display;

use crate::models::{ColorChar, PieceSymbol, SquareStr};

#[derive(tsify::Tsify, Serialize, Deserialize, PartialEq, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(transparent)]
pub struct HeadersObj(pub OrderMap<String, String>);

#[derive(tsify::Tsify, Serialize, Deserialize, Debug, PartialEq, Display)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub enum SquareColor {
    Light,
    Dark,
}

#[derive(tsify::Tsify, Serialize, Deserialize, PartialEq, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct MoveObject {
    pub from: SquareStr,
    pub to: SquareStr,
    #[tsify(optional)]
    pub promotion: Option<PieceSymbol>,
}

#[derive(tsify::Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct MoveFromSquares {
    pub from: SquareStr,
    pub to: SquareStr,
}

#[derive(tsify::Tsify, Serialize, Deserialize, Debug, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct CommentsObj {
    pub fen: String,
    #[tsify(optional)]
    pub comment: Option<String>,
    #[tsify(optional)]
    pub suffix_annotation: Option<String>,
    pub nags: Vec<String>,
}

#[derive(tsify::Tsify, Serialize, Deserialize, Debug, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct PrunedCommentsObj {
    pub fen: String,
    pub comment: String,
}

#[derive(tsify::Tsify, Serialize, Deserialize, Debug, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct CastlingObj {
    pub king: Option<bool>,
    pub queen: Option<bool>,
}

// this is like a custom result
#[derive(tsify::Tsify, Serialize, Deserialize, Debug, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct OkOrError {
    pub ok: bool,
    pub err: Option<String>,
}

#[derive(tsify::Tsify, Serialize, Deserialize, Debug, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct SquareInfoObj {
    pub square: SquareStr,
    pub r#type: PieceSymbol,
    pub color: ColorChar,
}

#[derive(tsify::Tsify, Serialize, Deserialize, Debug, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct PreserveHeaders {
    pub preserve_headers: bool,
}

#[derive(tsify::Tsify, Serialize, Deserialize, Debug, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct PGNOptions {
    /// Maximum line width for wrapping PGN moves (default: 80)
    pub max_width: Option<usize>,
    /// Custom newline sequence (default: "\r\n" per PGN spec)
    pub newline: Option<String>,
}

#[derive(tsify::Tsify, Serialize, Deserialize, Debug, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct LegalMovesFilterOptions {
    pub from_square: Option<SquareStr>,
    pub piece: Option<PieceSymbol>,
}
