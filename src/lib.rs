mod app;
mod core;
mod materials;
mod math;
mod objects;
mod render;
mod utils;
use app::application::{Application};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    let mut app = Application::new();
    app.start().await;
}
