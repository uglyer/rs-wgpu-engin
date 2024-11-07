//! Custom Bevy ECS bundle for shapes.

use bevy::{prelude::*, sprite::Mesh2dHandle};
use lyon_tessellation::{self as tess};
use crate::assets::shaders::background_image_material_2d::BackgroundImageMaterial2d;
use crate::content::shape::{geometry::Geometry, plugin::COLOR_MATERIAL_HANDLE};

/// A Bevy `Bundle` to represent a shape.
#[allow(missing_docs)]
#[derive(Bundle, Clone, Debug)]
pub struct ShapeBundle {
    pub path: Path,
    pub mesh: Mesh2dHandle,
    pub material: Handle<BackgroundImageMaterial2d>,
    pub spatial: SpatialBundle,
}

impl Default for ShapeBundle {
    fn default() -> Self {
        Self {
            path: default(),
            mesh: default(),
            material: COLOR_MATERIAL_HANDLE,
            spatial: default(),
        }
    }
}

#[allow(missing_docs)]
#[derive(Component, Default, Clone, Debug)]
pub struct Path(pub tess::path::Path);

impl Geometry for Path {
    fn add_geometry(&self, b: &mut tess::path::path::Builder) {
        b.extend_from_paths(&[self.0.as_slice()]);
    }
}
