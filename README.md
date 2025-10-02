# trpg-tools

Command‑line utilities for tabletop RPGs (TRPG), written in Rust.
The initial tool is a dice roller that supports NdM expressions (e.g., 3d6) and keeps a persistent history of rolls.

## Overview

- Roll dice using NdM format with a simple CLI.
- View a timestamped history of previous rolls.
- History is saved to a CSV file in your home directory for persistence.

## Requirements

- Rust toolchain with Cargo installed (via https://rustup.rs/)
  - Note: If you encounter compilation issues, update to the latest stable Rust using `rustup update stable`.
- A terminal that supports ANSI colors (for colored output in the history display).

## Setup

Clone and build:

```sh
git clone https://github.com/kinagiyuki/trpg-tools.git
cd trpg-tools
cargo build --release
```

The optimized binary will be at `target/release/trpg-tools` (or `target\release\trpg-tools.exe` on Windows).

You can also run without installing:

```sh
cargo run -- <command>
# examples:
cargo run -- roll 3d6
cargo run -- history
```

## Usage

After building, invoke the binary directly or via your PATH.

- Roll dice:
  ```sh
  trpg-tools roll 3d6
  ```
- Show history:
  ```sh
  trpg-tools history
  ```

Notes:
- The CLI help name is displayed as "rolling-dice" (from clap metadata), but the produced binary is `trpg-tools`.
- History is stored at:
  - Unix/macOS: `~/.trpg-tools/rolling-records.csv`
  - Windows: `%USERPROFILE%\.trpg-tools\rolling-records.csv`

### Convenience alias

You may alias the command to a shorter name, e.g. `tt`:

```sh
alias tt="trpg-tools"
```

## Scripts and commands

Common Cargo commands for this project:
- Build (debug): `cargo build`
- Build (release): `cargo build --release`
- Run: `cargo run -- <args>`
- Tests: `cargo test`
- Lint (if installed): `cargo clippy`
- Format: `cargo fmt`

CI:
GitHub Actions workflow `.github/workflows/rust.yml` builds the project and runs `cargo test` on pushes and pull requests to `main`.

## Project structure

```
trpg-tools/
├─ Cargo.toml
├─ src/
│  ├─ main.rs          # CLI entry point (clap subcommands: roll, history)
│  ├─ menu.rs          # Argument parsing/validation and user-facing actions
│  └─ dice_generator.rs# Dice rolling logic and CSV persistence
├─ README.md
├─ LICENSE             # MIT
└─ .github/
   └─ workflows/
      └─ rust.yml      # CI: build and test
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
