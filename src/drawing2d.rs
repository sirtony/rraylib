#![allow(clippy::too_many_arguments)]

use crate::error::Error;
use crate::graphics::Drawing;
use crate::shapes::Shape2D;
use crate::sys::*;
use crate::{newtype, try_lock};
use std::ffi::{c_void, CString};
use std::path::Path;
use std::ptr::{addr_of, addr_of_mut};
use std::sync::MutexGuard;

newtype!(Texture, unload_texture);
newtype!(RenderTexture, unload_render_texture);

pub trait Drawables2D {
    fn draw_shape<'t>(
        &mut self,
        shape: impl Into<Shape2D<'t>>,
        color: impl Into<Color>,
    ) -> crate::error::Result<()> {
        let color = color.into();
        match shape.into() {
            Shape2D::Pixel(p) => {
                unsafe { draw_pixel(p.x as i32, p.y as i32, color) };
                Ok(())
            }
            Shape2D::Line {
                start,
                end,
                thickness,
            } => {
                if let Some(thickness) = thickness {
                    unsafe { draw_line_ex(start, end, thickness, color) }
                } else {
                    unsafe { draw_line_v(start, end, color) }
                };
                Ok(())
            }
            Shape2D::LineStrip(points) => {
                let ptr = points.as_ptr();
                unsafe { draw_line_strip(ptr, points.len() as i32, color) };
                Ok(())
            }
            Shape2D::Bezier {
                start,
                end,
                thickness,
            } => {
                unsafe { draw_line_bezier(start, end, thickness, color) };
                Ok(())
            }
            Shape2D::Circle { center, radius } => {
                unsafe { draw_circle_v(center, radius, color) };
                Ok(())
            }
            Shape2D::Pie {
                center,
                radius,
                start_angle,
                end_angle,
                segments,
            } => unsafe {
                draw_circle_sector(
                    center,
                    radius,
                    start_angle,
                    end_angle,
                    segments as i32,
                    color,
                );
                Ok(())
            },
            Shape2D::Ellipse { center, radius } => {
                unsafe {
                    draw_ellipse(center.x as i32, center.y as i32, radius.x, radius.y, color)
                };
                Ok(())
            }
            Shape2D::Ring {
                center,
                inner_radius,
                outer_radius,
                angle,
                segments,
            } => {
                unsafe {
                    draw_ring(
                        center,
                        inner_radius,
                        outer_radius,
                        angle.x,
                        angle.y,
                        segments as i32,
                        color,
                    )
                };
                Ok(())
            }
            Shape2D::Rectangle { rect, rotation } => {
                if let Some(rotation) = rotation {
                    unsafe { draw_rectangle_pro(rect, rect.centerv(), rotation, color) }
                } else {
                    unsafe { draw_rectangle_rec(rect, color) }
                };
                Ok(())
            }
            Shape2D::RoundedRectangle {
                rect,
                roundness,
                segments,
            } => {
                unsafe { draw_rectangle_rounded(rect, roundness, segments, color) };
                Ok(())
            }
            Shape2D::Triangle { v1, v2, v3 } => {
                unsafe { draw_triangle(v1, v2, v3, color) };
                Ok(())
            }
            Shape2D::TriangleFan(points) => {
                let ptr = points.as_ptr();
                unsafe { draw_triangle_fan(ptr, points.len() as i32, color) };
                Ok(())
            }
            Shape2D::TriangleStrip(points) => {
                let ptr = points.as_ptr();
                unsafe { draw_triangle_strip(ptr, points.len() as i32, color) };
                Ok(())
            }
            Shape2D::Polygon {
                center,
                sides,
                radius,
                rotation,
            } => {
                unsafe { draw_poly(center, sides, radius, rotation, color) };
                Ok(())
            }
            Shape2D::Spline {
                spline_type,
                points,
                thickness,
            } => {
                spline_type.draw(points.iter().copied(), thickness, color);
                Ok(())
            }
        }
    }

    fn draw_lines<'t>(
        &mut self,
        shape: impl Into<Shape2D<'t>>,
        line_thickness: Option<f32>,
        color: impl Into<Color>,
    ) -> crate::error::Result<()> {
        let line_thickness = line_thickness.unwrap_or(1.0);
        let color = color.into();
        match shape.into() {
            Shape2D::Pixel(p) => {
                unsafe { draw_pixel(p.x as i32, p.y as i32, color) };
                Ok(())
            }
            Shape2D::Line {
                start,
                end,
                thickness,
            } => {
                if let Some(thickness) = thickness {
                    unsafe { draw_line_ex(start, end, thickness, color) }
                } else {
                    unsafe { draw_line_v(start, end, color) }
                };
                Ok(())
            }
            Shape2D::LineStrip(points) => {
                let ptr = points.as_ptr();
                unsafe { draw_line_strip(ptr, points.len() as i32, color) };
                Ok(())
            }
            Shape2D::Bezier {
                start,
                end,
                thickness,
            } => {
                unsafe { draw_line_bezier(start, end, thickness, color) };
                Ok(())
            }
            Shape2D::Circle { center, radius } => {
                unsafe { draw_circle_lines_v(center, radius, color) };
                Ok(())
            }
            Shape2D::Pie {
                center,
                radius,
                start_angle,
                end_angle,
                segments,
            } => {
                unsafe {
                    draw_circle_sector_lines(
                        center,
                        radius,
                        start_angle,
                        end_angle,
                        segments as i32,
                        color,
                    )
                };
                Ok(())
            }
            Shape2D::Ellipse { center, radius } => {
                unsafe {
                    draw_ellipse_lines(center.x as i32, center.y as i32, radius.x, radius.y, color)
                };
                Ok(())
            }
            Shape2D::Ring {
                center,
                inner_radius,
                outer_radius,
                angle,
                segments,
            } => {
                unsafe {
                    draw_ring_lines(
                        center,
                        inner_radius,
                        outer_radius,
                        angle.x,
                        angle.y,
                        segments as i32,
                        color,
                    )
                };
                Ok(())
            }
            Shape2D::Rectangle { rect, rotation } => {
                if rotation.is_some() {
                    Err(Error::OperationNotSupported {
                        verb: "line drawing",
                        noun: "rotated rectangles",
                    })
                } else {
                    unsafe { draw_rectangle_lines_ex(rect, line_thickness, color) };
                    Ok(())
                }
            }
            Shape2D::RoundedRectangle {
                rect,
                roundness,
                segments,
            } => {
                unsafe {
                    draw_rectangle_rounded_lines_ex(
                        rect,
                        roundness,
                        segments,
                        line_thickness,
                        color,
                    )
                };
                Ok(())
            }
            Shape2D::Triangle { v1, v2, v3 } => {
                unsafe { draw_triangle_lines(v1, v2, v3, color) };
                Ok(())
            }
            Shape2D::TriangleFan(_) => Err(Error::OperationNotSupported {
                verb: "line drawing",
                noun: "triangle fans",
            }),
            Shape2D::TriangleStrip(_) => Err(Error::OperationNotSupported {
                verb: "line drawing",
                noun: "triangle strips",
            }),
            Shape2D::Polygon {
                center,
                sides,
                radius,
                rotation,
            } => {
                unsafe {
                    draw_poly_lines_ex(center, sides, radius, rotation, line_thickness, color)
                };
                Ok(())
            }
            Shape2D::Spline {
                spline_type,
                points,
                thickness,
            } => {
                spline_type.draw(points.iter().copied(), thickness, color);
                Ok(())
            }
        }
    }

    fn draw_gradient_h<'t>(
        &mut self,
        shape: impl Into<Shape2D<'t>>,
        start_color: impl Into<Color>,
        end_color: impl Into<Color>,
    ) -> crate::error::Result<()> {
        let start_color = start_color.into();
        let end_color = end_color.into();

        match shape.into() {
            Shape2D::Rectangle { rect, rotation } => {
                if rotation.is_some() {
                    Err(Error::OperationNotSupported {
                        verb: "gradient drawing",
                        noun: "rotated rectangles",
                    })
                } else {
                    unsafe {
                        draw_rectangle_gradient_h(
                            rect.x as i32,
                            rect.y as i32,
                            rect.width as i32,
                            rect.height as i32,
                            start_color,
                            end_color,
                        )
                    };
                    Ok(())
                }
            }
            Shape2D::Circle { center, radius } => {
                unsafe {
                    draw_circle_gradient(
                        center.x as i32,
                        center.y as i32,
                        radius,
                        start_color,
                        end_color,
                    )
                };
                Ok(())
            }
            _ => Err(Error::OperationNotSupported {
                verb: "gradient drawing",
                noun: "non-rectangle/non-circle shapes",
            }),
        }
    }

    fn draw_gradient_v<'t>(
        &mut self,
        shape: impl Into<Shape2D<'t>>,
        start_color: impl Into<Color>,
        end_color: impl Into<Color>,
    ) -> crate::error::Result<()> {
        let start_color = start_color.into();
        let end_color = end_color.into();

        match shape.into() {
            Shape2D::Rectangle { rect, rotation } => {
                if rotation.is_some() {
                    Err(Error::OperationNotSupported {
                        verb: "gradient drawing",
                        noun: "rotated rectangles",
                    })
                } else {
                    unsafe {
                        draw_rectangle_gradient_v(
                            rect.x as i32,
                            rect.y as i32,
                            rect.width as i32,
                            rect.height as i32,
                            start_color,
                            end_color,
                        )
                    };
                    Ok(())
                }
            }
            Shape2D::Circle { center, radius } => {
                unsafe {
                    draw_circle_gradient(
                        center.x as i32,
                        center.y as i32,
                        radius,
                        start_color,
                        end_color,
                    )
                };
                Ok(())
            }
            _ => Err(Error::OperationNotSupported {
                verb: "gradient drawing",
                noun: "non-rectangle/non-circle shapes",
            }),
        }
    }

    fn draw_texture(
        &mut self,
        texture: &Texture,
        position: impl Into<Vector2>,
        tint: impl Into<Color>,
    ) -> crate::error::Result<()> {
        let position = position.into();
        unsafe { draw_texture_v(texture.as_raw(), position, tint.into()) };
        Ok(())
    }

    fn draw_texture_ex(
        &mut self,
        texture: &Texture,
        position: impl Into<Vector2>,
        rotation: f32,
        scale: f32,
        tint: impl Into<Color>,
    ) -> crate::error::Result<()> {
        let position = position.into();
        unsafe { draw_texture_ex(texture.as_raw(), position, rotation, scale, tint.into()) };
        Ok(())
    }

    fn draw_texture_clipped(
        &mut self,
        texture: &Texture,
        src: impl Into<Rectangle>,
        pos: impl Into<Vector2>,
        tint: impl Into<Color>,
    ) -> crate::error::Result<()> {
        unsafe { draw_texture_rec(texture.as_raw(), src.into(), pos.into(), tint.into()) };
        Ok(())
    }

    fn draw_texture_rotated(
        &mut self,
        texture: &Texture,
        src: impl Into<Rectangle>,
        dst: impl Into<Rectangle>,
        origin: impl Into<Vector2>,
        rotation: f32,
        tint: impl Into<Color>,
    ) -> crate::error::Result<()> {
        unsafe {
            draw_texture_pro(
                texture.as_raw(),
                src.into(),
                dst.into(),
                origin.into(),
                rotation,
                tint.into(),
            )
        };
        Ok(())
    }

    fn draw_texture_npatched(
        &mut self,
        texture: &Texture,
        info: impl Into<NPatchInfo>,
        dst: impl Into<Rectangle>,
        origin: impl Into<Vector2>,
        rotation: f32,
        tint: impl Into<Color>,
    ) -> crate::error::Result<()> {
        unsafe {
            draw_texture_n_patch(
                texture.as_raw(),
                info.into(),
                dst.into(),
                origin.into(),
                rotation,
                tint.into(),
            )
        };
        Ok(())
    }

    fn draw_text(
        &mut self,
        text: impl AsRef<str>,
        pos: impl Into<Vector2>,
        font_size: u32,
        color: impl Into<Color>,
    ) -> crate::error::Result<()> {
        let text = text.as_ref().as_bytes();
        let text = CString::new(text)?;
        let pos = pos.into();
        let (x, y): (i32, i32) = pos.into();

        unsafe { draw_text(text.as_ptr(), x, y, font_size as i32, color.into()) };
        Ok(())
    }

    fn draw_text_ex(
        &mut self,
        font: &crate::graphics::Font,
        text: impl AsRef<str>,
        pos: impl Into<Vector2>,
        font_size: f32,
        spacing: f32,
        tint: impl Into<Color>,
    ) -> crate::error::Result<()> {
        let text = text.as_ref().as_bytes();
        let text = CString::new(text)?;

        unsafe {
            draw_text_ex(
                font.as_raw(),
                text.as_ptr(),
                pos.into(),
                font_size,
                spacing,
                tint.into(),
            )
        };
        Ok(())
    }

    fn draw_text_rotated(
        &mut self,
        font: &crate::graphics::Font,
        text: impl AsRef<str>,
        pos: impl Into<Vector2>,
        origin: impl Into<Vector2>,
        rotation: f32,
        font_size: f32,
        spacing: f32,
        tint: impl Into<Color>,
    ) -> crate::error::Result<()> {
        let text = text.as_ref().as_bytes();
        let text = CString::new(text)?;

        unsafe {
            draw_text_pro(
                font.as_raw(),
                text.as_ptr(),
                pos.into(),
                origin.into(),
                rotation,
                font_size,
                spacing,
                tint.into(),
            )
        };
        Ok(())
    }

    fn clear_background(&mut self, color: impl Into<Color>) {
        unsafe { clear_background(color.into()) }
    }
}

