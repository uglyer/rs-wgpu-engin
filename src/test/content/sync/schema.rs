use bevy::prelude::*;
use serde_derive::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub enum SyncMethod {
    GetCmd,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug)]
pub enum SyncCmdType {
    Resize,
    Destroy,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SyncCmd {
    pub t: SyncCmdType,
    pub d: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SyncVec2 {
    pub x: f32,
    pub y: f32,
}

impl SyncCmd {
    pub fn as_vec2(&self) -> Vec2 {
        let vec2: SyncVec2 = serde_json::from_str(&self.d).unwrap();
        Vec2::new(vec2.x, vec2.y)
    }
}
