use serde::{Deserialize, Serialize};

use crate::tsify_structs::others::SquareInfoObj;

pub type BoardMatrixRow = Vec<Option<SquareInfoObj>>;
pub type BoardMatrix = Vec<BoardMatrixRow>;

#[derive(tsify::Tsify, Serialize, Deserialize, Debug, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct BoardMatrixReturnObj {
    #[tsify(type = "Array<Array<SquareInfoObj | null | undefined>>")]
    pub board_matrix: BoardMatrix,
}