impl Drawing<'_> {
    pub fn draw_fps(&mut self, x: i32, y: i32) {
        unsafe { draw_fps(x, y) }
    }

    pub fn begin_mode2d(&mut self, camera: &Camera2D) -> crate::error::Result<Drawing2D> {
        let guard = try_lock!(self.drawing).ok_or(Error::ThreadAlreadyLocked("drawing"))?;
        unsafe { begin_mode_2d(*camera) }
        Ok(Drawing2D(guard))
    }

    pub fn draw2d<F>(&mut self, camera: &Camera2D, f: F) -> crate::error::Result<()>
    where
        F: FnOnce(&mut Drawing2D),
    {
        let mut ctx = self.begin_mode2d(camera)?;
        f(&mut ctx);
        Ok(())
    }

    pub fn begin_texture_mode(
        &mut self,
        texture: &RenderTexture,
    ) -> crate::error::Result<DrawingTexture> {
        let guard = try_lock!(self.drawing).ok_or(Error::ThreadAlreadyLocked("drawing"))?;
        unsafe { begin_texture_mode(texture.as_raw()) }
        Ok(DrawingTexture(guard))
    }

    pub fn draw_to_texture<F>(&mut self, texture: &RenderTexture, f: F) -> crate::error::Result<()>
    where
        F: FnOnce(&mut DrawingTexture),
    {
        let mut ctx = self.begin_texture_mode(texture)?;
        f(&mut ctx);
        Ok(())
    }
}

