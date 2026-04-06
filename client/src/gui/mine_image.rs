use eframe::egui::{self, include_image, Vec2};
use egui::{Color32, Image, Response, Widget};

#[derive(Default)]
pub struct MineImage;

impl MineImage {
    pub fn ui(&mut self, ui: &mut egui::Ui) -> Response {
        Image::new(include_image!("../../assets/mine.png"))
            .bg_fill(Color32::from_rgba_premultiplied(150, 29, 27, 100))
            .fit_to_exact_size(Vec2::new(50., 50.))
            .ui(ui)
    }
}
