use eframe::egui::{CornerRadius, FontId, Frame, Margin, Response, RichText, Ui, Widget};

use crate::gui::colors::ColorScheme;
use crate::gui::strings::Strings;

pub struct ToWinSection<'a> {
    mines_remaining: usize,
    colors: &'a ColorScheme,
}

impl<'a> ToWinSection<'a> {
    pub fn new(mines_remaining: usize, colors: &'a ColorScheme) -> Self {
        Self {
            mines_remaining,
            colors,
        }
    }
}

impl<'a> Widget for ToWinSection<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        Frame::NONE
            .fill(self.colors.background_secondary)
            .corner_radius(CornerRadius::same(8))
            .inner_margin(Margin::symmetric(16, 12))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("⭐").size(16.0));
                    ui.label(
                        RichText::new(Strings::more_to_win_label(self.mines_remaining))
                            .color(self.colors.text_primary)
                            .font(FontId::proportional(16.0)),
                    );
                });
            })
            .response
    }
}