#[allow(dead_code)]
pub struct Drawing2D<'a>(MutexGuard<'a, ()>);

impl Drawables2D for Drawing2D<'_> {}

impl Drop for Drawing2D<'_> {
    fn drop(&mut self) {
        unsafe { end_mode_2d() }
    }
}

#[allow(dead_code)]
pub struct DrawingTexture<'a>(MutexGuard<'a, ()>);

impl Drawables2D for DrawingTexture<'_> {}

impl Drop for DrawingTexture<'_> {
    fn drop(&mut self) {
        unsafe { end_texture_mode() }
    }
}

impl RenderTexture {
    pub fn new(width: u32, height: u32) -> crate::error::Result<Self> {
        let tex = unsafe { load_render_texture(width as i32, height as i32) };
        let ptr = addr_of!(tex);
        if !unsafe { is_render_texture_valid(ptr.read()) } {
            return Err(Error::UnableToLoad("render texture"));
        }
        Ok(Self(tex))
    }
}

impl Texture {
    pub fn from_file(file_name: impl AsRef<Path>) -> crate::error::Result<Self> {
        let file_name = file_name.as_ref().as_os_str().as_encoded_bytes();
        let file_name = CString::new(file_name)?;
        let tex = unsafe { load_texture(file_name.as_ptr()) };
        let ptr = addr_of!(tex);
        if !unsafe { is_texture_valid(ptr.read()) } {
            return Err(Error::UnableToLoad("texture"));
        }
        Ok(Self(tex))
    }

