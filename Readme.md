# Hack Simulator
See http://nand2tetris.org/ for more details.

# Requirements
- Rust nightly - [multirust recommended](https://github.com/brson/multirust)
- SDL2

# Run the interpreter testsuite
```
$ cd interpreter
$ cargo test
```

# Run a testfile
```
$ cargo run --release -j4 -- --runner tests/data/Mult.tst
```

# Run the simulator
```
$ cargo run --release -j4 -- programs/Fill.hack
$ cargo run --release -j4 -- programs/Pong.hack
```

In case of trouble, ensure you try `cargo clean && cargo update`

# License
GPLv2
