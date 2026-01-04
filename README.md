# Mijnlieff solver

This program solves Andy Hopwood’s board game [Mijnlieff](https://www.hopwoodgames.com/shop-1/mijnlieff). For more information, see [my website](https://iamkate.com/data/mijnlieff/).

## Usage

After [installing Rust](https://www.rust-lang.org/tools/install), compile and run the program with:

```bash
cargo run --release
```

After some time (about a minute on my 2022 MacBook Air), it will output the following:

```
Analysed 1132845697 games in 52 seconds
Mijnlieff is a win for the second player with perfect play
```

## Tests

All functions are tested. Compile and run the tests with:

```bash
cargo test
```

## Documentation

All structs, functions, and constants are documented. Build and open the documentation with:

```bash
cargo doc --open
```

## Alternative boards

This program solves Mijnlieff on the standard board. To solve alternative boards, change the following constants:

- `Board::LINES`, which is an array of bit fields representing the possible lines of three squares. Note that the length of this array is hard-coded in `Board::calculate_scores()` as otherwise the compiler will object to evaluating the function at compile time in order to build the `Board::SCORES` constant.
- `Game::INITIAL_UNAVAILABLE`, which is a bit field representing the initially unavailable squares. As well as marking sqaures as unavailable if they are prohibited by the rules (such as the central squares on the standard board), the solver will run much more quickly if squares which are rotations or reflections of other squares are marked as unavilable.
- `Tile::MOVES`, which is an array of arrays of bit fields representing the squares unavailable after a move, indexed by tile discriminant and the square in which the tile was played.

In addition, changing `Solver::SQUARES_PREFERENCE` and `Solver::TILES_PREFERENCE` can lead to the solution being found much more quickly. For example, the `Solver::SQUARES_PREFERENCE` value that has been optimised for the standard board leads to the solution being found approximately 50 times more quickly than trying the squares in numerical order.

Note that the `Board` struct implements the `Display` trait to pretty-print the board, making it easier to verify that the bit fields have been entered correctly. For example, running:

```rust
println!("{}", Board::from(0b_1001_0100_0010_1111));
```

…will produce this output:

```
■ ■ ■ ■
□ ■ □ □
□ □ ■ □
■ □ □ ■
```
