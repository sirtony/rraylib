use crate::sys::*;
use crate::{guarded, try_lock, Error, Result};
use std::ffi::{c_void, CString};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Read;
use std::path::Path;
use std::ptr::{addr_of, addr_of_mut};
use std::sync::MutexGuard;

pub use crate::sys::{Camera2D, Camera3D, CameraMode, CameraProjection, Color};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Scaling {
    Bicubic,
    NearestNeighbor,
}

impl Default for Scaling {
    fn default() -> Self {
        Self::Bicubic
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SplineType {
    Linear,
    Basis,
    CatmullRom,
    Quadratic,
    Cubic,
}

impl SplineType {
    fn draw(
        &self,
        points: impl Iterator<Item = Vector2> + ExactSizeIterator,
        thickness: f32,
        color: impl Into<Color>,
    ) {
        let color = color.into();
        let points: Vec<Vector2> = points.collect();
        let ptr = points.as_ptr();

        match self {
            SplineType::Linear => unsafe {
                draw_spline_linear(ptr, points.len() as i32, thickness, color)
            },
            SplineType::Basis => unsafe {
                draw_spline_basis(ptr, points.len() as i32, thickness, color)
            },
            SplineType::CatmullRom => unsafe {
                draw_spline_catmull_rom(ptr, points.len() as i32, thickness, color)
            },
            SplineType::Quadratic => unsafe {
                draw_spline_bezier_quadratic(ptr, points.len() as i32, thickness, color)
            },
            SplineType::Cubic => unsafe {
                draw_spline_bezier_cubic(ptr, points.len() as i32, thickness, color)
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum Shape {
    Pixel(Vector2),
    Line {
        start: Vector2,
        end: Vector2,
        thickness: Option<f32>,
    },
    LineStrip(Vec<Vector2>),
    Bezier {
        start: Vector2,
        end: Vector2,
        thickness: f32,
    },
    Circle {
        center: Vector2,
        radius: f32,
    },
    Pie {
        center: Vector2,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        segments: u32,
    },
    Ellipse {
        center: Vector2,
        radius: Vector2,
    },
    Ring {
        center: Vector2,
        inner_radius: f32,
        outer_radius: f32,
        angle: Vector2,
        segments: u32,
    },
    Rectangle {
        rect: Rectangle,
        rotation: Option<f32>,
    },
    RoundedRectangle {
        rect: Rectangle,
        roundness: f32,
        segments: i32,
    },
    Triangle {
        v1: Vector2,
        v2: Vector2,
        v3: Vector2,
    },
    TriangleFan(Vec<Vector2>),
    TriangleStrip(Vec<Vector2>),
    Polygon {
        center: Vector2,
        sides: i32,
        radius: f32,
        rotation: f32,
    },
    Spline {
        spline_type: SplineType,
        points: Vec<Vector2>,
        thickness: f32,
    },
}

impl Shape {
    pub fn collides_with(&self, other: impl Into<Shape>) -> bool {
        let other = other.into();
        let pair = (self, &other);

        match pair {
            (Shape::Rectangle { rect: r1, .. }, Shape::Rectangle { rect: r2, .. }) => unsafe {
                check_collision_recs(*r1, *r2)
            },
            (
                Shape::Circle {
                    center: c1,
                    radius: r1,
                },
                Shape::Circle {
                    center: c2,
                    radius: r2,
                },
            ) => unsafe { check_collision_circles(*c1, *r1, *c2, *r2) },
            (
                Shape::Rectangle { rect: r, .. },
                Shape::Circle {
                    center: c,
                    radius: r2,
                },
            ) => unsafe { check_collision_circle_rec(*c, *r2, *r) },
            (
                Shape::Circle {
                    center: c,
                    radius: r,
                },
                Shape::Rectangle { rect: rc, .. },
            ) => unsafe { check_collision_circle_rec(*c, *r, *rc) },

            (
                Shape::Circle {
                    center: c,
                    radius: r,
                },
                Shape::Line {
                    start: s, end: e, ..
                },
            ) => unsafe { check_collision_circle_line(*c, *r, *s, *e) },
            (
                Shape::Line {
                    start: s, end: e, ..
                },
                Shape::Circle {
                    center: c,
                    radius: r,
                },
            ) => unsafe { check_collision_circle_line(*c, *r, *s, *e) },

            (Shape::Pixel(p), Shape::Rectangle { rect: r, .. }) => unsafe {
                check_collision_point_rec(*p, *r)
            },
            (Shape::Rectangle { rect: r, .. }, Shape::Pixel(p)) => unsafe {
                check_collision_point_rec(*p, *r)
            },

            (
                Shape::Pixel(p),
                Shape::Circle {
                    center: c,
                    radius: r,
                },
            ) => unsafe { check_collision_point_circle(*p, *c, *r) },
            (
                Shape::Circle {
                    center: c,
                    radius: r,
                },
                Shape::Pixel(p),
            ) => unsafe { check_collision_point_circle(*p, *c, *r) },

            (Shape::Pixel(p), Shape::Triangle { v1, v2, v3 }) => unsafe {
                check_collision_point_triangle(*p, *v1, *v2, *v3)
            },
            (Shape::Triangle { v1, v2, v3 }, Shape::Pixel(p)) => unsafe {
                check_collision_point_triangle(*p, *v1, *v2, *v3)
            },

            (
                Shape::Pixel(p),
                Shape::Line {
                    start: s, end: e, ..
                },
            ) => unsafe { check_collision_point_line(*p, *s, *e, 1) },
            (
                Shape::Line {
                    start: s, end: e, ..
                },
                Shape::Pixel(p),
            ) => unsafe { check_collision_point_line(*p, *s, *e, 1) },

            (
                Shape::Pixel(p),
                Shape::Polygon {
                    center,
                    sides,
                    radius,
                    ..
                },
            )
            | (
                Shape::Polygon {
                    center,
                    sides,
                    radius,
                    ..
                },
                Shape::Pixel(p),
            ) => {
                let mut points = Vec::with_capacity(*sides as usize);
                for i in 0..*sides {
                    let radius = *radius as f64;
                    let x = center.x as f64;
                    let y = center.y as f64;
                    let angle = 2.0 * PI / *sides as f64 * i as f64;
                    let x = x + radius * angle.cos();
                    let y = y + radius * angle.sin();
                    points.push(Vector2::new(x as f32, y as f32));
                }

                unsafe { check_collision_point_poly(*p, points.as_ptr(), points.len() as i32) }
            }

            _ => false,
        }
    }

    pub fn collision_point(&self, other: impl Into<Shape>) -> Option<(i32, i32)> {
        let (x, y): (i32, i32) = self.collision_pointv(other)?.into();
        Some((x, y))
    }

    pub fn collision_pointv(&self, other: impl Into<Shape>) -> Option<Vector2> {
        let other = other.into();
        let pair = (self, &other);

        match pair {
            (
                Shape::Line {
                    start: s1, end: e1, ..
                },
                Shape::Line {
                    start: s2, end: e2, ..
                },
            ) => {
                let mut ptr = Vector2::ZERO;
                let has_collision =
                    unsafe { check_collision_lines(*s1, *e1, *s2, *e2, addr_of_mut!(ptr)) };

                if has_collision {
                    Some(ptr)
                } else {
                    None
                }
            }

            _ => None,
        }
    }

    pub fn spline_point(&self, t: f32) -> Option<Vector2> {
        let t = t.clamp(0.0, 1.0);
        match self {
            Shape::Spline {
                spline_type,
                points,
                ..
            } => match spline_type {
                SplineType::Linear => {
                    let p1 = points.get(0)?;
                    let p2 = points.get(1)?;
                    Some(unsafe { get_spline_point_linear(*p1, *p2, t) })
                }
                SplineType::Basis => {
                    let p1 = points.get(0)?;
                    let p2 = points.get(1)?;
                    let p3 = points.get(2)?;
                    let p4 = points.get(3)?;
                    Some(unsafe { get_spline_point_basis(*p1, *p2, *p3, *p4, t) })
                }
                SplineType::CatmullRom => {
                    let p1 = points.get(0)?;
                    let p2 = points.get(1)?;
                    let p3 = points.get(2)?;
                    let p4 = points.get(3)?;
                    Some(unsafe { get_spline_point_catmull_rom(*p1, *p2, *p3, *p4, t) })
                }
                SplineType::Quadratic => {
                    let p1 = points.get(0)?;
                    let p2 = points.get(1)?;
                    let p3 = points.get(2)?;
                    Some(unsafe { get_spline_point_bezier_quad(*p1, *p2, *p3, t) })
                }
                SplineType::Cubic => {
                    let p1 = points.get(0)?;
                    let p2 = points.get(1)?;
                    let p3 = points.get(2)?;
                    let p4 = points.get(3)?;
                    Some(unsafe { get_spline_point_bezier_cubic(*p1, *p2, *p3, *p4, t) })
                }
            },
            _ => None,
        }
    }
}

impl<T: Into<Rectangle>> From<T> for Shape {
    fn from(rect: T) -> Self {
        Self::Rectangle {
            rect: rect.into(),
            rotation: None,
        }
    }
}

crate::utils::newtype!(VrStereoConfig, unload_vr_stereo_config);
crate::utils::newtype!(Shader, unload_shader);
crate::utils::newtype!(Texture2D, unload_texture);
crate::utils::newtype!(RenderTexture2D, unload_render_texture);
crate::utils::newtype!(Font, unload_font);

impl From<Image> for Texture2D {
    fn from(img: Image) -> Self {
        Self(unsafe { load_texture_from_image(img.as_raw()) })
    }
}

pub struct Image {
    img: crate::sys::Image,
    frames: Option<u32>,
}

impl Image {
    pub fn from_file(file_name: impl AsRef<Path>) -> Result<Self> {
        let file_name = file_name.as_ref().as_os_str().as_encoded_bytes();
        let file_name = CString::new(file_name)?;
        let img = unsafe { load_image(file_name.as_ptr()) };
        let ptr = addr_of!(img);

        if !unsafe { is_image_valid(ptr.read()) } {
            return Err(Error::UnableToLoad("image"));
        }

        Ok(Self { img, frames: None })
    }

    pub fn from_animated_file(file_name: impl AsRef<Path>) -> Result<Self> {
        let file_name = file_name.as_ref().as_os_str().as_encoded_bytes();
        let file_name = CString::new(file_name)?;
        let mut frames = 0;
        let img = unsafe { load_image_anim(file_name.as_ptr(), addr_of_mut!(frames)) };
        let ptr = addr_of!(img);

        if !unsafe { is_image_valid(ptr.read()) } {
            return Err(Error::UnableToLoad("image"));
        }

        Ok(Self {
            img,
            frames: Some(frames as u32),
        })
    }

    pub fn from_color(width: u32, height: u32, color: impl Into<Color>) -> Self {
        let color = color.into();
        let img = unsafe { gen_image_color(width as i32, height as i32, color) };
        Self { img, frames: None }
    }

    pub fn from_linear_gradient(
        width: u32,
        height: u32,
        direction: i32,
        start_color: impl Into<Color>,
        end_color: impl Into<Color>,
    ) -> Self {
        let start_color = start_color.into();
        let end_color = end_color.into();
        let img = unsafe {
            gen_image_gradient_linear(
                width as i32,
                height as i32,
                direction,
                start_color,
                end_color,
            )
        };
        Self { img, frames: None }
    }

    pub fn from_radial_gradient(
        width: u32,
        height: u32,
        density: f32,
        inner_color: impl Into<Color>,
        outer_color: impl Into<Color>,
    ) -> Self {
        let inner_color = inner_color.into();
        let outer_color = outer_color.into();
        let img = unsafe {
            gen_image_gradient_radial(
                width as i32,
                height as i32,
                density,
                inner_color,
                outer_color,
            )
        };
        Self { img, frames: None }
    }

    pub fn from_square_gradient(
        width: u32,
        height: u32,
        density: f32,
        inner_color: impl Into<Color>,
        outer_color: impl Into<Color>,
    ) -> Self {
        let inner_color = inner_color.into();
        let outer_color = outer_color.into();
        let img = unsafe {
            gen_image_gradient_square(
                width as i32,
                height as i32,
                density,
                inner_color,
                outer_color,
            )
        };
        Self { img, frames: None }
    }

    pub fn from_checker_pattern(
        width: u32,
        height: u32,
        checks_x: u32,
        checks_y: u32,
        col1: impl Into<Color>,
        col2: impl Into<Color>,
    ) -> Self {
        let col1 = col1.into();
        let col2 = col2.into();
        let img = unsafe {
            gen_image_checked(
                width as i32,
                height as i32,
                checks_x as i32,
                checks_y as i32,
                col1,
                col2,
            )
        };
        Self { img, frames: None }
    }

    pub fn from_white_noise(width: u32, height: u32, factor: f32) -> Self {
        let img = unsafe { gen_image_white_noise(width as i32, height as i32, factor) };
        Self { img, frames: None }
    }

    pub fn from_perlin_noise(
        width: u32,
        height: u32,
        offset_x: i32,
        offset_y: i32,
        scale: f32,
    ) -> Self {
        let img = unsafe {
            gen_image_perlin_noise(width as i32, height as i32, offset_x, offset_y, scale)
        };
        Self { img, frames: None }
    }

    pub fn from_cellular(width: u32, height: u32, tile_size: i32) -> Self {
        let img = unsafe { gen_image_cellular(width as i32, height as i32, tile_size) };
        Self { img, frames: None }
    }

    pub fn with_text(width: u32, height: u32, text: impl AsRef<str>) -> Result<Self> {
        let text = text.as_ref().as_bytes();
        let text = CString::new(text)?;
        let img = unsafe { gen_image_text(width as i32, height as i32, text.as_ptr()) };
        Ok(Self { img, frames: None })
    }

    pub fn from_text(
        text: impl AsRef<str>,
        font_size: u32,
        color: impl Into<Color>,
    ) -> Result<Self> {
        let text = text.as_ref().as_bytes();
        let text = CString::new(text)?;
        let color = color.into();
        let img = unsafe { image_text(text.as_ptr(), font_size as i32, color) };
        Ok(Self { img, frames: None })
    }

    pub fn from_text_ex(
        font: &Font,
        text: impl AsRef<str>,
        font_size: f32,
        spacing: f32,
        tint: impl Into<Color>,
    ) -> Result<Self> {
        let text = text.as_ref().as_bytes();
        let text = CString::new(text)?;
        let tint = tint.into();
        let img = unsafe { image_text_ex(font.as_raw(), text.as_ptr(), font_size, spacing, tint) };
        Ok(Self { img, frames: None })
    }

    pub fn from_screen() -> Result<Self> {
        let img = unsafe { load_image_from_screen() };
        let ptr = addr_of!(img);
        if !unsafe { is_image_valid(ptr.read()) } {
            Err(Error::UnableToLoad("image"))
        } else {
            Ok(Self { img, frames: None })
        }
    }

    pub fn is_valid(&self) -> bool {
        unsafe { is_image_valid(self.as_raw()) }
    }

    pub fn save(&self, file_name: impl AsRef<Path>) -> Result<()> {
        let file_name = file_name.as_ref().as_os_str().as_encoded_bytes();
        let file_name = CString::new(file_name)?;
        unsafe {
            export_image(self.as_raw(), file_name.as_ptr());
        }
        Ok(())
    }

    pub fn clone_from(&self, rc: impl Into<Rectangle>) -> Self {
        let rc = rc.into();
        let img = unsafe { image_from_image(self.as_raw(), rc) };
        Self {
            img,
            frames: self.frames,
        }
    }

    pub fn clone_channel(&self, channel: u32) -> Self {
        let img = unsafe { image_from_channel(self.as_raw(), channel as i32) };
        Self {
            img,
            frames: self.frames,
        }
    }

    pub fn change_format(&mut self, format: PixelFormat) {
        let ptr = addr_of_mut!(self.img);
        unsafe { image_format(ptr, format as i32) }
    }

    pub fn convert_to_power_of_two(&mut self, fill: impl Into<Color>) {
        let ptr = addr_of_mut!(self.img);
        unsafe { image_to_pot(ptr, fill.into()) }
    }

    pub fn crop(&mut self, rect: impl Into<Rectangle>) {
        let ptr = addr_of_mut!(self.img);
        unsafe { image_crop(ptr, rect.into()) }
    }

    pub fn alpha_crop(&mut self, threshold: f32) {
        let ptr = addr_of_mut!(self.img);
        unsafe { image_alpha_crop(ptr, threshold) }
    }

    pub fn alpha_clear(&mut self, color: impl Into<Color>, threshold: f32) {
        let ptr = addr_of_mut!(self.img);
        unsafe { image_alpha_clear(ptr, color.into(), threshold) }
    }

    pub fn alpha_mask(&mut self, alpha_mask: &Image) {
        let ptr = addr_of_mut!(self.img);
        unsafe { image_alpha_mask(ptr, alpha_mask.as_raw()) }
    }

    pub fn premultiply_alpha(&mut self) {
        let ptr = addr_of_mut!(self.img);
        unsafe { image_alpha_premultiply(ptr) }
    }

    pub fn gaussian_blur(&mut self, blur_size: i32) {
        let ptr = addr_of_mut!(self.img);
        unsafe { image_blur_gaussian(ptr, blur_size) }
    }

    pub fn kernel_convolution(&mut self, kernel: impl AsRef<[f32]> + ExactSizeIterator) {
        let len = kernel.len();
        let kernel = kernel.as_ref();
        let kernel = kernel.as_ptr();
        let ptr = addr_of_mut!(self.img);
        unsafe { image_kernel_convolution(ptr, kernel, len as i32) }
    }

    pub fn resize(&mut self, new_size: impl Into<Vector2>, algorithm: Scaling) {
        let ptr = addr_of_mut!(self.img);
        let new_size = new_size.into();
        let (w, h): (i32, i32) = new_size.into();

        match algorithm {
            Scaling::Bicubic => unsafe { image_resize(ptr, w, h) },
            Scaling::NearestNeighbor => unsafe { image_resize_nn(ptr, w, h) },
        }
    }

    pub fn resize_canvas(
        &mut self,
        new_size: impl Into<Vector2>,
        offset: impl Into<Vector2>,
        fill: impl Into<Color>,
    ) {
        let ptr = addr_of_mut!(self.img);
        let new_size = new_size.into();
        let (w, h): (i32, i32) = new_size.into();
        let offset = offset.into();
        let (x, y): (i32, i32) = offset.into();

        unsafe { image_resize_canvas(ptr, w, h, x, y, fill.into()) }
    }

    pub fn compute_mipmaps(&mut self) {
        let ptr = addr_of_mut!(self.img);
        unsafe { image_mipmaps(ptr) }
    }

    pub fn dither(&mut self, r: i32, g: i32, b: i32, a: i32) {
        let ptr = addr_of_mut!(self.img);
        unsafe { image_dither(ptr, r, g, b, a) }
    }

    pub fn flip_vertical(&mut self) {
        let ptr = addr_of_mut!(self.img);
        unsafe { image_flip_vertical(ptr) }
    }

    pub fn flip_horizontal(&mut self) {
        let ptr = addr_of_mut!(self.img);
        unsafe { image_flip_horizontal(ptr) }
    }

    pub fn rotate_clockwise(&mut self) {
        let ptr = addr_of_mut!(self.img);
        unsafe { image_rotate_cw(ptr) }
    }

    pub fn rotate_counter_clockwise(&mut self) {
        let ptr = addr_of_mut!(self.img);
        unsafe { image_rotate_ccw(ptr) }
    }

    pub fn tint(&mut self, color: impl Into<Color>) {
        let ptr = addr_of_mut!(self.img);
        unsafe { image_color_tint(ptr, color.into()) }
    }

    pub fn invert(&mut self) {
        let ptr = addr_of_mut!(self.img);
        unsafe { image_color_invert(ptr) }
    }

    pub fn grayscale(&mut self) {
        let ptr = addr_of_mut!(self.img);
        unsafe { image_color_grayscale(ptr) }
    }

    pub fn greyscale(&mut self) {
        self.grayscale();
    }

    pub fn contrast(&mut self, contrast: f32) {
        let ptr = addr_of_mut!(self.img);
        unsafe { image_color_contrast(ptr, contrast.clamp(-100.0, 100.0)) }
    }

    pub fn brightness(&mut self, brightness: i32) {
        let ptr = addr_of_mut!(self.img);
        unsafe { image_color_brightness(ptr, brightness.clamp(-255, 255)) }
    }

    pub fn replace_color(&mut self, color: impl Into<Color>, replace: impl Into<Color>) {
        let ptr = addr_of_mut!(self.img);
        unsafe { image_color_replace(ptr, color.into(), replace.into()) }
    }

    pub fn alpha_border(&self, threshold: f32) -> Rectangle {
        unsafe { get_image_alpha_border(self.as_raw(), threshold) }
    }

    pub fn colors(&self) -> Vec<Color> {
        let mut colors_raw = unsafe { load_image_colors(self.as_raw()) };
        let pixel_count = self.width() * self.height();
        let mut colors = Vec::with_capacity(pixel_count as usize);

        for _ in 0..pixel_count {
            colors.push(unsafe { colors_raw.read() });
            colors_raw = unsafe { colors_raw.add(1) };
        }

        unsafe { unload_image_colors(colors_raw) }

        colors
    }

    pub fn palette(&mut self, max_size: usize) -> Vec<Color> {
        let mut count: i32 = 0;
        let mut raw =
            unsafe { load_image_palette(self.as_raw(), max_size as i32, addr_of_mut!(count)) };
        let mut colors = Vec::with_capacity(count as usize);

        for _ in 0..count {
            colors.push(unsafe { raw.read() });
            raw = unsafe { raw.add(1) };
        }

        unsafe { unload_image_palette(raw) }

        colors
    }

    pub fn pixel(&self, loc: impl Into<Vector2>) -> Color {
        let loc = loc.into();
        let (x, y): (i32, i32) = loc.into();
        unsafe { get_image_color(self.as_raw(), x, y) }
    }

    pub fn width(&self) -> u32 {
        unsafe { self.as_raw().width as u32 }
    }

    pub fn height(&self) -> u32 {
        unsafe { self.as_raw().height as u32 }
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width(), self.height())
    }

    pub fn sizev(&self) -> Vector2 {
        self.size().into()
    }

    pub fn draw_image(
        &mut self,
        src: impl Into<Image>,
        src_rect: impl Into<Rectangle>,
        dest_rect: impl Into<Rectangle>,
        tint: impl Into<Color>,
    ) {
        let ptr = addr_of_mut!(self.img);
        unsafe {
            image_draw(
                ptr,
                src.into().as_raw(),
                src_rect.into(),
                dest_rect.into(),
                tint.into(),
            )
        }
    }

    pub fn draw_text(
        &mut self,
        text: impl AsRef<str>,
        pos: impl Into<Vector2>,
        font_size: u32,
        color: impl Into<Color>,
    ) -> Result<()> {
        let ptr = addr_of_mut!(self.img);
        let text = text.as_ref().as_bytes();
        let text = CString::new(text)?;
        let pos = pos.into();
        let (x, y): (i32, i32) = pos.into();

        Ok(unsafe { image_draw_text(ptr, text.as_ptr(), x, y, font_size as i32, color.into()) })
    }

    pub fn draw_text_ex(
        &mut self,
        font: &Font,
        text: impl AsRef<str>,
        pos: impl Into<Vector2>,
        font_size: f32,
        spacing: f32,
        tint: impl Into<Color>,
    ) -> Result<()> {
        let ptr = addr_of_mut!(self.img);
        let text = text.as_ref().as_bytes();
        let text = CString::new(text)?;

        Ok(unsafe {
            image_draw_text_ex(
                ptr,
                font.as_raw(),
                text.as_ptr(),
                pos.into(),
                font_size,
                spacing,
                tint.into(),
            )
        })
    }

    pub unsafe fn as_raw(&self) -> crate::sys::Image {
        let ptr = addr_of!(self.img);
        ptr.read()
    }
}

impl Clone for Image {
    fn clone(&self) -> Self {
        let img = unsafe { image_copy(self.as_raw()) };
        Self {
            img,
            frames: self.frames,
        }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { unload_image(self.as_raw()) }
    }
}

impl Drawables2D for Image {
    fn draw_shape(&mut self, shape: impl Into<Shape>, color: impl Into<Color>) -> Result<()> {
        let color = color.into();
        let img_ptr = addr_of_mut!(self.img);

        match shape.into() {
            Shape::Pixel(p) => Ok(unsafe { draw_pixel(p.x as i32, p.y as i32, color) }),
            Shape::Line {
                start,
                end,
                thickness,
            } => Ok(if let Some(thickness) = thickness {
                unsafe { image_draw_line_ex(img_ptr, start, end, thickness as i32, color) }
            } else {
                unsafe { image_draw_line_v(img_ptr, start, end, color) }
            }),
            Shape::LineStrip(points) => {
                // there is no line strip function for images, so implement it manually
                for i in 0..points.len() - 1 {
                    let start = points[i];
                    let end = points[i + 1];
                    unsafe { image_draw_line_v(img_ptr, start, end, color) }
                }

                Ok(())
            }
            Shape::Circle { center, radius } => {
                Ok(unsafe { image_draw_circle_v(img_ptr, center, radius as i32, color) })
            }
            Shape::Rectangle { rect, rotation } => {
                if let Some(_) = rotation {
                    Err(Error::OperationNotSupported {
                        verb: "shape drawing",
                        noun: "rotated rectangles",
                    })
                } else {
                    Ok(unsafe { image_draw_rectangle_rec(img_ptr, rect, color) })
                }
            }
            Shape::Triangle { v1, v2, v3 } => {
                Ok(unsafe { image_draw_triangle(img_ptr, v1, v2, v3, color) })
            }
            Shape::TriangleFan(mut points) => {
                let ptr = points.as_mut_ptr();
                Ok(unsafe { image_draw_triangle_fan(img_ptr, ptr, points.len() as i32, color) })
            }
            Shape::TriangleStrip(mut points) => {
                let ptr = points.as_mut_ptr();
                Ok(unsafe { image_draw_triangle_strip(img_ptr, ptr, points.len() as i32, color) })
            }

            Shape::Bezier { .. } => Err(Error::OperationNotSupported {
                verb: "shape drawing",
                noun: "bezier curves",
            }),
            Shape::Pie { .. } => Err(Error::OperationNotSupported {
                verb: "shape drawing",
                noun: "pie slices",
            }),
            Shape::Ellipse { .. } => Err(Error::OperationNotSupported {
                verb: "shape drawing",
                noun: "ellipses",
            }),
            Shape::Ring { .. } => Err(Error::OperationNotSupported {
                verb: "shape drawing",
                noun: "rings",
            }),
            Shape::RoundedRectangle { .. } => Err(Error::OperationNotSupported {
                verb: "shape drawing",
                noun: "rounded rectangles",
            }),
            Shape::Polygon { .. } => Err(Error::OperationNotSupported {
                verb: "shape drawing",
                noun: "polygons",
            }),
            Shape::Spline { .. } => Err(Error::OperationNotSupported {
                verb: "shape drawing",
                noun: "splines",
            }),
        }
    }

    fn draw_lines(
        &mut self,
        shape: impl Into<Shape>,
        line_thickness: Option<f32>,
        color: impl Into<Color>,
    ) -> Result<()> {
        let line_thickness = line_thickness.unwrap_or(1.0);
        let color = color.into();
        let img_ptr = addr_of_mut!(self.img);
        match shape.into() {
            Shape::Pixel(p) => {
                Ok(unsafe { image_draw_pixel(img_ptr, p.x as i32, p.y as i32, color) })
            }
            Shape::Line {
                start,
                end,
                thickness,
            } => Ok(if let Some(thickness) = thickness {
                unsafe { image_draw_line_ex(img_ptr, start, end, thickness as i32, color) }
            } else {
                unsafe { image_draw_line_v(img_ptr, start, end, color) }
            }),
            Shape::LineStrip(points) => {
                // there is no line strip function for images, so implement it manually
                for i in 0..points.len() - 1 {
                    let start = points[i];
                    let end = points[i + 1];
                    unsafe { image_draw_line_v(img_ptr, start, end, color) }
                }

                Ok(())
            }
            Shape::Circle { center, radius } => {
                Ok(unsafe { image_draw_circle_lines_v(img_ptr, center, radius as i32, color) })
            }
            Shape::Rectangle { rect, rotation } => {
                if let Some(_) = rotation {
                    Err(Error::OperationNotSupported {
                        verb: "line drawing",
                        noun: "rotated rectangles",
                    })
                } else {
                    Ok(unsafe {
                        image_draw_rectangle_lines(img_ptr, rect, line_thickness as i32, color)
                    })
                }
            }
            Shape::Triangle { v1, v2, v3 } => Ok(unsafe { draw_triangle_lines(v1, v2, v3, color) }),
            Shape::TriangleFan(_) => Err(Error::OperationNotSupported {
                verb: "line drawing",
                noun: "triangle fans",
            }),
            Shape::TriangleStrip(_) => Err(Error::OperationNotSupported {
                verb: "line drawing",
                noun: "triangle strips",
            }),
            Shape::Bezier { .. } => Err(Error::OperationNotSupported {
                verb: "line drawing",
                noun: "bezier curves",
            }),
            Shape::Pie { .. } => Err(Error::OperationNotSupported {
                verb: "line drawing",
                noun: "pie slices",
            }),
            Shape::Ellipse { .. } => Err(Error::OperationNotSupported {
                verb: "line drawing",
                noun: "ellipses",
            }),
            Shape::Ring { .. } => Err(Error::OperationNotSupported {
                verb: "line drawing",
                noun: "rings",
            }),
            Shape::RoundedRectangle { .. } => Err(Error::OperationNotSupported {
                verb: "line drawing",
                noun: "rounded rectangles",
            }),
            Shape::Polygon { .. } => Err(Error::OperationNotSupported {
                verb: "line drawing",
                noun: "polygons",
            }),
            Shape::Spline { .. } => Err(Error::OperationNotSupported {
                verb: "line drawing",
                noun: "splines",
            }),
        }
    }

    fn draw_gradient_h(
        &mut self,
        _shape: impl Into<Shape>,
        _start_color: impl Into<Color>,
        _end_color: impl Into<Color>,
    ) -> Result<()> {
        Err(Error::OperationNotSupported {
            verb: "gradient drawing",
            noun: "horizontal gradients",
        })
    }

    fn draw_gradient_v(
        &mut self,
        _shape: impl Into<Shape>,
        _start_color: impl Into<Color>,
        _end_color: impl Into<Color>,
    ) -> Result<()> {
        Err(Error::OperationNotSupported {
            verb: "gradient drawing",
            noun: "vertical gradients",
        })
    }

    fn draw_texture(
        &mut self,
        _texture: &Texture2D,
        _position: impl Into<Vector2>,
        _tint: Option<Color>,
    ) -> Result<()> {
        Err(Error::OperationNotSupported {
            verb: "texture drawing",
            noun: "textures",
        })
    }
}

impl Shader {
    pub fn from_file(vs: impl AsRef<Path>, fs: impl AsRef<Path>) -> Result<Self> {
        let mut vsf = File::open(vs)?;
        let mut fsf = File::open(fs)?;

        let mut vs = String::new();
        let mut fs = String::new();

        vsf.read_to_string(&mut vs)?;
        fsf.read_to_string(&mut fs)?;

        let vs = CString::new(vs)?;
        let fs = CString::new(fs)?;

        let shader = unsafe { load_shader_from_memory(vs.as_ptr(), fs.as_ptr()) };
        let ptr = addr_of!(shader);

        if !unsafe { is_shader_valid(ptr.read()) } {
            Err(Error::UnableToLoad("shader"))
        } else {
            Ok(Self(shader))
        }
    }

    pub fn location(&self, uniform: impl AsRef<str>) -> Result<i32> {
        let uniform = CString::new(uniform.as_ref())?;
        let loc = unsafe { get_shader_location(self.as_raw(), uniform.as_ptr()) };
        if loc == -1 {
            Err(Error::UnableToLoad("shader location"))
        } else {
            Ok(loc)
        }
    }

    pub fn attr(&self, attr: impl AsRef<str>) -> Result<i32> {
        let attr = CString::new(attr.as_ref())?;
        let loc = unsafe { get_shader_location_attrib(self.as_raw(), attr.as_ptr()) };
        if loc == -1 {
            Err(Error::UnableToLoad("shader attribute"))
        } else {
            Ok(loc)
        }
    }

    pub fn set_value<T>(
        &mut self,
        loc: impl AsRef<str>,
        value: T,
        uniform_type: ShaderUniformDataType,
    ) -> Result<()> {
        let loc = self.location(loc)?;
        let ptr = addr_of!(value) as *const c_void;
        unsafe { set_shader_value(self.as_raw(), loc, ptr, uniform_type as i32) }
        Ok(())
    }

    pub fn set_values<T>(
        &mut self,
        loc: impl AsRef<str>,
        uniform_type: ShaderUniformDataType,
        values: impl AsRef<[T]>,
    ) -> Result<()> {
        let loc = self.location(loc)?;
        let vec = values.as_ref().iter().collect::<Vec<_>>();
        let ptr = vec.as_ptr() as *const c_void;
        unsafe {
            set_shader_value_v(
                self.as_raw(),
                loc,
                ptr,
                uniform_type as i32,
                vec.len() as i32,
            )
        }
        Ok(())
    }

    pub fn set_matrix(&mut self, loc: impl AsRef<str>, mat: impl Into<Matrix>) -> Result<()> {
        let loc = self.location(loc)?;
        let mat = mat.into();
        unsafe { set_shader_value_matrix(self.as_raw(), loc, mat) }
        Ok(())
    }

    pub fn set_texture(&mut self, loc: impl AsRef<str>, texture: &Texture2D) -> Result<()> {
        let loc = self.location(loc)?;
        unsafe { set_shader_value_texture(self.as_raw(), loc, texture.as_raw()) }
        Ok(())
    }
}

impl Camera3D {
    pub fn position(&self, pos: Vector3) -> Self {
        Self {
            position: pos,
            ..*self
        }
    }

    pub fn positionf(&self, x: f32, y: f32, z: f32) -> Self {
        self.position(Vector3::new(x, y, z))
    }

    pub fn target(&self, target: Vector3) -> Self {
        Self { target, ..*self }
    }

    pub fn targetf(&self, x: f32, y: f32, z: f32) -> Self {
        self.target(Vector3::new(x, y, z))
    }

    pub fn up(&self, up: Vector3) -> Self {
        Self { up, ..*self }
    }

    pub fn upf(&self, x: f32, y: f32, z: f32) -> Self {
        self.up(Vector3::new(x, y, z))
    }

    pub fn fovy(&self, fovy: f32) -> Self {
        Self { fovy, ..*self }
    }

    pub fn projection(&self, projection: CameraProjection) -> Self {
        Self {
            projection: projection as i32,
            ..*self
        }
    }

    pub fn perspective(position: Vector3, target: Vector3, up: Vector3, fovy: f32) -> Self {
        Self {
            position,
            target,
            up,
            fovy,
            projection: CameraProjection::Perspective as i32,
        }
    }

    pub fn orthographic(position: Vector3, target: Vector3, up: Vector3, fovy: f32) -> Self {
        Self {
            position,
            target,
            up,
            fovy,
            projection: CameraProjection::Orthographic as i32,
        }
    }

    pub fn update(&mut self, mode: CameraMode) {
        unsafe { update_camera(self, mode as i32) }
    }

    pub fn update_pro(&mut self, movement: Vector3, rotation: Vector3, zoom: f32) {
        unsafe { update_camera_pro(self, movement, rotation, zoom) }
    }
}

impl Default for Camera3D {
    fn default() -> Self {
        Self {
            position: Vector3::ZERO,
            target: Vector3::ZERO,
            up: Vector3::UP,
            fovy: 45.0,
            projection: CameraProjection::Perspective as i32,
        }
    }
}

impl Camera {
    pub fn screen_to_world(&self, position: Vector2) -> Ray {
        unsafe { get_screen_to_world_ray(position, *self) }
    }

    pub fn screen_to_world_viewport(&self, position: Vector2, width: u32, height: u32) -> Ray {
        unsafe { get_screen_to_world_ray_ex(position, *self, width as i32, height as i32) }
    }

    pub fn world_to_screen(&self, position: Vector3) -> Vector2 {
        unsafe { get_world_to_screen(position, *self) }
    }

    pub fn world_to_screen_viewport(&self, position: Vector3, width: u32, height: u32) -> Vector2 {
        unsafe { get_world_to_screen_ex(position, *self, width as i32, height as i32) }
    }

    pub fn matrix(&self) -> Matrix {
        unsafe { get_camera_matrix(*self) }
    }
}

impl Into<Matrix> for Camera {
    fn into(self) -> Matrix {
        unsafe { get_camera_matrix(self) }
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

impl Into<Matrix> for Camera2D {
    fn into(self) -> Matrix {
        unsafe { get_camera_matrix_2d(self) }
    }
}

pub trait Drawables2D {
    fn draw_shape(&mut self, shape: impl Into<Shape>, color: impl Into<Color>) -> Result<()> {
        let color = color.into();
        match shape.into() {
            Shape::Pixel(p) => Ok(unsafe { draw_pixel(p.x as i32, p.y as i32, color) }),
            Shape::Line {
                start,
                end,
                thickness,
            } => Ok(if let Some(thickness) = thickness {
                unsafe { draw_line_ex(start, end, thickness, color) }
            } else {
                unsafe { draw_line_v(start, end, color) }
            }),
            Shape::LineStrip(points) => {
                let ptr = points.as_ptr();
                Ok(unsafe { draw_line_strip(ptr, points.len() as i32, color) })
            }
            Shape::Bezier {
                start,
                end,
                thickness,
            } => Ok(unsafe { draw_line_bezier(start, end, thickness, color) }),
            Shape::Circle { center, radius } => Ok(unsafe { draw_circle_v(center, radius, color) }),
            Shape::Pie {
                center,
                radius,
                start_angle,
                end_angle,
                segments,
            } => unsafe {
                Ok(draw_circle_sector(
                    center,
                    radius,
                    start_angle,
                    end_angle,
                    segments as i32,
                    color,
                ))
            },
            Shape::Ellipse { center, radius } => Ok(unsafe {
                draw_ellipse(center.x as i32, center.y as i32, radius.x, radius.y, color)
            }),
            Shape::Ring {
                center,
                inner_radius,
                outer_radius,
                angle,
                segments,
            } => Ok(unsafe {
                draw_ring(
                    center,
                    inner_radius,
                    outer_radius,
                    angle.x,
                    angle.y,
                    segments as i32,
                    color,
                )
            }),
            Shape::Rectangle { rect, rotation } => Ok(if let Some(rotation) = rotation {
                unsafe { draw_rectangle_pro(rect, rect.centerv(), rotation, color) }
            } else {
                unsafe { draw_rectangle_rec(rect, color) }
            }),
            Shape::RoundedRectangle {
                rect,
                roundness,
                segments,
            } => Ok(unsafe { draw_rectangle_rounded(rect, roundness, segments, color) }),
            Shape::Triangle { v1, v2, v3 } => Ok(unsafe { draw_triangle(v1, v2, v3, color) }),
            Shape::TriangleFan(points) => {
                let ptr = points.as_ptr();
                Ok(unsafe { draw_triangle_fan(ptr, points.len() as i32, color) })
            }
            Shape::TriangleStrip(points) => {
                let ptr = points.as_ptr();
                Ok(unsafe { draw_triangle_strip(ptr, points.len() as i32, color) })
            }
            Shape::Polygon {
                center,
                sides,
                radius,
                rotation,
            } => Ok(unsafe { draw_poly(center, sides, radius, rotation, color) }),
            Shape::Spline {
                spline_type,
                points,
                thickness,
            } => Ok(spline_type.draw(points.iter().copied(), thickness, color)),
        }
    }

    fn draw_lines(
        &mut self,
        shape: impl Into<Shape>,
        line_thickness: Option<f32>,
        color: impl Into<Color>,
    ) -> Result<()> {
        let line_thickness = line_thickness.unwrap_or(1.0);
        let color = color.into();
        match shape.into() {
            Shape::Pixel(p) => Ok(unsafe { draw_pixel(p.x as i32, p.y as i32, color) }),
            Shape::Line {
                start,
                end,
                thickness,
            } => Ok(if let Some(thickness) = thickness {
                unsafe { draw_line_ex(start, end, thickness, color) }
            } else {
                unsafe { draw_line_v(start, end, color) }
            }),
            Shape::LineStrip(points) => {
                let ptr = points.as_ptr();
                Ok(unsafe { draw_line_strip(ptr, points.len() as i32, color) })
            }
            Shape::Bezier {
                start,
                end,
                thickness,
            } => Ok(unsafe { draw_line_bezier(start, end, thickness, color) }),
            Shape::Circle { center, radius } => {
                Ok(unsafe { draw_circle_lines_v(center, radius, color) })
            }
            Shape::Pie {
                center,
                radius,
                start_angle,
                end_angle,
                segments,
            } => Ok(unsafe {
                draw_circle_sector_lines(
                    center,
                    radius,
                    start_angle,
                    end_angle,
                    segments as i32,
                    color,
                )
            }),
            Shape::Ellipse { center, radius } => Ok(unsafe {
                draw_ellipse_lines(center.x as i32, center.y as i32, radius.x, radius.y, color)
            }),
            Shape::Ring {
                center,
                inner_radius,
                outer_radius,
                angle,
                segments,
            } => Ok(unsafe {
                draw_ring_lines(
                    center,
                    inner_radius,
                    outer_radius,
                    angle.x,
                    angle.y,
                    segments as i32,
                    color,
                )
            }),
            Shape::Rectangle { rect, rotation } => {
                if let Some(_) = rotation {
                    Err(Error::OperationNotSupported {
                        verb: "line drawing",
                        noun: "rotated rectangles",
                    })
                } else {
                    Ok(unsafe { draw_rectangle_lines_ex(rect, line_thickness, color) })
                }
            }
            Shape::RoundedRectangle {
                rect,
                roundness,
                segments,
            } => Ok(unsafe {
                draw_rectangle_rounded_lines_ex(rect, roundness, segments, line_thickness, color)
            }),
            Shape::Triangle { v1, v2, v3 } => Ok(unsafe { draw_triangle_lines(v1, v2, v3, color) }),
            Shape::TriangleFan(_) => Err(Error::OperationNotSupported {
                verb: "line drawing",
                noun: "triangle fans",
            }),
            Shape::TriangleStrip(_) => Err(Error::OperationNotSupported {
                verb: "line drawing",
                noun: "triangle strips",
            }),
            Shape::Polygon {
                center,
                sides,
                radius,
                rotation,
            } => Ok(unsafe {
                draw_poly_lines_ex(center, sides, radius, rotation, line_thickness, color)
            }),
            Shape::Spline {
                spline_type,
                points,
                thickness,
            } => Ok(spline_type.draw(points.iter().copied(), thickness, color)),
        }
    }

    fn draw_gradient_h(
        &mut self,
        shape: impl Into<Shape>,
        start_color: impl Into<Color>,
        end_color: impl Into<Color>,
    ) -> Result<()> {
        let start_color = start_color.into();
        let end_color = end_color.into();

        match shape.into() {
            Shape::Rectangle { rect, rotation } => {
                if let Some(_) = rotation {
                    Err(Error::OperationNotSupported {
                        verb: "gradient drawing",
                        noun: "rotated rectangles",
                    })
                } else {
                    Ok(unsafe {
                        draw_rectangle_gradient_h(
                            rect.x as i32,
                            rect.y as i32,
                            rect.width as i32,
                            rect.height as i32,
                            start_color,
                            end_color,
                        )
                    })
                }
            }
            Shape::Circle { center, radius } => Ok(unsafe {
                draw_circle_gradient(
                    center.x as i32,
                    center.y as i32,
                    radius,
                    start_color,
                    end_color,
                )
            }),
            _ => Err(Error::OperationNotSupported {
                verb: "gradient drawing",
                noun: "non-rectangle/non-circle shapes",
            }),
        }
    }

    fn draw_gradient_v(
        &mut self,
        shape: impl Into<Shape>,
        start_color: impl Into<Color>,
        end_color: impl Into<Color>,
    ) -> Result<()> {
        let start_color = start_color.into();
        let end_color = end_color.into();

        match shape.into() {
            Shape::Rectangle { rect, rotation } => {
                if let Some(_) = rotation {
                    Err(Error::OperationNotSupported {
                        verb: "gradient drawing",
                        noun: "rotated rectangles",
                    })
                } else {
                    Ok(unsafe {
                        draw_rectangle_gradient_v(
                            rect.x as i32,
                            rect.y as i32,
                            rect.width as i32,
                            rect.height as i32,
                            start_color,
                            end_color,
                        )
                    })
                }
            }
            Shape::Circle { center, radius } => Ok(unsafe {
                draw_circle_gradient(
                    center.x as i32,
                    center.y as i32,
                    radius,
                    start_color,
                    end_color,
                )
            }),
            _ => Err(Error::OperationNotSupported {
                verb: "gradient drawing",
                noun: "non-rectangle/non-circle shapes",
            }),
        }
    }

    fn draw_texture(
        &mut self,
        texture: &Texture2D,
        position: impl Into<Vector2>,
        tint: Option<Color>,
    ) -> Result<()> {
        let position = position.into();
        let tint = tint.unwrap_or(Color::WHITE);
        Ok(unsafe { draw_texture_v(texture.as_raw(), position, tint) })
    }
}

guarded!(Drawing, drawing);

impl<'a> Drawing<'a> {
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
        unsafe { begin_texture_mode(texture.as_raw()) }
        Ok(DrawingTexture(guard))
    }

    pub fn draw_to_texture<F>(&mut self, texture: &RenderTexture2D, f: F) -> Result<()>
    where
        F: FnOnce(&mut DrawingTexture),
    {
        let mut ctx = self.begin_texture_mode(texture)?;
        f(&mut ctx);
        Ok(())
    }

    pub fn begin_shader_mode(&mut self, shader: &Shader) -> Result<DrawingShaded> {
        let guard = try_lock!(self.drawing).ok_or(Error::ThreadAlreadyLocked("drawing"))?;
        unsafe { begin_shader_mode(shader.as_raw()) }
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
        unsafe { begin_vr_stereo_mode(config.as_raw()) }
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

impl<'a> Drawables2D for Drawing<'a> {}

#[allow(dead_code)]
pub struct Drawing2D<'a>(MutexGuard<'a, ()>);

impl Drop for Drawing2D<'_> {
    fn drop(&mut self) {
        unsafe { end_mode_2d() }
    }
}

impl<'a> Drawables2D for Drawing2D<'a> {}

#[allow(dead_code)]
pub struct Drawing3D<'a>(MutexGuard<'a, ()>);

impl<'a> Drawing3D<'a> {
    pub fn draw_grid(&mut self, slices: i32, spacing: f32) {
        unsafe { draw_grid(slices, spacing) }
    }

    pub fn draw_plane(&mut self, center: Vector3, size: Vector2, color: Color) {
        unsafe { draw_plane(center, size, color) }
    }

    pub fn draw_cube(
        &mut self,
        position: Vector3,
        width: f32,
        height: f32,
        length: f32,
        color: Color,
    ) {
        unsafe { draw_cube(position, width, height, length, color) }
    }

    pub fn draw_cubev(&mut self, position: Vector3, size: Vector3, color: Color) {
        unsafe { draw_cube_v(position, size, color) }
    }
}

impl Drop for Drawing3D<'_> {
    fn drop(&mut self) {
        unsafe { end_mode_3d() }
    }
}

#[allow(dead_code)]
pub struct DrawingTexture<'a>(MutexGuard<'a, ()>);

impl<'a> Drawables2D for DrawingTexture<'a> {}

impl Drop for DrawingTexture<'_> {
    fn drop(&mut self) {
        unsafe { end_texture_mode() }
    }
}

#[allow(dead_code)]
pub struct DrawingShaded<'a>(MutexGuard<'a, ()>);

impl Drop for DrawingShaded<'_> {
    fn drop(&mut self) {
        unsafe { end_shader_mode() }
    }
}

#[allow(dead_code)]
pub struct DrawingBlended<'a>(MutexGuard<'a, ()>);

impl Drop for DrawingBlended<'_> {
    fn drop(&mut self) {
        unsafe { end_blend_mode() }
    }
}

#[allow(dead_code)]
pub struct DrawingViewport<'a>(MutexGuard<'a, ()>);

impl Drop for DrawingViewport<'_> {
    fn drop(&mut self) {
        unsafe { end_scissor_mode() }
    }
}

#[allow(dead_code)]
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

impl From<[u8; 3]> for Color {
    fn from([r, g, b]: [u8; 3]) -> Self {
        Self { r, g, b, a: 255 }
    }
}

impl Into<[u8; 3]> for Color {
    fn into(self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

impl From<[u8; 4]> for Color {
    fn from([r, g, b, a]: [u8; 4]) -> Self {
        Self { r, g, b, a }
    }
}

impl Into<[u8; 4]> for Color {
    fn into(self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl Hash for Color {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.r.hash(state);
        self.g.hash(state);
        self.b.hash(state);
        self.a.hash(state);
    }
}

impl Eq for Color {}
impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
    }
}
