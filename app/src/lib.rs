#![recursion_limit = "256"]
mod app;
mod components;
use app::App;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
