# Puzzle Solver

A Rust program that solves a calendar-based polyomino puzzle: for every valid date in the year (January 1 – December 31), it enumerates all possible ways to place eight hexomino-shaped pieces on an 8×8 grid such that no piece occupies any bit reserved for the month or day.

## How It Works

The board is a single `u64` bitmask representing an 8×8 grid (64 cells). Each cell is one bit — set (`1`) or unset (`0`). The program models:

- **The border** — fixed bits on the board that no piece may overlap.
- **Month bits** — six specific columns reserved for encoding which month (1–12) a date falls in.
- **Day bits** — five specific cells reserved for encoding which day of the month (1–31).

Eight hexomino pieces (6-cell polyominoes) are placed on the remaining free cells:

| Index | Name      | Bitmask         | Bounding Box |
|-------|-----------|-----------------|--------------|
| 0     | P Piece   | `0x0010303`     | 2×3          |
| 1     | Corner    | `0x0010107`     | 3×3          |
| 2     | U Piece   | `0x0030103`     | 2×3          |
| 3     | L Piece   | `0x01010103`    | 2×4          |
| 4     | Z Piece   | `0x02030101`    | 2×4          |
| 5     | Block     | `0x0030303`     | 2×3          |
| 6     | S Piece   | `0x0060203`     | 3×3          |
| 7     | Y Piece   | `0x01030101`    | 2×4          |

Each piece is rotated (90° clockwise) and mirrored to generate all valid orientations. The solver then recursively places each piece on the board, skipping positions that overlap with already-placed pieces or reserved bits, until all eight pieces are positioned — producing one solution.

For every valid date, the program filters those solutions: a solution is valid for a given date only if **none** of its placed pieces cover any month bit *and* none cover any day bit. Each valid solution is written to `solutions/<Month>/<Day>.txt` as eight space-separated hex values (one per piece).

## Project Structure

```
puzzle-solver/
├── src/
│   ├── main.rs      # Solver logic, lookup tables, recursive backtracking
│   └── piece.rs     # Piece data structure with rotate/mirror/variant generation
├── solutions/       # Output directory — one .txt per calendar date
│   ├── January/
│   │   └── 1.txt    # All valid placements for Jan 1
│   ├── February/
│   ├── ...
│   └── December/
├── Cargo.toml       # Package config (Rust edition 2021, no external deps)
├── Cargo.lock
└── .gitignore
```

## Building & Running

Requires [Rust](https://rust-lang.org/) (edition 2021, no third-party dependencies).

```bash
# Build in release mode
cargo build --release

# Run the solver
cargo run --release
```

The program prints three timing/count stats:

- **Number of iterations** — total placement attempts across all recursive branches.
- **Number of solutions + invalids** — raw placements found (before date filtering).
- **Time elapsed for search** — how long the backtracking took.
- **Number of valid solutions** — placements that fit a specific date.
- **Time elapsed for data writing** — how long it took to write output files.

## Output Format

Each `solutions/<Month>/<Day>.txt` contains one solution per line. Each line has eight space-separated 16-character hexadecimal values representing the bitmasks of the eight placed pieces in order:

```
0018380000000000 00000002020E0000 E0A0000000000000 ...
0C1C000000000000 00000002020E0000 0000382800000000 ...
...
```

## Algorithm Complexity

The solver uses recursive backtracking with bitmask overlap checks. For each of the eight pieces, it tries every orientation × every board position, pruning branches where a piece would overlap an already-placed one or reserved bits. The date-filtering pass then runs over all raw solutions to keep only those compatible with each calendar date.

## License

This project is shared as-is for reference and experimentation purposes.
