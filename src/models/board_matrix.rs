use serde::{Deserialize, Serialize};

use crate::models::utils::SquareInfoObj;
pub type BoardObjRow = Vec<Option<SquareInfoObj>>;
pub type BoardMatrix = Vec<BoardObjRow>;

#[derive(tsify::Tsify, Serialize, Deserialize, Debug, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct BoardState {
    #[tsify(type = "Array<Array<SquareInfoObj | null | undefined>>")]
    pub board_matrix: BoardMatrix,
}
