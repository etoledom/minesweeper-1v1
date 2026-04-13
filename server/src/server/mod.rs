mod game;
use axum::extract::ws::{Message, WebSocket};
use game::*;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use minesweeper_multiplayer::messages::*;
use minesweeper_multiplayer::serializables::*;
use uuid::Uuid;

use futures::channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, stream::TryStreamExt, StreamExt};

pub type Tx = UnboundedSender<Message>;
pub type PeerMap = Arc<Mutex<HashMap<Uuid, Tx>>>;
pub type MultiGames = Arc<Mutex<Vec<Game>>>;
pub type Players = Arc<Mutex<HashMap<Uuid, String>>>;

pub struct Server {
    peer_map: PeerMap,
    games: MultiGames,
    players: Players,
}

impl Server {
    pub fn new(peer_map: PeerMap, games: MultiGames, players: Players) -> Self {
        Server {
            peer_map,
            games,
            players,
        }
    }

    pub async fn handle_connection(self: Arc<Self>, socket: WebSocket) {
        let uuid = Uuid::new_v4();

        println!("WebSocket connection established with ID: {}", uuid.to_string());

        let (tx, rx) = unbounded();

        self.peer_map.lock().unwrap().insert(uuid, tx);
        self.request_identification(uuid);

        let (outgoing, incoming) = socket.split();

        let receive_from_client = incoming.try_for_each(|msg| {
            println!("Received a message from {}: {}", uuid, msg.to_text().unwrap());
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
        self.remove_player(uuid);
    }

    fn remove_player(&self, id: Uuid) {
        let mut games = self.games.lock().unwrap();
        if let Some(index) = games.iter().position(|game| game.get_host().get_id() == id) {
            let removed_game = games.remove(index);
            if let Some(client) = removed_game.get_client() {
                self.send_message_to(client, SimpleMessage::new("host_disconnected").to_json());
            }
        } else {
            let mut games_with_clients = games.iter_mut().filter(|game| game.has_client());
            if let Some(game) = games_with_clients.find(|game| game.get_client().unwrap().get_id() == id) {
                game.remove_client();
                self.send_message_to(game.get_host(), SimpleMessage::new("client_disconnected").to_json());
            }
        }
        self.players.lock().unwrap().remove(&id);
        self.peer_map.lock().unwrap().remove(&id);
    }

    fn request_identification(&self, id: Uuid) {
        let message = Message::Text(SimpleMessage::new("identify").to_json().into());
        let peer_guard = self.peer_map.lock().unwrap();
        let tx = peer_guard.get(&id);
        println!("-> Sending identify");
        match tx.unwrap().unbounded_send(message) {
            Ok(_) => println!("Ok."),
            Err(err) => println!("{}", err),
        }
    }

    fn handle_identification_message(&self, message: IdentificationMessage, id: Uuid) {
        let player = Player::new(id, message.name, "");
        self.players.lock().unwrap().insert(id, player.game_id());
    }

    fn handle_received_message(&self, msg: Message, id: Uuid) {
        let message_string = msg.to_text().unwrap();
        if let Ok(message) = IdentificationMessage::from_json(message_string) {
            println!("Identification received for {}", message.name);
            self.handle_identification_message(message, id);
        } else if let Ok(message) = CellSelectedMessage::from_json(message_string) {
            let game_id = self.players.lock().unwrap().get(&id).unwrap().clone();
            let mut games = self.games.lock().unwrap();
            let game = games.iter_mut().find(|game| game.get_id() == game_id).unwrap();
            game.player_selected(message.coordinates.into());
            self.send_selected_to_players(game, message.coordinates, id);
        } else if let Ok(message) = CreateGameMessage::from_json(message_string) {
            let mut games_guard = self.games.lock().unwrap();
            let game_id = Uuid::new_v4().to_string();
            let player = Player::new(id, message.game.name, &game_id);
            let mut players = self.players.lock().unwrap();
            players.insert(id, player.game_id());
            let game = Game::new(player, game_id, message.game.difficulty.into());
            games_guard.push(game);
            self.send_message_to_id(&id, SimpleMessage::new("waiting_enemy").to_json());

            for other_player in players.iter() {
                if id != *other_player.0 {
                    println!("Sending to: {}", id);
                    self.send_open_games(other_player.0, &games_guard);
                }
            }
        } else if let Ok(message) = JoinGameMessage::from_json(message_string) {
            println!("-> Client joined game");
            let mut games = self.games.lock().unwrap();
            let game = games.iter_mut().find(|game| game.get_id() == message.game_id).unwrap();
            let client = Player::new(id, message.client_name, game.get_id());
            self.players.lock().unwrap().insert(id, client.game_id());
            game.set_client(client);
            game.setup_multi_game();
            self.send_new_game_to_players(game);
        } else if let Ok(message) = SimpleMessage::from_json(message_string) {
            if message.name == "games_request" {
                self.send_open_games(&id, &self.games.lock().unwrap());
            }
        }
    }

    fn send_open_games(&self, id: &Uuid, games: &Vec<Game>) {
        let game_defs = games
            .iter()
            .filter(|game| !game.has_client())
            .map(|game| {
                let name = game.get_host().get_name();
                let id = game.get_id();
                let difficulty = game.get_difficulty();
                GameDefinition::new(id, name, difficulty.clone())
            })
            .collect();
        let message = OpenGamesMessage::new(game_defs);
        self.send_message_to_id(id, message.to_json());
    }

    fn send_selected_to_players(&self, game: &Game, coordinates: SerializablePoint, sender_id: Uuid) {
        for player in game.get_players() {
            let is_active = game.is_player_active(player.get_id());
            println!("SenderID = playerID: {} = {}", sender_id, player.get_id());
            self.send_message_to(
                player,
                CellSelectedMessage::new(coordinates, sender_id != player.get_id(), is_active).to_json(),
            );
        }
    }

    fn send_new_game_to_players(&self, game: &Game) {
        for player in game.get_players() {
            let is_active = game.is_player_active(player.get_id());
            let local_player = player.get_name().to_string();
            let remote_player = if let Some(remote_player) = game
                .get_players()
                .iter()
                .find(|other_player| other_player.get_id() != player.get_id())
            {
                remote_player.get_name().to_string()
            } else {
                "".to_string()
            };
            self.send_message_to(
                player,
                GameStartMessage::new(game.get_inner_game(), is_active, local_player, remote_player).to_json(),
            );
        }
    }

    fn send_message_to(&self, player: &Player, message_json: String) {
        self.send_message_to_id(&player.get_id(), message_json);
    }

    fn send_message_to_id(&self, id: &Uuid, message_json: String) {
        let peers = self.peer_map.lock().unwrap();
        let sender = peers.get(&id);
        sender
            .unwrap()
            .unbounded_send(Message::Text(message_json.into()))
            .unwrap();
    }

    fn _send_to_all(&self, msg: Message) {
        let peers = self.peer_map.lock().unwrap();

        let broadcast_recipients = peers.iter().map(|(_, ws_sink)| ws_sink);

        println!("Sending message to all");
        for recp in broadcast_recipients {
            recp.unbounded_send(msg.clone()).unwrap();
        }
    }
}
