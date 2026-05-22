use serde::{Deserialize, Serialize};
use shakmaty::{Color, Piece};

use crate::models::{ColorChar, PieceSymbol};

#[derive(tsify::Tsify, Serialize, Deserialize, Debug, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PieceObj {
    pub r#type: PieceSymbol,
    pub color: ColorChar,
}

impl From<&shakmaty::Piece> for PieceObj {
    fn from(value: &shakmaty::Piece) -> Self {
        PieceObj {
            color: match value.color {
                Color::White => ColorChar::W,
                Color::Black => ColorChar::B,
            },
            r#type: PieceSymbol::from_shakmaty_piece_role(&value.role),
        }
    }
}

impl From<&PieceObj> for shakmaty::Piece {
    fn from(value: &PieceObj) -> Piece {
        Piece {
            color: match value.color {
                ColorChar::W => Color::White,
                ColorChar::B => Color::Black,
            },
            role: match value.r#type {
                PieceSymbol::P => shakmaty::Role::Pawn,
                PieceSymbol::N => shakmaty::Role::Knight,
                PieceSymbol::B => shakmaty::Role::Bishop,
                PieceSymbol::R => shakmaty::Role::Rook,
                PieceSymbol::Q => shakmaty::Role::Queen,
                PieceSymbol::K => shakmaty::Role::King,
            },
        }
    }
}
