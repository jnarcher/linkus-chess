# linkus-chess

Welcome to the **Linkus Engine**! This project is a fast, efficient, and modular chess engine implemented in Rust using *bitboard* techniques for board representation.

---

## Overview  

This chess engine is built with **bitboards**, a compact representation of a chess board that uses 64-bit integers for efficient move generation and board operations. Written in **Rust**, the engine leverages safety and performance optimizations.

This project is designed to be:
- **Fast**: Optimized with bit-level operations.
- **Lightweight**: Minimal dependencies and efficient memory usage.

---

## Features

- **Bitboard Representation**: Efficient and compact board representation.  
- **Move Generation**: Fast legal move generation with bitwise operations.  
- **Search Algorithms**: Negamax recursive search with Alpha-Beta pruning, iterative deepening, and quiescence search.  
- **Performance Testing Results**: Generates all moves possible at a certain depth in a certain position.
- **UCI Protocol Support**: Limited compatibility with Universal Chess Interface (UCI) for integration with chess GUIs.

---

## Installation

1. Ensure you have **Rust** installed. If not, install it via [rustup](https://rustup.rs/):

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2.	Clone the repository:

    ```bash
    git clone https://github.com/jnarcher/linkus-chess.git
    cd linkus-chess
    ```

3.	Build the project using cargo:

    ```bash

    cargo build --release
    ```

## Usage

### Command-Line Interface (CLI)

Run the engine directly in the terminal:

cargo run --release

### UCI Integration

The engine supports a limited version of the UCI protocol. Below are a list of supported commands:

- `d` - Displays current board state.
- `isready` - Checks if engine is ready to receive input.
- `ucinewgame` - Resets the board to the starting position.
- `position fen <fen string>` - Sets the position of the board using a FEN string.
- `position startpos` - Sets the position of the board to the starting position. (Same thing as `position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1`)
- `go perft <depth>` - Generates possible moves at a given `depth`.
- `go depth <depth>` - Generates the best move at a given depth for the current position.
- `uci` - Responds with the engine name and author name.
- `quit` - Quits the program.

#### Making moves

Currently there is a limited way of making moves against the chess engine. Since, per UCI standards, the engine is stateless, you must enter in either the FEN string of the current board state each time you want to generate the best move, or input the full move list.
This can be done with the `position [fen <FEN string> | startpos] moves [move list]` command where `move list` is a space separated list of the moves you want to enact on the board.
An example is like so:

```bash
position startpos moves e2e4 e4e5
d

8  r n b q k b n r
7  p p p p . p p p
6  . . . . . . . .
5  . . . . p . . .
4  . . . . P . . .
3  . . . . . . . .
2  P P P P . P P P
1  R N B Q K B N R

   a b c d e f g h

to_move = WHITE
castling_rights = 1111
en_passant = e6
```

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Acknowledgements

Special thanks to the Rust community and resources like:
	•	The Chess Programming Wiki
	•	Rust Programming Language Book



