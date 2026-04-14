use minesweeper_core::Difficulty;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct GameDefinition {
    pub host_name: String,
    pub id: String,
    pub difficulty: Difficulty,
}

impl GameDefinition {
    pub fn new(id: impl Into<String>, host_name: impl Into<String>, difficulty: Difficulty) -> Self {
        GameDefinition {
            host_name: host_name.into(),
            id: id.into(),
            difficulty: difficulty.into(),
        }
    }
}
