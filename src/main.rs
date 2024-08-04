use rs_wgpu_engine::run;

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
fn main() {
    println!("hello rust.");
    run();
}
