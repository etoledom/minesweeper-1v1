# Minesweeper 1v1


<img width="1672" height="941" alt="minesboomer" src="https://github.com/user-attachments/assets/079a701e-81e4-4deb-90ed-5fc3a173605b" />

A real-time multiplayer Minesweeper game where two players compete on the same board. Built in Rust, from game logic to server to browser client.

**[Try it live](https://minesweeper.fly.dev/)**

## The Game

Two players join a game and play on the same Minesweeper board simultaneously. The goal is to find as many mines as possible. A square without a mine will give the turn to the opponent.

### Rules

- Both players see the same board in real time
- Click a cell to reveal it.
- If it's a mine is a point for you. It will be automatically flagged
- If it's not a mine, it's the opponent's turn
- Reveal more mines than your opponent to win
- Numbers show how many mines are adjacent to a cell

## Architecture

The project is a Cargo workspace with four crates:

- **`core`**: pure game logic with no dependencies. Handles board generation, cell revealing, mine detection, and win conditions for a solo game. Published as [`minesweeper_core`](https://crates.io/crates/minesweeper_core) on crates.io.
- **`multiplayer`**: implementation of the multiplayer rules. Contains shared types (game state, player actions, messages) used by both client and server to stay in sync.
- **`server`**: serves the web client as static files and manages game sessions over WebSockets, built with [Axum](https://github.com/tokio-rs/axum) and [Tokio](https://tokio.rs/).
- **`client`**: a native desktop and a WebAssembly app built with [egui](https://github.com/emilk/egui) that runs in the browser and communicates with the server via WebSocket.

## Running Locally

**Prerequisites:** Rust, [`Trunk`](https://trunk-rs.github.io/trunk/) (`cargo install trunk`), and [`just`](https://github.com/casey/just) (`cargo install just`).

Run both server and client:

**Build and serve the WASM client:**
```sh
just client
```

**Start the server:**
```sh
just server
```

**Alternatively, run desktop client**
```sh
cargo run -p minesboomer
```
