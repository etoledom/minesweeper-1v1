use std::fmt::Display;

use ewebsock::{WsEvent, WsMessage, WsReceiver, WsSender};
use minesweeper_multiplayer::messages::*;

pub enum Message {
    GameStarted(GameStartMessage),
    OpenGames(OpenGamesMessage),
    CellSelected(CellSelectedMessage),
    Text(String),
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::GameStarted(_) => write!(f, "Game start"),
            Message::OpenGames(msg) => write!(f, "Open games list: {}", msg.games.len()),
            Message::CellSelected(msg) => write!(f, "Cell selected: ({}, {})", msg.coordinates.x, msg.coordinates.y),
            Message::Text(msg) => write!(f, "{}", msg),
        }
    }
}

pub struct WSClient {
    sender: WsSender,
    reseiver: WsReceiver,
}

impl WSClient {
    pub fn new(sender: WsSender, reseiver: WsReceiver) -> Self {
        WSClient { sender, reseiver }
    }

    pub fn poll(&self) -> Vec<Message> {
        let mut messages = vec![];
        while let Some(event) = self.reseiver.try_recv() {
            match event {
                WsEvent::Opened => println!("Opened"),
                WsEvent::Message(ws_message) => {
                    messages.push(self.receive_message(ws_message));
                }
                WsEvent::Error(err) => println!("Error: {}", err),
                WsEvent::Closed => println!("Closed"),
            }
        }
        messages
    }

    pub fn send_message(&mut self, message: impl JsonConvertible) {
        self.sender.send(WsMessage::Text(message.to_json()));
    }

    fn receive_message(&self, message: WsMessage) -> Message {
        let string = match message {
            WsMessage::Text(text) => text,
            _ => "".to_string(),
        };

        if let Ok(msg) = serde_json::from_str::<GameStartMessage>(&string) {
            println!("-> GameStartMessage. active: {}", msg.is_active);
            Message::GameStarted(msg)
        } else if let Ok(msg) = serde_json::from_str::<CellSelectedMessage>(&string) {
            println!("-> CellSelectedMessage: {}", msg.to_json());
            Message::CellSelected(msg)
        } else if let Ok(msg) = OpenGamesMessage::from_json(&string) {
            println!("-> OpenGamesMessage: {}", msg.to_json());
            Message::OpenGames(msg)
        } else if let Ok(msg) = serde_json::from_str::<SimpleMessage>(&string) {
            println!("-> SimpleMessage: {}", msg.name);
            Message::Text(msg.name)
        } else {
            Message::Text("Error".to_string())
        }
    }
}
