use egui::{Color32, FontId, Response, Vec2, Widget};

pub struct AvatarCircle {
    letter: char,
    color: Color32,
    size: f32,
}

impl AvatarCircle {
    pub fn new(letter: char, color: Color32, size: f32) -> Self {
        Self { letter, color, size }
    }
}

impl Widget for AvatarCircle {
    fn ui(self, ui: &mut egui::Ui) -> Response {
        // Avatar circle with initial
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(self.size), egui::Sense::hover());
        let center = rect.center();
        let radius = self.size / 2.0;

        ui.painter().circle_filled(center, radius, self.color);

        ui.painter().text(
            center,
            egui::Align2::CENTER_CENTER,
            self.letter,
            FontId::proportional(18.0),
            Color32::WHITE,
        );

        response
    }
}
