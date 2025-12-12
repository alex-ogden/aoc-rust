# aoc-rust

A repo to track my advent of code attempts (specifically the ones in Rust)

### How to run:

A simple `cargo run` from the root of the repo should do it. Or you can do a `cargo build` and run the binary from `targets/debug/aoc-rust` (or `targets/release/aoc-rust` if you specified `--release` in your build command)
Running the individual parts/days/months can be done using the following syntax:

```bash
cargo run 2025 5 1  # Runs 2025 day 5 part 1
cargo run 2022 5    # Runs 2022 day 5 part 1 and 2
cargo run 2025      # Runs all 2025 solutions, including part 1 and 2
```

Some days also have multi-threaded solutions or visualisations, see `main.rs` for how to run these (usually appending `vis` or `mt` after specifying day/part)

### Goals/Contraints:

- Ensure all solutions complete in 1 second or less on moderate hardware (when built with --release)
- Ensure only the standard library is used (unless it's a seperate visualisation/multi-threaded/etc... solution
- Make code as readable and idiomatic as possible
- Googling is allowed (obviously)
