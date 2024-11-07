//! Types for defining shape color and options.

use bevy::{color::Color, ecs::component::Component};
use lyon_tessellation::{FillOptions, StrokeOptions};

/// Defines the fill options for the lyon tessellator and color of the generated
/// vertices.
#[allow(missing_docs)]
#[derive(Component, Debug, Clone, PartialEq)]
pub struct Fill {
    pub options: FillOptions,
    pub color: Color,
    pub resource_id: Option<String>,
    pub disabled: Option<bool>,
}

impl Fill {
    /// Convenience constructor requiring only the `Color`.
    #[must_use]
    pub fn color(color: impl Into<Color>) -> Self {
        Self {
            options: FillOptions::default(),
            color: color.into(),
            resource_id: None,
            disabled: None
        }
    }
    #[must_use]
    pub fn new_by_resource_id(resource_id: String) -> Self {
        Self {
            options: FillOptions::default(),
            color: Color::WHITE,
            resource_id: Some(resource_id),
            disabled: None
        }
    }
    /// Convenience constructor requiring only the `Color`.
    #[must_use]
    pub fn disabled() -> Self {
        Self {
            options: FillOptions::default(),
            color: Color::WHITE,
            resource_id: None,
            disabled: Option::from(true)
        }
    }
}

/// Defines the stroke options for the lyon tessellator and color of the
/// generated vertices.
#[allow(missing_docs)]
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Stroke {
    pub options: StrokeOptions,
    pub color: Color,
    pub disabled: Option<bool>,
}

impl Stroke {
    /// Constructor that requires a `Color` and a line width.
    #[must_use]
    pub fn new(color: impl Into<Color>, line_width: f32) -> Self {
        Self {
            options: StrokeOptions::default().with_line_width(line_width),
            color: color.into(),
            disabled: None,
        }
    }

    /// Convenience constructor requiring only the `Color`.
    #[must_use]
    pub fn color(color: impl Into<Color>) -> Self {
        Self {
            options: StrokeOptions::default(),
            color: color.into(),
            disabled: None,
        }
    }

    #[must_use]
    pub fn disabled() -> Self {
        Self {
            options: StrokeOptions::default(),
            color: Color::BLACK,
            disabled: Option::from(true)
        }
    }
}
