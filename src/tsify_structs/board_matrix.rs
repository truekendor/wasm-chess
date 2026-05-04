use serde::{Deserialize, Serialize};

use crate::tsify_structs::others::SquareInfoObj;

pub type BoardMatrixRow = Vec<Option<SquareInfoObj>>;
pub type BoardMatrix = Vec<BoardMatrixRow>;

#[derive(tsify::Tsify, Serialize, Deserialize, Debug, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct BoardMatrixReturnObj {
    pub board_matrix: BoardMatrix,
}
