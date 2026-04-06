mod gui;
mod networking;

use minesweeper_multiplayer::{Difficulty, Multiplayer};
use networking::*;

use gui::gameplay::MinesBoomer;

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();
    wasm_bindgen_futures::spawn_local(async {
        use eframe::{wasm_bindgen::JsCast, web_sys};

        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("the_canvas_id").unwrap().dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

        let app = make_app();
        eframe::WebRunner::new()
            .start(canvas, eframe::WebOptions::default(), Box::new(|_cc| Ok(Box::new(app))))
            .await
            .expect("failed to start eframe");
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    let app = make_app();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("MinesBooMer", native_options, Box::new(|_| Ok(Box::new(app))))
}

fn make_app() -> MinesBoomer {
    let options = ewebsock::Options::default();
    let (sender, receiver) = ewebsock::connect("ws://localhost:8000", options).unwrap();

    let game = Multiplayer::new(["Player 1", "Player 2"], Difficulty::Easy);

    let client = WSClient::new(sender, receiver);

    MinesBoomer::new(client, game)
}
