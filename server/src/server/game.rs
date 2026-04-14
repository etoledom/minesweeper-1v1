use minesweeper_multiplayer::{Difficulty, Multiplayer};
use minesweeper_multiplayer::{Game as CoreGame, Point};
use uuid::Uuid;

#[derive(Debug)]
pub struct Game {
    id: String,
    host: Uuid,
    client: Option<Uuid>,
    multi_game: Multiplayer,
}

impl Game {
    pub fn new(player: Uuid, id: impl Into<String> + Clone, difficulty: Difficulty) -> Self {
        Game {
            host: player,
            client: None,
            multi_game: Multiplayer::new(id.clone().into(), "", "", difficulty),
            id: id.into(),
        }
    }

    pub fn set_local_name(&mut self, name: String) {
        self.multi_game.local_player.name = name;
    }

    pub fn setup_multi_game(&mut self) {
        self.multi_game.local_player.id = self.host.to_string();
        self.multi_game.remote_player.id = self.client.as_ref().unwrap().to_string();
    }

    pub fn get_inner_game(&self) -> &CoreGame {
        &self.multi_game.game
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_client(&self) -> Option<&Uuid> {
        self.client.as_ref()
    }

    pub fn set_client(&mut self, client: Uuid, name: String) {
        self.client = Some(client);
        self.multi_game.remote_player.name = name;
        self.multi_game.remote_player.id = client.to_string();
    }

    pub fn remove_client(&mut self) {
        self.client = None;
    }

    pub fn has_client(&self) -> bool {
        self.client.is_some()
    }

    pub fn get_host(&self) -> &Uuid {
        &self.host
    }

    pub fn get_host_name(&self) -> &str {
        &self.multi_game.local_player.name
    }

    pub fn get_client_name(&self) -> &str {
        &self.multi_game.remote_player.name
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

    pub fn get_players(&self) -> Vec<&Uuid> {
        let mut players = vec![&self.host];
        if let Some(client) = &self.client {
            players.push(client);
        }
        players
    }

    pub fn has_player(&self, player: &Uuid) -> bool {
        self.get_players().contains(&player)
    }
}
