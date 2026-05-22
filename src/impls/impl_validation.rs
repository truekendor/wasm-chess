use super::*;

#[wasm_bindgen]
impl WasmChess {
    #[wasm_bindgen(js_name = "validateFen")]
    pub fn validate_fen(&self, fen: String) -> OkOrError {
        match fen.parse::<Fen>() {
            Ok(_) => OkOrError {
                ok: true,
                err: None,
            },
            Err(err) => OkOrError {
                ok: false,
                err: Some(err.to_string()),
            },
        }
    }
}
