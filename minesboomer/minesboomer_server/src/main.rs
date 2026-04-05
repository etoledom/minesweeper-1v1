mod server;
use server::*;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use std::thread;

use std::{
    io::{prelude::*, BufReader},
    net::TcpStream,
};

use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::Response,
    routing::get,
    Router,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    tokio::spawn(start_web_server());

    let addr = "0.0.0.0:8000".to_string();

    let multi_games: MultiGames = Arc::new(Mutex::new(vec![]));

    let state = PeerMap::new(Mutex::new(HashMap::new()));
    let players = Players::new(Mutex::new(HashMap::new()));

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    let server = Arc::new(Server::new(state.clone(), Arc::clone(&multi_games), Arc::clone(&players)));

    // Let's spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        let server = Arc::clone(&server);
        tokio::spawn(server.handle_connection(stream, addr));
    }
}

async fn start_web_server() {
    let app = Router::new().route("/", get(handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        if socket.send(msg).await.is_err() {
            break; // client disconnected
        }
    }
}
