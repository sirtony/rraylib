use crate::sys::*;
use crate::{try_lock, Error, Result};
use std::sync::{Mutex, MutexGuard};

pub use crate::sys::{Camera2D, Camera3D, Color};

pub struct Drawing<'a> {
    _guard: MutexGuard<'a, ()>,
    drawing: Mutex<()>,
}

impl<'a> Drawing<'a> {
    pub(crate) fn new(guard: MutexGuard<'a, ()>, drawing: Mutex<()>) -> Self {
        Self {
            _guard: guard,
            drawing,
        }
    }

    pub fn clear_background(&mut self, color: Color) {
        unsafe { clear_background(color) }
    }

    pub fn draw_fps(&mut self, x: i32, y: i32) {
        unsafe { draw_fps(x, y) }
    }

    pub fn begin_mode2d(&mut self, camera: &Camera2D) -> Result<Drawing2D> {
        let guard = try_lock!(self.drawing).ok_or(Error::ThreadAlreadyLocked("drawing"))?;
        unsafe { begin_mode_2d(*camera) }
        Ok(Drawing2D(guard))
    }

    pub fn draw2d<F>(&mut self, camera: &Camera2D, f: F) -> Result<()>
    where
        F: FnOnce(&mut Drawing2D),
    {
        let mut ctx = self.begin_mode2d(camera)?;
        f(&mut ctx);
        Ok(())
    }

    pub fn begin_mode3d(&mut self, camera: &Camera3D) -> Result<Drawing3D> {
        let guard = try_lock!(self.drawing).ok_or(Error::ThreadAlreadyLocked("drawing"))?;
        unsafe { begin_mode_3d(*camera) }
        Ok(Drawing3D(guard))
    }

    pub fn draw3d<F>(&mut self, camera: &Camera3D, f: F) -> Result<()>
    where
        F: FnOnce(&mut Drawing3D),
    {
        let mut ctx = self.begin_mode3d(camera)?;
        f(&mut ctx);
        Ok(())
    }

    pub fn begin_texture_mode(&mut self, texture: &RenderTexture2D) -> Result<DrawingTexture> {
        let guard = try_lock!(self.drawing).ok_or(Error::ThreadAlreadyLocked("drawing"))?;
        unsafe { begin_texture_mode(*texture) }
        Ok(DrawingTexture(guard))
    }

    pub fn draw_texture<F>(&mut self, texture: &RenderTexture2D, f: F) -> Result<()>
    where
        F: FnOnce(&mut DrawingTexture),
    {
        let mut ctx = self.begin_texture_mode(texture)?;
        f(&mut ctx);
        Ok(())
    }

    pub fn begin_shader_mode(&mut self, shader: &Shader) -> Result<DrawingShaded> {
        let guard = try_lock!(self.drawing).ok_or(Error::ThreadAlreadyLocked("drawing"))?;
        unsafe { begin_shader_mode(*shader) }
        Ok(DrawingShaded(guard))
    }

    pub fn draw_shader<F>(&mut self, shader: &Shader, f: F) -> Result<()>
    where
        F: FnOnce(&mut DrawingShaded),
    {
        let mut ctx = self.begin_shader_mode(shader)?;
        f(&mut ctx);
        Ok(())
    }

    pub fn begin_blend_mode(&mut self, mode: BlendMode) -> Result<DrawingBlended> {
        let guard = try_lock!(self.drawing).ok_or(Error::ThreadAlreadyLocked("drawing"))?;
        unsafe { begin_blend_mode(mode as i32) }
        Ok(DrawingBlended(guard))
    }

    pub fn draw_blend<F>(&mut self, mode: BlendMode, f: F) -> Result<()>
    where
        F: FnOnce(&mut DrawingBlended),
    {
        let mut ctx = self.begin_blend_mode(mode)?;
        f(&mut ctx);
        Ok(())
    }

