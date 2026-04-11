mod gui;
mod networking;
use networking::*;

use gui::gameplay::MinesBoomer;

#[cfg(target_arch = "wasm32")]
use eframe::{wasm_bindgen::JsCast, web_sys};

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();
    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document
            .get_element_by_id("the_canvas_id")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();

        let app = make_app();
        eframe::WebRunner::new()
            .start(canvas, eframe::WebOptions::default(), Box::new(|_cc| Ok(Box::new(app))))
            .await
            .expect("failed to start eframe");
    });
}

#[cfg(target_arch = "wasm32")]
fn get_ws_url() -> String {
    if cfg!(debug_assertions) {
        "ws://localhost:8080/ws".to_string()
    } else {
        let window = web_sys::window().unwrap();
        let location = window.location();
        let host = location.host().unwrap();
        let protocol = if location.protocol().unwrap() == "https:" {
            "wss"
        } else {
            "ws"
        };
        format!("{protocol}://{host}/ws")
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    let app = make_app();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("MinesBooMer", native_options, Box::new(|_| Ok(Box::new(app))))
}

#[cfg(not(target_arch = "wasm32"))]
fn get_ws_url() -> String {
    "ws://localhost:8080/ws".to_string()
}

fn make_app() -> MinesBoomer {
    let options = ewebsock::Options::default();
    let (sender, receiver) = ewebsock::connect(get_ws_url(), options).unwrap();
    let client = WSClient::new(sender, receiver);
    MinesBoomer::new(client)

    // testing Game UI fast
    // let mut boomer = MinesBoomer::new(client);
    // let game = Multiplayer::new(
    //     "Local Player",
    //     "Remote Player",
    //     minesweeper_multiplayer::Difficulty::Medium,
    // );
    // boomer.game = Some(game);
    // boomer
}
