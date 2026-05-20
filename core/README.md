# minesweeper_core

Core game logic for Minesweeper. Handles board generation, cell state, mine placement, and win/loss conditions, with no UI or platform dependencies.

Used by [minesweeper-1v1](https://github.com/etoledom/minesweeper-1v1), a real-time multiplayer Minesweeper game.

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
minesweeper_core = "0.2.0"
```

To enable serialization with `serde`:

```toml
[dependencies]
minesweeper_core = { version = "0.2.0", features = ["serde"] }
```

## Example

```rust
use minesweeper_core::{Game, Difficulty, Point};

// Create a new game
let mut game = Game::new(Difficulty::Easy);

// Reveal a cell
game.selected_at(Point { x: 3, y: 4 });

// Flag/unflag a cell
game.toggle_flagged(Point { x: 0, y: 0 });

// Check game state
if game.is_game_over() {
    println!("Hit a mine!");
} else if game.is_win() {
    println!("All mines found!");
}

// Inspect a cell
if let Some(cell) = game.board.cell_at(Point { x: 3, y: 4 }) {
    println!("Mine: {}, Cleared: {}, Flagged: {}",
        cell.is_mine(),
        cell.is_cleared(),
        cell.is_flagged()
    );
}
```

## Running the example

A visual Minesweeper game is included as an example, built with [piston_window](https://github.com/PistonDevelopers/piston_window):

```sh
cargo run --example visual
```

## License

MIT