    pub fn from_cubemap(
        img: &crate::graphics::Image,
        layout: CubemapLayout,
    ) -> crate::error::Result<Self> {
        let tex = unsafe { load_texture_cubemap(img.as_raw(), layout as i32) };
        let ptr = addr_of!(tex);

        if !unsafe { is_texture_valid(ptr.read()) } {
            return Err(Error::UnableToLoad("texture"));
        }
        Ok(Self(tex))
    }

    pub fn update(&mut self, pixels: impl AsRef<[u8]>) {
        let data = pixels.as_ref();
        unsafe { update_texture(self.as_raw(), data.as_ptr() as *const c_void) }
    }

    pub fn update_ex(&mut self, rec: impl Into<Rectangle>, pixels: impl AsRef<[u8]>) {
        let rec = rec.into();
        let data = pixels.as_ref();
        unsafe { update_texture_rec(self.as_raw(), rec, data.as_ptr() as *const c_void) }
    }

    pub fn compute_mipmaps(&mut self) {
        let ptr = addr_of_mut!(self.0);
        unsafe { gen_texture_mipmaps(ptr) }
    }

    pub fn filter_mode(&mut self, filter: TextureFilter) {
        unsafe { set_texture_filter(self.as_raw(), filter as i32) }
    }

    pub fn wrapping_mode(&mut self, wrap: TextureWrap) {
        unsafe { set_texture_wrap(self.as_raw(), wrap as i32) }
    }
}

