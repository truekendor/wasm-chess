use super::*;

#[wasm_bindgen]
impl WasmChess {
    pub fn ascii(&self) -> String {
        ascii::from_board(&self.chess.board())
    }
}
