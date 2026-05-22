use serde::{Deserialize, Serialize};

#[derive(tsify::Tsify, Serialize, Deserialize, Debug, PartialEq, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub enum ColorChar {
    W,
    B,
}

impl From<&shakmaty::Color> for ColorChar {
    fn from(color: &shakmaty::Color) -> Self {
        match color {
            shakmaty::Color::White => ColorChar::W,
            shakmaty::Color::Black => ColorChar::B,
        }
    }
}

impl From<&ColorChar> for shakmaty::Color {
    fn from(value: &ColorChar) -> Self {
        match value {
            ColorChar::W => shakmaty::Color::White,
            ColorChar::B => shakmaty::Color::Black,
        }
    }
}
