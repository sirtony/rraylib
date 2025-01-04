use crate::sys::*;
use crate::{try_lock, Error, Result};
use std::sync::{Mutex, MutexGuard};

pub struct Drawing<'a> {
    _guard: MutexGuard<'a, ()>,
    drawing: Mutex<()>,
}
pub struct Drawing2D<'a>(MutexGuard<'a, ()>);
pub struct Drawing3D<'a>(MutexGuard<'a, ()>);

impl<'a> Drawing<'a> {
    pub(crate) fn new(guard: MutexGuard<'a, ()>, drawing: Mutex<()>) -> Self {
        Self {
            _guard: guard,
            drawing,
        }
    }

    pub fn clear_background(&self, color: Color) {
        unsafe { ClearBackground(color) }
    }

    pub fn draw_fps(&self, x: i32, y: i32) {
        unsafe { DrawFPS(x, y) }
    }

    pub fn begin_mode2d(&self, camera: &Camera2D) -> Result<Drawing2D> {
        let guard = try_lock!(self.drawing).ok_or(Error::ThreadAlreadyLocked("drawing"))?;
        unsafe { BeginMode2D(*camera) }
        Ok(Drawing2D(guard))
    }

    pub fn begin_mode3d(&self, camera: &Camera3D) -> Result<Drawing3D> {
        let guard = try_lock!(self.drawing).ok_or(Error::ThreadAlreadyLocked("drawing"))?;
        unsafe { BeginMode3D(*camera) }
        Ok(Drawing3D(guard))
    }
}

impl<'a> Drop for Drawing<'a> {
    fn drop(&mut self) {
        unsafe { EndDrawing() }
    }
}

impl Color {
    pub const LIGHT_GRAY: Self = Self {
        r: 200,
        g: 200,
        b: 200,
        a: 255,
    };
    pub const GRAY: Self = Self {
        r: 130,
        g: 130,
        b: 130,
        a: 255,
    };
    pub const DARK_GRAY: Self = Self {
        r: 80,
        g: 80,
        b: 80,
        a: 255,
    };
    pub const YELLOW: Self = Self {
        r: 253,
        g: 249,
        b: 0,
        a: 255,
    };
    pub const GOLD: Self = Self {
        r: 255,
        g: 203,
        b: 0,
        a: 255,
    };
    pub const ORANGE: Self = Self {
        r: 255,
        g: 161,
        b: 0,
        a: 255,
    };
    pub const PINK: Self = Self {
        r: 255,
        g: 109,
        b: 194,
        a: 255,
    };
    pub const RED: Self = Self {
        r: 230,
        g: 41,
        b: 55,
        a: 255,
    };
    pub const MAROON: Self = Self {
        r: 190,
        g: 33,
        b: 55,
        a: 255,
    };
    pub const GREEN: Self = Self {
        r: 0,
        g: 228,
        b: 48,
        a: 255,
    };
    pub const LIME: Self = Self {
        r: 0,
        g: 158,
        b: 47,
        a: 255,
    };
    pub const DARK_GREEN: Self = Self {
        r: 0,
        g: 117,
        b: 44,
        a: 255,
    };
    pub const SKY_BLUE: Self = Self {
        r: 102,
        g: 191,
        b: 255,
        a: 255,
    };
    pub const BLUE: Self = Self {
        r: 0,
        g: 121,
        b: 241,
        a: 255,
    };
    pub const DARK_BLUE: Self = Self {
        r: 0,
        g: 82,
        b: 172,
        a: 255,
    };
    pub const PURPLE: Self = Self {
        r: 200,
        g: 122,
        b: 255,
        a: 255,
    };
    pub const VIOLET: Self = Self {
        r: 135,
        g: 60,
        b: 190,
        a: 255,
    };
    pub const DARK_PURPLE: Self = Self {
        r: 112,
        g: 31,
        b: 126,
        a: 255,
    };
    pub const BEIGE: Self = Self {
        r: 211,
        g: 176,
        b: 131,
        a: 255,
    };
    pub const BROWN: Self = Self {
        r: 127,
        g: 106,
        b: 79,
        a: 255,
    };
    pub const DARK_BROWN: Self = Self {
        r: 76,
        g: 63,
        b: 47,
        a: 255,
    };
    pub const WHITE: Self = Self {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const BLACK: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const BLANK: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
    pub const MAGENTA: Self = Self {
        r: 255,
        g: 0,
        b: 255,
        a: 255,
    };
    pub const RAY_WHITE: Self = Self {
        r: 245,
        g: 245,
        b: 245,
        a: 255,
    };

    pub fn fade(&self, alpha: f32) -> Self {
        unsafe { Fade(*self, alpha) }
    }

    pub fn normalize(&self) -> Vector4 {
        unsafe { ColorNormalize(*self) }
    }

    pub fn as_hsv(&self) -> Vector3 {
        unsafe { ColorToHSV(*self) }
    }

    pub fn from_hsv(hsv: Vector3) -> Self {
        unsafe { ColorFromHSV(hsv.x, hsv.y, hsv.z) }
    }

    pub fn tint(&self, tint: &Color) -> Self {
        unsafe { ColorTint(*self, *tint) }
    }

    pub fn brightness(&self, factor: f32) -> Self {
        unsafe { ColorBrightness(*self, factor) }
    }

    pub fn contrast(&self, contrast: f32) -> Self {
        unsafe { ColorContrast(*self, contrast) }
    }

    pub fn opacity(&self, alpha: f32) -> Self {
        unsafe { ColorAlpha(*self, alpha) }
    }

    pub fn alpha_blend(&self, src: &Color, tint: &Color) -> Self {
        unsafe { ColorAlphaBlend(*self, *src, *tint) }
    }

    pub fn lerp(&self, target: &Color, amount: f32) -> Self {
        unsafe { ColorLerp(*self, *target, amount) }
    }
}

impl From<i32> for Color {
    fn from(color: i32) -> Self {
        unsafe { GetColor(color as u32) }
    }
}

impl Into<i32> for Color {
    fn into(self) -> i32 {
        unsafe { ColorToInt(self) }
    }
}

impl From<u32> for Color {
    fn from(color: u32) -> Self {
        unsafe { GetColor(color) }
    }
}

impl Into<u32> for Color {
    fn into(self) -> u32 {
        unsafe { ColorToInt(self) as u32 }
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        Self { r, g, b, a }
    }
}

impl Into<(u8, u8, u8, u8)> for Color {
    fn into(self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self { r, g, b, a: 255 }
    }
}

impl Into<(u8, u8, u8)> for Color {
    fn into(self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
}

impl Eq for Color {}
impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
    }
}