    pub fn begin_viewport_mode(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<DrawingViewport> {
        let guard = try_lock!(self.drawing).ok_or(Error::ThreadAlreadyLocked("drawing"))?;
        unsafe { begin_scissor_mode(x, y, width, height) }
        Ok(DrawingViewport(guard))
    }

    pub fn begin_viewport_mode_rect(&mut self, viewport: &Rectangle) -> Result<DrawingViewport> {
        self.begin_viewport_mode(
            viewport.x as i32,
            viewport.y as i32,
            viewport.width as i32,
            viewport.height as i32,
        )
    }

    pub fn begin_viewport_mode_vec4(&mut self, viewport: &Vector4) -> Result<DrawingViewport> {
        self.begin_viewport_mode(
            viewport.x as i32,
            viewport.y as i32,
            viewport.z as i32,
            viewport.w as i32,
        )
    }

    pub fn begin_viewport_mode_vec2(
        &mut self,
        location: &Vector2,
        size: &Vector2,
    ) -> Result<DrawingViewport> {
        self.begin_viewport_mode(
            location.x as i32,
            location.y as i32,
            size.x as i32,
            size.y as i32,
        )
    }

    pub fn draw_viewport<F>(&mut self, x: i32, y: i32, width: i32, height: i32, f: F) -> Result<()>
    where
        F: FnOnce(&mut DrawingViewport),
    {
        let mut ctx = self.begin_viewport_mode(x, y, width, height)?;
        f(&mut ctx);
        Ok(())
    }

    pub fn draw_viewport_rect<F>(&mut self, viewport: &Rectangle, f: F) -> Result<()>
    where
        F: FnOnce(&mut DrawingViewport),
    {
        self.draw_viewport(
            viewport.x as i32,
            viewport.y as i32,
            viewport.width as i32,
            viewport.height as i32,
            f,
        )
    }

    pub fn draw_viewport_vec4<F>(&mut self, viewport: &Vector4, f: F) -> Result<()>
    where
        F: FnOnce(&mut DrawingViewport),
    {
        self.draw_viewport(
            viewport.x as i32,
            viewport.y as i32,
            viewport.z as i32,
            viewport.w as i32,
            f,
        )
    }

    pub fn draw_viewport_vec2<F>(&mut self, location: &Vector2, size: &Vector2, f: F) -> Result<()>
    where
        F: FnOnce(&mut DrawingViewport),
    {
        self.draw_viewport(
            location.x as i32,
            location.y as i32,
            size.x as i32,
            size.y as i32,
            f,
        )
    }

    pub fn begin_vr_stereo_mode(&mut self, config: VrStereoConfig) -> Result<DrawingVr> {
        let guard = try_lock!(self.drawing).ok_or(Error::ThreadAlreadyLocked("drawing"))?;
        unsafe { begin_vr_stereo_mode(config) }
        Ok(DrawingVr(guard))
    }

    pub fn draw_vr<F>(&mut self, config: VrStereoConfig, f: F) -> Result<()>
    where
        F: FnOnce(&mut DrawingVr),
    {
        let mut ctx = self.begin_vr_stereo_mode(config)?;
        f(&mut ctx);
        Ok(())
    }
}

impl<'a> Drop for Drawing<'a> {
    fn drop(&mut self) {
        unsafe { end_drawing() }
    }
}

pub struct Drawing2D<'a>(MutexGuard<'a, ()>);

impl Drop for Drawing2D<'_> {
    fn drop(&mut self) {
        unsafe { end_mode_2d() }
    }
}

pub struct Drawing3D<'a>(MutexGuard<'a, ()>);

impl Drop for Drawing3D<'_> {
    fn drop(&mut self) {
        unsafe { end_mode_3d() }
    }
}

pub struct DrawingTexture<'a>(MutexGuard<'a, ()>);

impl Drop for DrawingTexture<'_> {
    fn drop(&mut self) {
        unsafe { end_texture_mode() }
    }
}

pub struct DrawingShaded<'a>(MutexGuard<'a, ()>);

impl Drop for DrawingShaded<'_> {
    fn drop(&mut self) {
        unsafe { end_shader_mode() }
    }
}

pub struct DrawingBlended<'a>(MutexGuard<'a, ()>);

impl Drop for DrawingBlended<'_> {
    fn drop(&mut self) {
        unsafe { end_blend_mode() }
    }
}

pub struct DrawingViewport<'a>(MutexGuard<'a, ()>);

impl Drop for DrawingViewport<'_> {
    fn drop(&mut self) {
        unsafe { end_scissor_mode() }
    }
}

pub struct DrawingVr<'a>(MutexGuard<'a, ()>);

impl Drop for DrawingVr<'_> {
    fn drop(&mut self) {
        unsafe { end_vr_stereo_mode() }
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
        unsafe { fade(*self, alpha) }
    }

    pub fn normalize(&self) -> Vector4 {
        unsafe { color_normalize(*self) }
    }

    pub fn as_hsv(&self) -> Vector3 {
        unsafe { color_to_hsv(*self) }
    }

    pub fn from_hsv(hsv: Vector3) -> Self {
        unsafe { color_from_hsv(hsv.x, hsv.y, hsv.z) }
    }

    pub fn tint(&self, tint: &Color) -> Self {
        unsafe { color_tint(*self, *tint) }
    }

    pub fn brightness(&self, factor: f32) -> Self {
        unsafe { color_brightness(*self, factor) }
    }

    pub fn contrast(&self, contrast: f32) -> Self {
        unsafe { color_contrast(*self, contrast) }
    }

    pub fn opacity(&self, alpha: f32) -> Self {
        unsafe { color_alpha(*self, alpha) }
    }

    pub fn alpha_blend(&self, src: &Color, tint: &Color) -> Self {
        unsafe { color_alpha_blend(*self, *src, *tint) }
    }

    pub fn lerp(&self, target: &Color, amount: f32) -> Self {
        unsafe { color_lerp(*self, *target, amount) }
    }
}

impl From<i32> for Color {
    fn from(color: i32) -> Self {
        unsafe { get_color(color as u32) }
    }
}

impl Into<i32> for Color {
    fn into(self) -> i32 {
        unsafe { color_to_int(self) }
    }
}

impl From<u32> for Color {
    fn from(color: u32) -> Self {
        unsafe { get_color(color) }
    }
}

impl Into<u32> for Color {
    fn into(self) -> u32 {
        unsafe { color_to_int(self) as u32 }
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
