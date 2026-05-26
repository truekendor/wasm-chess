/* @ts-self-types="./chess_wasm.d.ts" */

import * as wasm from "./chess_wasm_bg.wasm";
import { __wbg_set_wasm } from "./chess_wasm_bg.js";
__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    WasmChess, findDivergence, movesToSan, movesToUci
} from "./chess_wasm_bg.js";
