use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct App2DOptions {
    #[serde(rename = "canvasId")]
    pub canvas_id: String,
}
