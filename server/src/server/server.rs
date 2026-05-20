use axum::extract::ws::{Message, WebSocket};

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use minesweeper_multiplayer::{messages::*, DeserializeOwned, Point};
use minesweeper_multiplayer::{serializables::*, Serialize};
use uuid::Uuid;

use futures::channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, stream::TryStreamExt, StreamExt};

use crate::server::game::*;

pub type Tx = UnboundedSender<Message>;
pub type MultiGames = Arc<Mutex<Vec<Game>>>;
pub type ConnectedUsers = Arc<Mutex<HashMap<Uuid, Tx>>>;

pub struct Server {
    games: MultiGames,
    connections: ConnectedUsers,
}

impl Server {
    pub fn new() -> Self {
        Server {
            games: MultiGames::new(Mutex::new(vec![])),
            connections: ConnectedUsers::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn handle_connection(self: Arc<Self>, socket: WebSocket) {
        let uuid = Uuid::new_v4();

        println!("WebSocket connection established with ID: {}", uuid.to_string());

        let (tx, rx) = unbounded();

        self.send_connected_message(&tx);
        self.connections
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .insert(uuid, tx);

        let (outgoing, incoming) = socket.split();

        let receive_from_client = incoming.try_for_each(|msg| {
            println!(
                "Received a message from {}: {}",
                uuid,
                msg.to_text().unwrap_or_default()
            );
            self.handle_received_message(msg, uuid);
            future::ok(())
        });

        let send_to_client = rx.map(Ok).forward(outgoing);

        // Run until client disconnects or channel is closed.
        tokio::select! {
            _ = receive_from_client => {},
            _ = send_to_client => {},
        }

        println!("{} disconnected", uuid.to_string());
        if let Some(tx) = self.connections.lock().unwrap_or_else(|e| e.into_inner()).remove(&uuid) {
            self.remove_player(uuid, &tx);
        }
    }

    fn remove_player(&self, id: Uuid, sender: &Tx) {
        let mut games = self.games.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(index) = games.iter().position(|game| *game.get_host() == id) {
            games.remove(index);
            self.send_message_to(sender, SimpleMessage::new("host_disconnected").to_json());
        } else {
            let mut games_with_clients = games.iter_mut().filter(|game| game.has_client());
            if let Some(game) = games_with_clients.find(|game| *game.get_client().unwrap() == id) {
                game.remove_client();
                self.send_message_to(sender, SimpleMessage::new("client_disconnected").to_json());
            }
        }
    }

    fn send_connected_message(&self, tx: &Tx) {
        let message = Message::Text(SimpleMessage::new("connected").to_json().into());

        println!("-> Sending identify");
        match tx.unbounded_send(message) {
            Ok(_) => println!("Ok."),
            Err(err) => println!("{}", err),
        }
    }

    fn handle_received_message(&self, msg: Message, sender_id: Uuid) -> Option<()> {
        let message_string = msg.to_text().ok()?;
        let connections = self.connections.lock().unwrap_or_else(|e| e.into_inner());
        let mut games = self.games.lock().unwrap_or_else(|e| e.into_inner());
        let sender = connections.get(&sender_id)?;

        if let Ok(message) = CellSelectedMessage::from_json(message_string) {
            println!("-> Received cell selected message");

            let game = games.iter_mut().find(|game| game.get_id() == message.game_id)?;
            game.player_selected(message.coordinates.into());
            self.send_selected_to_players(game, message.coordinates, &connections, sender_id);
        } else if let Ok(message) = CreateGameMessage::from_json(message_string) {
            println!("-> Creating new game");

            let game_id = Uuid::new_v4();

            let mut game = Game::new(sender_id, game_id, message.game.difficulty.into());
            game.set_local_name(message.game.host_name);

            games.push(game);

            println!("Sending waiting_enemy message");
            self.send_message_to(&sender, SimpleMessage::new("waiting_enemy").to_json());

            println!("Will try to send open games message");
            for connection in connections.iter() {
                if sender_id != *connection.0 {
                    println!("Sending open games message");
                    self.send_open_games(&games, connection.1);
                }
            }
        } else if let Ok(message) = JoinGameMessage::from_json(message_string) {
            println!("-> Client joined game");

            let game = games.iter_mut().find(|game| game.get_id() == message.game_id)?;
            game.set_client(sender_id, message.client_name);
            game.setup_multi_game();

            self.send_new_game_to_players(game, &connections);

            // messager.send_new_game_to_players(game);
        } else if let Ok(message) = SimpleMessage::from_json(message_string) {
            if message.name == "games_request" {
                self.send_open_games(&games, sender);
            }
        }

        Some(())
    }

    fn send_open_games(&self, games: &Vec<Game>, sender: &Tx) {
        let game_defs = games
            .iter()
            .filter(|game| !game.has_client())
            .map(|game| {
                let name = game.get_host_name();
                let id = game.get_id();
                let difficulty = game.get_difficulty();
                GameDefinition::new(id, name, difficulty.clone())
            })
            .collect();
        OpenGamesMessage::new(game_defs).send_message(sender);
    }

    fn send_selected_to_players(
        &self,
        game: &Game,
        coordinates: Point,
        connections: &HashMap<Uuid, Tx>,
        sender_id: Uuid,
    ) -> Option<()> {
        println!("-> Sending cell selected message");

        for player in game.get_players() {
            let is_active = game.is_player_active(player.to_string());
            let sender = connections.get(&player)?;

            CellSelectedMessage::new(coordinates, game.get_id().to_string(), sender_id != *player, is_active)
                .send_message(sender);
        }
        Some(())
    }

    fn send_new_game_to_players(&self, game: &Game, connections: &HashMap<Uuid, Tx>) {
        for player in game.get_players() {
            let is_active = game.is_player_active(player.to_string());
            let (local_name, remote_name) = if game.get_host() == player {
                (game.get_host_name().to_string(), game.get_client_name().to_string())
            } else {
                (game.get_client_name().to_string(), game.get_host_name().to_string())
            };
            if let Some(sender) = connections.get(&player) {
                GameStartMessage::new(
                    game.get_id(),
                    game.get_inner_game().clone(),
                    is_active,
                    local_name,
                    remote_name,
                )
                .send_message(sender);
            }
        }
    }

    fn send_message_to(&self, sender: &Tx, message_json: String) {
        sender.unbounded_send(Message::Text(message_json.into())).unwrap();
    }
}

trait MessageSender {
    fn send_message(&self, connection: &Tx);
}

impl<T: Serialize + DeserializeOwned> MessageSender for T {
    fn send_message(&self, connection: &Tx) {
        connection
            .unbounded_send(Message::Text(self.to_json().into()))
            .unwrap_or_default();
    }
}
