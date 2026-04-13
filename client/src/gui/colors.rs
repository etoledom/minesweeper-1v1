use egui::Color32;

#[allow(dead_code)]
pub struct ColorScheme {
    pub background_primary: Color32,
    pub background_secondary: Color32,
    pub background_cell_hidden: Color32,
    pub background_cell_revealed: Color32,

    pub text_primary: Color32,
    pub text_secondary: Color32,
    pub text_muted: Color32,

    pub player_self: Color32,
    pub player_opponent: Color32,

    pub mine_self: Color32,
    pub mine_opponent: Color32,

    pub accent: Color32,
    pub separator: Color32,

    pub difficulty_easy: Color32,
    pub difficulty_medium: Color32,
    pub difficulty_hard: Color32,
}

#[allow(dead_code)]
impl ColorScheme {
    pub fn dark() -> Self {
        Self {
            background_primary: Color32::from_rgb(30, 30, 34),
            background_secondary: Color32::from_rgb(42, 42, 46),
            background_cell_hidden: Color32::from_rgb(74, 74, 80),
            background_cell_revealed: Color32::from_rgb(184, 184, 184),

            text_primary: Color32::from_rgb(232, 232, 232),
            text_secondary: Color32::from_rgb(170, 170, 170),
            text_muted: Color32::from_rgb(136, 136, 136),

            player_self: Color32::from_rgb(52, 152, 219),
            player_opponent: Color32::from_rgb(192, 57, 43),

            mine_self: Color32::from_rgb(39, 174, 96),
            mine_opponent: Color32::from_rgb(192, 57, 43),

            accent: Color32::from_rgb(240, 192, 64),
            separator: Color32::from_rgb(58, 58, 64),

            difficulty_easy: Color32::from_rgb(39, 174, 96), // green, matches player_self
            difficulty_medium: Color32::from_rgb(241, 196, 15), // amber/yellow
            difficulty_hard: Color32::from_rgb(231, 76, 60), // red
        }
    }

    pub fn light() -> Self {
        Self {
            background_primary: Color32::from_rgb(245, 245, 245),
            background_secondary: Color32::from_rgb(255, 255, 255),
            background_cell_hidden: Color32::from_rgb(200, 200, 205),
            background_cell_revealed: Color32::from_rgb(240, 240, 240),

            text_primary: Color32::from_rgb(30, 30, 34),
            text_secondary: Color32::from_rgb(80, 80, 80),
            text_muted: Color32::from_rgb(130, 130, 130),

            player_self: Color32::from_rgb(30, 108, 175),
            player_opponent: Color32::from_rgb(192, 57, 43),

            mine_self: Color32::from_rgb(39, 174, 96),
            mine_opponent: Color32::from_rgb(192, 57, 43),

            accent: Color32::from_rgb(186, 140, 20),
            separator: Color32::from_rgb(220, 220, 225),

            difficulty_easy: Color32::from_rgb(30, 132, 73),
            difficulty_medium: Color32::from_rgb(183, 149, 11),
            difficulty_hard: Color32::from_rgb(192, 57, 43),
        }
    }
}
