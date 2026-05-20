pub struct Strings;

#[allow(dead_code)]
impl Strings {
    pub fn your_turn() -> &'static str {
        "Your turn"
    }
    pub fn opponents_turn(name: &str) -> String {
        format!("{}'s turn", name)
    }
    pub fn mines_found() -> &'static str {
        "mines"
    }
    pub fn to_win() -> &'static str {
        "To win"
    }
    pub fn more_to_win_label(count: usize) -> String {
        format!("{} more to win", count)
    }
    pub fn more() -> &'static str {
        "more"
    }
    pub fn progress() -> &'static str {
        "Progress"
    }
    pub fn found() -> &'static str {
        "Found"
    }
    pub fn winner() -> &'static str {
        "WINNER!"
    }
    pub fn app_title() -> &'static str {
        "MinesBooMer!"
    }
    pub fn find_game_subtitle() -> &'static str {
        "Find a game to join or create your own"
    }
    pub fn new_game_button() -> &'static str {
        "New game"
    }
    pub fn waiting_for_opponent() -> &'static str {
        "Waiting for opponent"
    }
    pub fn player_local_label() -> &'static str {
        "You"
    }
    pub fn player_remote_label() -> &'static str {
        "Remote"
    }
    pub fn mines_label() -> &'static str {
        "mines"
    }
    pub fn difficulty_easy() -> &'static str {
        "Easy"
    }
    pub fn difficulty_medium() -> &'static str {
        "Medium"
    }
    pub fn difficulty_hard() -> &'static str {
        "Hard"
    }
    pub fn join_button() -> &'static str {
        "Join"
    }
    pub fn cancel_button() -> &'static str {
        "Cancel"
    }
    pub fn no_games_available() -> &'static str {
        "No games available"
    }
    pub fn new_game_title() -> &'static str {
        "Create a new game"
    }
    pub fn your_name() -> &'static str {
        "Enter your name"
    }
    pub fn difficulty_label() -> &'static str {
        "Choose difficulty level"
    }
    pub fn create_game_button() -> &'static str {
        "Create game"
    }
    pub fn board_label() -> &'static str {
        "Board"
    }
    pub fn first_to_label() -> &'static str {
        "Mines to win"
    }
    pub fn join_game_title() -> &'static str {
        "Join game"
    }
    pub fn join_as_label() -> &'static str {
        "Join as..."
    }
}
