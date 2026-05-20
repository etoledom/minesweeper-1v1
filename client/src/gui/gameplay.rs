use crate::{
    gui::{
        board::Board as BoardUI,
        colors::ColorScheme,
        game_panel::{
            game_list::{GameList, GameListAction},
            join_game_form::{JoinGameAction, JoinGameForm},
            new_game_form::{NewGameForm, NewGameFormAction},
            players_section::PlayersSection,
            progress_section::ProgressSection,
            to_win_section::ToWinSection,
            turn_indicator::TurnIndicator,
            waiting::WaitingScreen,
        },
    },
    networking::{Message, WSClient},
};

use minesweeper_multiplayer::*;

use crate::gui::strings::Strings;

use eframe::egui;
use egui::{Label, RichText, Ui, Widget};

#[derive(Clone)]
pub struct OpenGame {
    pub name: String,
    pub difficulty: Difficulty,
    pub game_id: String,
}

pub struct MinesBoomer {
    pub game: Option<Multiplayer>,
    ws_client: WSClient,
    is_active: bool,
    games_list: Vec<OpenGame>,
    pub waiting_for_enemy: bool,
    waiting_game_info: Option<(String, Difficulty)>,
    show_game_name_popup: bool,
    last_remote_move: Option<Point>,
    new_game_form: NewGameForm,
    join_game_form: JoinGameForm,
    scheme: ColorScheme,
}

impl MinesBoomer {
    pub fn new(ws_client: WSClient) -> Self {
        MinesBoomer {
            game: None,
            ws_client,
            is_active: false,
            games_list: vec![],
            waiting_for_enemy: false,
            waiting_game_info: None,
            show_game_name_popup: false,
            last_remote_move: None,
            new_game_form: Default::default(),
            join_game_form: Default::default(),
            scheme: ColorScheme::dark(),
        }
    }

    fn draw_board(&mut self, game: &mut Multiplayer, ui: &mut Ui) {
        if let Some(coordinates) = BoardUI::new(&game, &game.local_player, self.last_remote_move, &self.scheme).show(ui)
        {
            self.on_cell_tapped(game, coordinates);
        }
    }

    fn draw_gui(&mut self, game: &Multiplayer, ui: &mut Ui) {
        if let Some(winner) = game.winner() {
            ui.vertical_centered_justified(|ui| {
                ui.heading(RichText::new(Strings::winner()).color(self.scheme.accent));
                ui.heading(RichText::new(winner.name.to_string()).color(self.scheme.text_primary));
            });
            return;
        }

        ui.vertical_centered_justified(|ui| {
            TurnIndicator {
                is_my_turn: self.is_active,
                opponent_name: &game.remote_player.name,
                colors: &self.scheme,
            }
            .ui(ui);

            ui.separator();
            ui.add_space(10.);

            PlayersSection::new(&game.local_player, &game.remote_player, &self.scheme).ui(ui);

            ui.separator();
            ui.add_space(10.);

            ProgressSection::new(
                &game.local_player,
                &game.remote_player,
                game.total_mines_to_win(),
                &self.scheme,
            )
            .ui(ui);

            ui.add_space(10.);
            ui.separator();
            ui.add_space(10.);

            ToWinSection::new(game.local_to_win(), &self.scheme).ui(ui);
        });
    }

    fn draw_game_list(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.add(Label::new(
                RichText::new(Strings::app_title()).size(50.).color(self.scheme.text_primary),
            ));
            ui.separator();
            ui.add_space(10.);
            ui.label(egui::RichText::new(Strings::find_game_subtitle()).color(self.scheme.text_secondary));
            ui.add_space(10.);
            ui.vertical_centered(|ui| {
                ui.set_max_width(400.);
                if ui.button(Strings::new_game_button()).clicked() {
                    self.show_game_name_popup = true;
                }

                let action = GameList::new(&self.games_list, &self.scheme).show(ui);
                match action {
                    GameListAction::Join(game) => {
                        self.join_game_form.open(game);
                    }
                    _ => {}
                }

                if self.join_game_form.is_open() {
                    match self.join_game_form.show(ui, &self.scheme) {
                        JoinGameAction::Join(game_id, player_name) => self.send_join_game_message(game_id, player_name),
                        JoinGameAction::Cancel => self.join_game_form.reset(),
                        _ => {}
                    }
                }

                if self.show_game_name_popup {
                    let action = self.new_game_form.show(ui, &self.scheme);

                    match action {
                        NewGameFormAction::Create { name, difficulty } => {
                            self.waiting_game_info = Some((name.clone(), difficulty));
                            self.send_create_new_game_message(name, difficulty);
                            self.show_game_name_popup = false;
                        }
                        NewGameFormAction::Cancel => {
                            self.show_game_name_popup = false;
                        }
                        NewGameFormAction::None => {}
                    }
                }
            });
        });
    }

    fn draw_waiting_screen(&self, ui: &mut Ui) {
        let game_info = self
            .waiting_game_info
            .as_ref()
            .map(|(name, diff)| (name.as_str(), diff));
        WaitingScreen::new(game_info, &self.scheme).show(ui);
    }

    fn on_cell_tapped(&mut self, game: &mut Multiplayer, coordinates: Point) {
        if !self.is_active {
            return;
        }
        if game.winner().is_none() {
            game.player_selected(coordinates);
            self.send_selected_message(coordinates, game.game_id.clone());
        }
    }
}

