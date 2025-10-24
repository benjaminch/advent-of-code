# advent-of-code

This repository holds my solutions for [Advent-of-Code](https://adventofcode.com/).

## Project Structure

This project uses a Rust workspace structure with:
- **One workspace member per year** (not per day) for faster builds
- **Shared utilities module** (`utils/`) for common functions used across years
- **Unique binary names** to avoid collisions (e.g., `2024_day01`, `2019_day05`)

```
advent-of-code/
├── Cargo.toml              # Root workspace
├── utils/                  # Shared utilities across all years
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs          # Common functions: read_input(), read_lines(), etc.
└── years/
    ├── 2018/               # 2018 solutions (days 1-3)
    │   ├── Cargo.toml
    │   ├── inputs/         # Input files
    │   └── src/
    │       ├── lib.rs      # Year-specific code + re-exports utils
    │       ├── dayXX.rs    # Day modules
    │       └── bin/        # Binaries (one per day)
    ├── 2019/               # 2019 solutions (days 1-11) + intcode library
    ├── 2020/               # 2020 solutions (days 1-10)
    ├── 2021/               # 2021 solutions (days 1-3)
    └── 2024/               # 2024 solutions (days 1-2)
```

## Running Solutions

```bash
# Run a specific day (from year directory)
cd years/2024
cargo run --bin 2024_day01

# Run from root directory
cargo run -p aoc_2024 --bin 2024_day01

# Build all solutions
cargo build --workspace

# Run tests for a specific year
cargo test -p aoc_2024

# Run all tests
cargo test --workspace
```

## Shared Utilities

The `utils/` module provides common functions available to all years:

```rust
use aoc_utils::*;

// Read input file (from inputs/dayXX.txt)
let input = read_input(1);

// Read as lines
let lines = read_lines(1);

// Parse lines into a type
let numbers: Vec<i32> = parse_lines(1);

// More specialized functions
let groups = read_groups(1);        // Split by blank lines
let csv: Vec<i32> = read_csv(1);    // Parse CSV
let grid = read_char_grid(1);       // 2D character grid
let digits = read_digit_grid(1);    // 2D digit grid
```

## Adding a New Year

See [NEW_YEAR_SETUP_GUIDE.md](NEW_YEAR_SETUP_GUIDE.md) for step-by-step instructions.

Quick version:

```bash
mkdir -p years/2025/{src/bin,inputs}
cp years/2024/Cargo.toml years/2025/
cp years/2024/src/lib.rs years/2025/src/
# Update package name in Cargo.toml
# Add "years/2025" to root Cargo.toml workspace members
```

## Documentation

- **[NEW_YEAR_SETUP_GUIDE.md](NEW_YEAR_SETUP_GUIDE.md)** - How to add new years/days

## Features

✅ Cargo workspace with one package per year
✅ Shared utilities module for common functions
✅ Consistent structure across all years
✅ Unique binary names (no collisions)
✅ Input files in predictable locations
✅ Inline tests with `#[cfg(test)]`
✅ All solutions build and test successfully
