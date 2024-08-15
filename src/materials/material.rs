use std::collections::HashMap;

pub enum Side {
    BackSide,
    DoubleSide,
    FrontSide,
}

impl Default for Side {
    fn default() -> Self {
        Side::FrontSide
    }
}

pub struct Material {
    color: [f32; 4],
    side: Side,
}

impl Material {
    pub fn new_basic_color(
        color: [f32; 4],
        side: Side,
    ) -> Self {
        Material {
            color,
            side,
        }
    }

    pub fn borrow_side(&self) -> &Side {
        &self.side
    }
}
