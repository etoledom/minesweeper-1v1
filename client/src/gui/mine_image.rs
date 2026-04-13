use eframe::egui::{self, include_image, Vec2};
use egui::{Color32, Image, Response, Widget};

// Keeping this in case images are needed soon.
// TODO: Remove if images will not be needed.
#[derive(Default)]
#[allow(dead_code)]
pub struct MineImage;

#[allow(dead_code)]
impl MineImage {
    pub fn ui(ui: &mut egui::Ui) -> Response {
        Image::new(include_image!("../../assets/mine.png"))
            .bg_fill(Color32::from_rgba_premultiplied(150, 29, 27, 100))
            .fit_to_exact_size(Vec2::new(50., 50.))
            .ui(ui)
    }
}
