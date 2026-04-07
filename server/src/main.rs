mod server;
use server::*;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{State, WebSocketUpgrade},
    response::Response,
    routing::get,
    Router,
};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let multi_games: MultiGames = Arc::new(Mutex::new(vec![]));

    let state = PeerMap::new(Mutex::new(HashMap::new()));
    let players = Players::new(Mutex::new(HashMap::new()));
    let server = Arc::new(Server::new(state, multi_games, players));

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .fallback_service(ServeDir::new(concat!(env!("CARGO_MANIFEST_DIR"), "/dist")))
        .with_state(Arc::clone(&server));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(State(server): State<Arc<Server>>, ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(move |socket| async move {
        server.handle_connection(socket).await;
    })
}
