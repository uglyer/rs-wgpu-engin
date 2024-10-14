pub mod app2d;
pub mod utils;
pub mod content;
pub mod ffi;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn init() {
    // cfg_if::cfg_if! {
    //     if #[cfg(target_arch = "wasm32")] {
    //         std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    //         console_log::init_with_level(log::Level::Info).expect("Couldn't initialize logger");
    //     } else {
    //         env_logger::builder()
    //             .format_timestamp(Some(env_logger::TimestampPrecision::Millis))
    //             .filter_level(log::LevelFilter::Info)
    //             .init();
    //     }
    // }
}

