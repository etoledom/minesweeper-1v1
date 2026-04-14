mod server;
use crate::server::*;

use std::sync::Arc;

use axum::{
    extract::{State, WebSocketUpgrade},
    response::Response,
    routing::get,
    Router,
};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let server = Arc::new(Server::new());

    let dist_path = if std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/dist")).exists() {
        concat!(env!("CARGO_MANIFEST_DIR"), "/dist")
    } else {
        "/dist"
    };

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .fallback_service(ServeDir::new(dist_path))
        .with_state(Arc::clone(&server));

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(State(server): State<Arc<Server>>, ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(move |socket| async move {
        server.handle_connection(socket).await;
    })
}
