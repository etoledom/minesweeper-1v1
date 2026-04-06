use minesweeper_core::{Board, Difficulty};
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
    pub name: String,
    board: SerializableBoard,
    pub is_active: bool,
}

impl GameStartMessage {
    pub fn new(board: SerializableBoard, is_active: bool) -> Self {
        GameStartMessage {
            name: "start".to_owned(),
            board,
            is_active,
        }
    }

    pub fn get_board(&self) -> Board {
        self.board.clone().into()
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
pub struct IdentificationMessage {
    pub name: String,
    pub user_id: String,
}

impl IdentificationMessage {
    pub fn new(user_id: String) -> Self {
        IdentificationMessage {
            name: "user_identification".to_owned(),
            user_id,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CellSelectedMessage {
    pub name: String,
    pub is_active_player: bool,
    pub coordinates: SerializablePoint,
}

impl CellSelectedMessage {
    pub fn new(coordinates: SerializablePoint, is_active_player: bool) -> Self {
        CellSelectedMessage {
            name: "cell_selected".to_owned(),
            is_active_player,
            coordinates,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct OpenGamesMessage {
    pub name: String,
    pub games: Vec<GameDefinition>,
}

impl OpenGamesMessage {
    pub fn new(games: Vec<GameDefinition>) -> Self {
        OpenGamesMessage { name: "open_games".to_owned(), games }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateGameMessage {
    pub name: String,
    pub game: GameDefinition,
}

impl CreateGameMessage {
    pub fn new(name: impl Into<String>, difficulty: Difficulty) -> Self {
        let game = GameDefinition::new("", name, difficulty);
        CreateGameMessage { name: "create_game".to_owned(), game }
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
