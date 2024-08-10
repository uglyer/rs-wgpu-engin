use winit::{
    event::*,
    event_loop::EventLoop,
    window::WindowBuilder,
};
mod app;
mod render;
use app::application::{Application};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    let mut app = Application::new().await;
    app.start().await;
}
