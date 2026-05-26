# todo

---
## Build

To build project from source run

`wasm-pack build --target web` or for bundlers `wasm-pack build --target bundler --out-dir builds`

---

# Acknowledgements

## chess.js

This project was heavily inspired by

* chess.js API design
* developer experience of chess.js
* tests

Most of the tests in this repo are directly taken from chess.js test suite

Thanks to all chess.js contributors for creating and maintaining one of the most approachable and reliable libraries in the JS ecosystem

Repository:

* [https://github.com/jhlywa/chess.js](https://github.com/jhlywa/chess.js)

---

## shakmaty

This project is fundamentally powered by the incredible `shakmaty` project.

Core chess logic, move legality, SAN handling, position management and more
are built on top of shakmaty and shakmaty pgn-reader crates. Whitout shakmaty this project would not exist

Thanks to shakmaty maintainers and all contributors for building one of the strongest chess libraries available in Rust

Repositories:

* [https://github.com/niklasf/shakmaty](https://github.com/niklasf/shakmaty)
* [https://github.com/niklasf/shakmaty/tree/master/pgn-reader](https://github.com/niklasf/shakmaty/tree/master/pgn-reader)
