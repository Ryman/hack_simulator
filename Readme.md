# Hack Simulator
See http://nand2tetris.org/ for more details.

# Requirements
- Rust 1.21.0 (https://rustup.rs/ recommended)
- SDL2

# Run the interpreter testsuite
```
$ cd interpreter
$ cargo test
```

# Run a testfile
```
$ cargo run --release -- --runner interpreter/tests/data/Mult.tst
```

# Run the simulator
```
$ cargo run --release -- programs/Fill.hack
$ cargo run --release -- programs/Pong.hack
```

In case of trouble, ensure you try `cargo clean && cargo update`

# License
GPLv2
