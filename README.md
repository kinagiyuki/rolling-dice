# trpg-tools

Command-line tools for playing TRPG written in Rust

## Installation

To install the `trpg-tools` program, you need to have Rust 1.88.0 or later installed on your system.
You can then build and run the project using Cargo:

```shell
sh git clone https://github.com/kinagiyuki/trpg-tools.git
cd trpg-tools
cargo build --release
```
Then the built binary will be `./target/release`

## Usage

The `trpg-tools` program is a command-line interface (CLI) tool that allows you to roll dice according to the NdM
format (e.g., 3d6).
It also provides functionality to display the roll history.

### Commands

#### Roll

To roll dice, use the `roll` subcommand followed by the NdM expression:

```shell
trpg-tools roll 3d6
```

This will output the result of rolling three six-sided dice.

#### History

To view the roll history, use the `history` subcommand:

```shell
trpg-tools history
```

This will display all previous roll records stored in the CSV file located at `~/.trpg-tools/rolling-records.csv`.

### For convenience

It is recommended to alias `trpg-tools` to its shorten form `tt`

## Features

- **Rolling Dice**: Supports rolling dice using the NdM format (e.g., 3d6).
- **Roll History**: Keeps a record of all rolls and displays them upon request.
- **CSV Storage**: Stores roll records in a CSV file for persistence.

## Contributing

Contributions are welcome! Feel free to open issues, create pull requests, or submit feature requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