impl eframe::App for MinesBoomer {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ui.ctx());

        self.scheme = if ui.visuals().dark_mode {
            ColorScheme::dark()
        } else {
            ColorScheme::light()
        };

        for message in self.ws_client.poll().iter() {
            self.handle_message(message);
        }
        ui.request_repaint();

        if let Some(mut game) = self.game.take() {
            egui::Panel::right("game_panel")
                .max_size(500.0)
                .show_separator_line(true)
                .resizable(false)
                .show_inside(ui, |ui| {
                    self.draw_gui(&game, ui);
                });

            egui::CentralPanel::default().show_inside(ui, |ui| {
                ui.horizontal_top(|ui| {
                    self.draw_board(&mut game, ui);
                });
            });
            self.game = Some(game);
        } else {
            egui::CentralPanel::default().show_inside(ui, |ui| {
                if self.waiting_for_enemy {
                    self.draw_waiting_screen(ui);
                    return;
                }

                self.draw_game_list(ui);
                return;
            });
        }
    }
}

// Messages

impl MinesBoomer {
    pub fn handle_message(&mut self, message: &Message) {
        println!("Receiving: {}", message);
        match message {
            Message::GameStarted(msg) => {
                let game = msg.get_game();
                let mut multi_game =
                    Multiplayer::new_with_game(game, msg.game_id.clone(), &msg.local_player, &msg.remote_player);

                multi_game.local_player.is_active = msg.is_active;
                multi_game.remote_player.is_active = !msg.is_active;

                self.is_active = msg.is_active;
                self.game = Some(multi_game);

                self.waiting_for_enemy = false;
                self.waiting_game_info = None;
            }
            Message::OpenGames(msg) => {
                let games = msg
                    .games
                    .iter()
                    .map(|game| OpenGame {
                        name: game.host_name.clone(),
                        difficulty: game.difficulty.into(),
                        game_id: game.id.clone(),
                    })
                    .collect();
                self.games_list = games;
            }
            Message::CellSelected(msg) => {
                if let Some(game) = &mut self.game {
                    let coordinates: Point = msg.coordinates.into();
                    game.player_selected(coordinates);
                    if msg.is_remote_sender {
                        self.last_remote_move = Some(coordinates);
                    }
                    game.local_player.is_active = msg.is_active_player;
                    game.remote_player.is_active = !msg.is_active_player;
                    self.is_active = msg.is_active_player;
                }
            }
            Message::Text(msg) => match msg.as_str() {
                "connected" => {
                    self.request_open_games();
                }
                "waiting_enemy" => {
                    self.waiting_for_enemy = true;
                }
                "client_disconnected" => self.waiting_for_enemy = true,
                "host_disconnected" => {
                    self.game = None;
                    self.games_list = vec![];
                    self.request_open_games();
                }
                _ => return,
            },
        }
        println!("Ok.");
    }

    pub fn request_open_games(&mut self) {
        println!("<- Sending games request");
        let message = SimpleMessage::new("games_request");
        self.send_message(message);
    }

    pub fn send_selected_message(&mut self, coordinates: Point, game_id: String) {
        println!("<- Sending cell selected");
        let message = CellSelectedMessage::new(coordinates, game_id, true, false);
        self.send_message(message);
    }

    fn send_join_game_message(&mut self, game_id: impl Into<String>, player_name: impl Into<String>) {
        println!("<- Sending joing game");
        let message = JoinGameMessage::new(game_id, player_name);
        self.send_message(message);
    }

    fn send_message(&mut self, message: impl JsonConvertible) {
        self.ws_client.send_message(message);
    }
}

impl MinesBoomer {
    fn send_create_new_game_message(&mut self, name: String, difficulty: Difficulty) {
        println!("<- Sending create new game");
        let message = CreateGameMessage::new(name, difficulty);
        println!("Message: {}", message.to_json());
        self.send_message(message);
    }
}
