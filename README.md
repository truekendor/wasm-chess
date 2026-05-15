# TODO: change repo name to Chess.WASM or wasm-chess



# Development

Build:

```bash
wasm-pack build --target web
```

Run tests:

```bash
cargo test
```


---


## Direct Board Manipulation

Some low-level board editing APIs commonly found in chess.js are intentionally not yet implemented:

* `clear()`
* `put()`
* `remove()`

These features are planned for a future release once a clean and well-defined API design is finalized.

---

# Acknowledgements

## chess.js

This project was heavily inspired by:

* the chess.js API design
* the developer experience of chess.js
* chess.js test suite

Many tests were ported and adapted from the chess.js repository to Rust in order to verify compatibility and behavior.

Huge thanks to the chess.js maintainers and contributors for creating one of the most approachable chess libraries in the JavaScript ecosystem.

Repository:

* [https://github.com/jhlywa/chess.js](https://github.com/jhlywa/chess.js)

---

## shakmaty

This project is fundamentally powered by the incredible `shakmaty` project.

Core chess logic, move legality, SAN handling, position management and more
are built on top of:

* shakmaty
* shakmaty pgn-reader

Without shakmaty this project would not exist.

Thanks to shakmaty maintainers and all contributors for building one of the strongest chess libraries available in Rust

Repositories:

* [https://github.com/niklasf/shakmaty](https://github.com/niklasf/shakmaty)
* [https://github.com/niklasf/shakmaty/tree/master/pgn-reader](https://github.com/niklasf/shakmaty/tree/master/pgn-reader)

---
