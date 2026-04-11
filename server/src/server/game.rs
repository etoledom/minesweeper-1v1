use minesweeper_multiplayer::{Difficulty, Multiplayer};
use minesweeper_multiplayer::{Game as CoreGame, Point};
use uuid::Uuid;

#[derive(Debug)]
pub struct Player {
    id: Uuid,
    name: String,
    game_id: String,
}

impl Player {
    pub fn new(id: Uuid, name: String, game_id: impl Into<String>) -> Self {
        Player {
            id,
            name,
            game_id: game_id.into(),
        }
    }

    pub fn game_id(&self) -> String {
        self.game_id.clone()
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug)]
pub struct Game {
    id: String,
    host: Player,
    client: Option<Player>,
    multi_game: Multiplayer,
}

impl Game {
    pub fn new(player: Player, id: impl Into<String>, difficulty: Difficulty) -> Self {
        Game {
            host: player,
            client: None,
            multi_game: Multiplayer::new("", "", difficulty),
            id: id.into(),
        }
    }

    pub fn setup_multi_game(&mut self) {
        self.multi_game.local_player.id = self.host.get_id().to_string();
        self.multi_game.remote_player.id = self.client.as_ref().unwrap().get_id().to_string();
    }

    pub fn get_inner_game(&self) -> &CoreGame {
        &self.multi_game.game
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_client(&self) -> Option<&Player> {
        self.client.as_ref()
    }

    pub fn set_client(&mut self, client: Player) {
        self.client = Some(client);
    }

    pub fn remove_client(&mut self) {
        self.client = None;
    }

    pub fn has_client(&self) -> bool {
        self.client.is_some()
    }

    pub fn get_host(&self) -> &Player {
        &self.host
    }

    pub fn get_difficulty(&self) -> &Difficulty {
        self.multi_game.get_difficulty()
    }

    pub fn player_selected(&mut self, coordinates: Point) {
        self.multi_game.player_selected(coordinates);
    }

    pub fn is_player_active(&self, player_id: impl Into<String>) -> bool {
        self.multi_game.current_player().id == player_id.into()
    }

    pub fn get_players(&self) -> Vec<&Player> {
        let mut players = vec![&self.host];
        if let Some(client) = &self.client {
            players.push(client);
        }
        players
    }
}
