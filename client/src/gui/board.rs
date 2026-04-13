use eframe::egui::{self, Button, Color32, ScrollArea, Ui, Vec2};
use egui::{RichText, Stroke};
use minesweeper_multiplayer::{Cell, CellKind, CellState, Multiplayer, Player, Point};

use crate::gui::colors::ColorScheme;

const CELL_SIZE: f32 = 50.0;

pub struct Board<'a> {
    game: &'a Multiplayer,
    local_player: &'a Player,
    last_opponent_move: Option<Point>,
    colors: &'a ColorScheme,
}

impl<'a> Board<'a> {
    pub fn new(
        game: &'a Multiplayer,
        local_player: &'a Player,
        last_opponent_move: Option<Point>,
        colors: &'a ColorScheme,
    ) -> Self {
        Self {
            game,
            local_player,
            last_opponent_move,
            colors,
        }
    }

    pub fn show(self, ui: &mut Ui) -> Option<Point> {
        let mut tapped: Option<Point> = None;
        let dimensions = self.game.get_board_dimentions();

        let board_width = dimensions.width as f32 * CELL_SIZE;
        let board_height = dimensions.height as f32 * CELL_SIZE;

        ScrollArea::both().auto_shrink([false, false]).show(ui, |ui| {
            ui.horizontal(|ui| {
                let available = ui.available_width();
                if available > board_width {
                    ui.add_space((available - board_width) / 2.0);
                }

                ui.vertical(|ui| {
                    let available = ui.available_height();
                    if available > board_height {
                        ui.add_space((available - board_height) / 2.0);
                    }

                    ui.horizontal(|ui| {
                        self.draw_board(&mut tapped, dimensions, ui);
                    });
                });
            });
        });

        tapped
    }

    fn draw_board(&self, tapped: &mut Option<Point>, dimensions: minesweeper_multiplayer::Size, ui: &mut Ui) {
        for x in 0..dimensions.width {
            ui.vertical(|ui| {
                for y in 0..dimensions.height {
                    let coordinates = Point { x, y };
                    let Some(cell) = self.game.get_board().cell_at(coordinates) else {
                        continue;
                    };
                    let cell = cell.clone();

                    if cell.is_mine() && cell.is_cleared() {
                        let color = self.mine_color(coordinates);
                        draw_flag(ui, color, self.colors);
                    } else {
                        let bg_color = self.cell_background_color(cell);
                        let text = cell_text(&cell);
                        let current_point = Point { x, y };
                        let stroke = self
                            .last_opponent_move
                            .filter(|last_move| *last_move == current_point)
                            .map(|_| Stroke::new(2.0, self.colors.player_opponent))
                            .unwrap_or_default();

                        if ui
                            .add_sized([CELL_SIZE, CELL_SIZE], Button::new(text).fill(bg_color).stroke(stroke))
                            .clicked()
                        {
                            *tapped = Some(coordinates);
                        }
                    }
                }
            });
        }
    }

    fn cell_background_color(&self, cell: Cell) -> Color32 {
        if cell.is_cleared() {
            self.colors.background_cell_revealed
        } else {
            self.colors.background_cell_hidden
        }
    }

    fn mine_color(&self, coordinates: Point) -> Color32 {
        if self.local_player.has_mine(coordinates) {
            self.colors.player_self
        } else {
            self.colors.player_opponent
        }
    }
}

fn cell_text(cell: &Cell) -> RichText {
    match (cell.state, cell.kind) {
        (CellState::Cleared, CellKind::Number(number)) => RichText::new(number.to_string())
            .color(number_color(number))
            .size(16.0)
            .strong(),
        (_, _) => RichText::default(),
    }
}

fn number_color(n: u8) -> Color32 {
    match n {
        1 => Color32::from_rgb(25, 118, 210),
        2 => Color32::from_rgb(56, 142, 60),
        3 => Color32::from_rgb(211, 47, 47),
        4 => Color32::from_rgb(123, 31, 162),
        5 => Color32::from_rgb(255, 143, 0),
        6 => Color32::from_rgb(0, 151, 167),
        7 => Color32::from_rgb(66, 66, 66),
        8 => Color32::from_rgb(158, 158, 158),
        _ => Color32::WHITE,
    }
}

fn draw_flag(ui: &mut Ui, flag_color: Color32, colors: &ColorScheme) {
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(CELL_SIZE), egui::Sense::hover());

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();

        // Cell background
        painter.rect_filled(rect, 0.0, colors.background_cell_revealed);

        // Pole
        let pole_x = rect.left() + CELL_SIZE * 0.35;
        let pole_top = rect.top() + CELL_SIZE * 0.15;
        let pole_bottom = rect.bottom() - CELL_SIZE * 0.15;
        painter.line_segment(
            [egui::pos2(pole_x, pole_top), egui::pos2(pole_x, pole_bottom)],
            egui::Stroke::new(2.0, colors.text_primary),
        );

        // Flag (triangle)
        let flag_points = vec![
            egui::pos2(pole_x, pole_top),
            egui::pos2(rect.right() - CELL_SIZE * 0.2, pole_top + CELL_SIZE * 0.2),
            egui::pos2(pole_x, pole_top + CELL_SIZE * 0.4),
        ];
        painter.add(egui::Shape::convex_polygon(flag_points, flag_color, egui::Stroke::NONE));
    }
}
