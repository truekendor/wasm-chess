use std::str::FromStr;

use serde::{Deserialize, Serialize};
use shakmaty::Role;

#[derive(tsify::Tsify, Serialize, Deserialize, Debug, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub enum PieceSymbol {
    P,
    N,
    B,
    R,
    Q,
    K,
}

impl PieceSymbol {
    pub fn as_str(&self) -> &'static str {
        match self {
            PieceSymbol::P => "p",
            PieceSymbol::N => "n",
            PieceSymbol::B => "b",
            PieceSymbol::R => "r",
            PieceSymbol::Q => "q",
            PieceSymbol::K => "k",
        }
    }
}

impl From<&shakmaty::Piece> for PieceSymbol {
    fn from(piece: &shakmaty::Piece) -> Self {
        match piece.role {
            shakmaty::Role::Pawn => PieceSymbol::P,
            shakmaty::Role::Knight => PieceSymbol::N,
            shakmaty::Role::Bishop => PieceSymbol::B,
            shakmaty::Role::Rook => PieceSymbol::R,
            shakmaty::Role::Queen => PieceSymbol::Q,
            shakmaty::Role::King => PieceSymbol::K,
        }
    }
}

impl From<&shakmaty::Role> for PieceSymbol {
    fn from(role: &shakmaty::Role) -> Self {
        match role {
            Role::Pawn => PieceSymbol::P,
            Role::Knight => PieceSymbol::N,
            Role::Bishop => PieceSymbol::B,
            Role::Rook => PieceSymbol::R,
            Role::Queen => PieceSymbol::Q,
            Role::King => PieceSymbol::K,
        }
    }
}

impl From<&PieceSymbol> for shakmaty::Role {
    fn from(val: &PieceSymbol) -> Self {
        match val {
            PieceSymbol::P => Role::Pawn,
            PieceSymbol::N => Role::Knight,
            PieceSymbol::B => Role::Bishop,
            PieceSymbol::R => Role::Rook,
            PieceSymbol::Q => Role::Queen,
            PieceSymbol::K => Role::King,
        }
    }
}

impl FromStr for PieceSymbol {
    type Err = String;

    fn from_str(str: &str) -> Result<PieceSymbol, Self::Err> {
        match str {
            "p" | "P" => Ok(PieceSymbol::P),
            "n" | "N" => Ok(PieceSymbol::N),
            "b" | "B" => Ok(PieceSymbol::B),
            "r" | "R" => Ok(PieceSymbol::R),
            "q" | "Q" => Ok(PieceSymbol::Q),
            "k" | "K" => Ok(PieceSymbol::K),
            _ => Err(format!("Unable to parse string \"{}\" as PieceSymbol", str)),
        }
    }
}