impl From<crate::graphics::Image> for Texture {
    fn from(img: crate::graphics::Image) -> Self {
        Self(unsafe { load_texture_from_image(img.as_raw()) })
    }
}

impl Camera2D {
    pub fn new(offset: Vector2, target: Vector2, rotation: f32, zoom: f32) -> Self {
        Self {
            offset,
            target,
            rotation,
            zoom,
        }
    }

    pub fn offset(&self, offset: Vector2) -> Self {
        Self { offset, ..*self }
    }

    pub fn offsetf(&self, x: f32, y: f32) -> Self {
        self.offset(Vector2::new(x, y))
    }

    pub fn target(&self, target: Vector2) -> Self {
        Self { target, ..*self }
    }

    pub fn targetf(&self, x: f32, y: f32) -> Self {
        self.target(Vector2::new(x, y))
    }

    pub fn rotation(&self, rotation: f32) -> Self {
        Self { rotation, ..*self }
    }

    pub fn zoom(&self, zoom: f32) -> Self {
        Self { zoom, ..*self }
    }

    pub fn world_to_screen(&self, position: Vector2) -> Vector2 {
        unsafe { get_world_to_screen_2d(position, *self) }
    }

    pub fn screen_to_world(&self, position: Vector2) -> Vector2 {
        unsafe { get_screen_to_world_2d(position, *self) }
    }

    pub fn matrix(&self) -> Matrix {
        unsafe { get_camera_matrix_2d(*self) }
    }
}

impl Default for Camera2D {
    fn default() -> Self {
        Self {
            offset: Vector2::ZERO,
            target: Vector2::ZERO,
            rotation: 0.0,
            zoom: 1.0,
        }
    }
}

impl From<Camera2D> for Matrix {
    fn from(val: Camera2D) -> Self {
        unsafe { get_camera_matrix_2d(val) }
    }
}
