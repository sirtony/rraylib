/// 3D drawing and rendering.
pub mod drawing3d;

/// 2D drawing and rendering.
pub mod drawing2d;

/// Core drawing and rendering functionality.
pub mod core;
/// 2D and 3D shapes and collision detection.
pub mod shapes;

pub use crate::graphics::core::*;
pub use crate::graphics::drawing2d::*;
pub use crate::graphics::drawing3d::*;
pub use crate::graphics::shapes::*;
pub use crate::sys::{Camera2D, Camera3D, CameraMode, CameraProjection, Color};
