use minesweeper_core::{Difficulty, Game, Point};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json;

use crate::serializables::*;

pub trait JsonConvertible: Sized {
    fn from_json(str: &str) -> Result<Self, serde_json::Error>;
    fn to_json(&self) -> String;
}

impl<T: Serialize + DeserializeOwned> JsonConvertible for T {
    fn from_json(str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(str)
    }

    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct GameStartMessage {
    pub game_id: String,
    pub local_player: String,
    pub remote_player: String,
    pub is_active: bool,
    game: Game,
}

impl GameStartMessage {
    pub fn new(
        game_id: impl Into<String>,
        game: Game,
        is_active: bool,
        local_player: String,
        remote_player: String,
    ) -> Self {
        GameStartMessage {
            game_id: game_id.into(),
            local_player,
            remote_player,
            game,
            is_active,
        }
    }

    pub fn get_game(&self) -> Game {
        self.game.clone().into()
    }
}

#[derive(Serialize, Deserialize)]
pub struct SimpleMessage {
    pub name: String,
}

impl SimpleMessage {
    pub fn new(name: impl Into<String>) -> Self {
        SimpleMessage { name: name.into() }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CellSelectedMessage {
    pub game_id: String,
    pub is_remote_sender: bool,
    pub is_active_player: bool,
    pub coordinates: Point,
}

impl CellSelectedMessage {
    pub fn new(coordinates: Point, game_id: String, is_remote_sender: bool, is_active_player: bool) -> Self {
        CellSelectedMessage {
            game_id,
            is_remote_sender,
            is_active_player,
            coordinates,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct OpenGamesMessage {
    pub games: Vec<GameDefinition>,
}

impl OpenGamesMessage {
    pub fn new(games: Vec<GameDefinition>) -> Self {
        OpenGamesMessage { games }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateGameMessage {
    pub game: GameDefinition,
}

impl CreateGameMessage {
    pub fn new(name: impl Into<String>, difficulty: Difficulty) -> Self {
        let game = GameDefinition::new("", name, difficulty);
        CreateGameMessage { game }
    }
}

#[derive(Serialize, Deserialize)]
pub struct JoinGameMessage {
    pub name: String,
    pub game_id: String,
    pub client_name: String,
}

impl JoinGameMessage {
    pub fn new(game_id: impl Into<String>, client_name: impl Into<String>) -> Self {
        JoinGameMessage {
            name: "join_game".to_owned(),
            game_id: game_id.into(),
            client_name: client_name.into(),
        }
    }
}
