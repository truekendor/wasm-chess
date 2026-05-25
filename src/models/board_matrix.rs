use serde::{Deserialize, Serialize};

use crate::models::utils::SquareInfo;
pub type BoardObjRow = Vec<Option<SquareInfo>>;
pub type BoardMatrix = Vec<BoardObjRow>;

#[derive(tsify::Tsify, Serialize, Deserialize, PartialEq, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(transparent)]
pub struct BoardState(
    #[tsify(type = "Array<Array<SquareInfoObj | null | undefined>>")] pub BoardMatrix,
);
