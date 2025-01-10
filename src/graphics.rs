use crate::drawing2d::Texture;
pub use crate::drawing2d::*;
pub use crate::drawing3d::*;
pub use crate::shapes::*;
use crate::sys::*;
pub use crate::sys::{Camera2D, Camera3D, CameraMode, CameraProjection, Color};
use crate::{guarded, newtype, try_lock, Error, Result};
use std::ffi::{c_void, CString};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Read;
use std::path::Path;
use std::ptr::{addr_of, addr_of_mut};
use std::sync::MutexGuard;

#[derive(Debug, Clone, Copy)]
pub struct Kernel<const N: usize>([f32; N]);

impl<const N: usize> Kernel<N> {
    pub fn new() -> Self {
        Self([0.0; N])
    }

    pub fn get_at(&self, x: usize, y: usize) -> Option<f32> {
        let idx = x + y * N;
        self.get(idx)
    }

    pub fn get_at_mut(&mut self, x: usize, y: usize) -> Option<&mut f32> {
        let idx = x + y * N;
        self.get_mut(idx)
    }

    pub fn get(&self, idx: usize) -> Option<f32> {
        if idx >= N {
            None
        } else {
            Some(self.0[idx])
        }
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut f32> {
        if idx >= N {
            None
        } else {
            Some(&mut self.0[idx])
        }
    }

    pub fn set(&mut self, idx: usize, value: f32) {
        if let Some(v) = self.get_mut(idx) {
            *v = value;
        }
    }

    pub fn set_at(&mut self, x: usize, y: usize, value: f32) {
        let idx = x + y * N;
        self.set(idx, value);
    }

    pub fn len(&self) -> usize {
        N * N
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn iter(&self) -> std::slice::Iter<'_, f32> {
        self.0.iter()
    }

    pub fn into_iter(self) -> std::array::IntoIter<f32, N> {
        self.0.into_iter()
    }

    pub fn try_copy_from<I>(&mut self, iter: I) -> Result<()>
    where
        I: IntoIterator<Item = f32> + ExactSizeIterator,
    {
        // raylib's kernel convolution requires the kernel to be square.
        // the function does check this, but it returns early and prints a warning to stderr and
        // allows execution to continue as normal, so we check here and return an error if
        // the kernel is not square to avoid any surprises.
        let sqr_root = (iter.len() as f32).sqrt() as usize;
        if sqr_root != N {
            return Err(Error::KernelNotSquare);
        }

        if iter.len() != N * N {
            return Err(Error::InsufficientData(format!(
                "{0}x{0} kernel (expected {1} floats, but only got {2})",
                N,
                N * N,
                iter.len()
            )));
        }

        let mut iter = iter.into_iter();
        for i in 0..N {
            // calling unwrap won't trigger a panic because we guaranteed the length above
            self.set(i, iter.next().unwrap());
        }

        Ok(())
    }
}

impl<const N: usize> Default for Kernel<N> {
    fn default() -> Self {
        Self::new()
    }
}

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

newtype!(Font, unload_font);
newtype!(Shader, unload_shader);

impl Font {
    pub fn from_file(file_name: impl AsRef<Path>) -> Result<Self> {
        let file_name = file_name.as_ref().as_os_str().as_encoded_bytes();
        let file_name = CString::new(file_name)?;
        let font = unsafe { load_font(file_name.as_ptr()) };
        let ptr = addr_of!(font);
        if !unsafe { is_font_valid(ptr.read()) } {
            return Err(Error::UnableToLoad("font"));
        }
        Ok(Self(font))
    }

    pub fn from_image(img: &Image, key: impl Into<Color>, first_char: i32) -> Result<Self> {
        let key = key.into();
        let font = unsafe { load_font_from_image(img.as_raw(), key, first_char) };
        let ptr = addr_of!(font);
        if !unsafe { is_font_valid(ptr.read()) } {
            return Err(Error::UnableToLoad("font"));
        }
        Ok(Self(font))
    }

    pub fn glyph(&self, codepoint: i32) -> GlyphInfo {
        unsafe { get_glyph_info(self.as_raw(), codepoint) }
    }

    pub fn glyph_index(&self, codepoint: i32) -> i32 {
        unsafe { get_glyph_index(self.as_raw(), codepoint) }
    }

    pub fn atlas_rect(&self, codepoint: i32) -> Rectangle {
        unsafe { get_glyph_atlas_rec(self.as_raw(), codepoint) }
    }
}

impl Default for Font {
    fn default() -> Self {
        Self(unsafe { get_font_default() })
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

    pub fn kernel_convolution<const N: usize>(&mut self, kernel: &Kernel<N>) {
        let ptr = addr_of_mut!(self.img);
        unsafe { image_kernel_convolution(ptr, kernel.0.as_ptr(), kernel.len() as i32) }
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

    /**
        Returns the raw pointer to the underlying raylib type.
        # Safety
        This method is unsafe because it allows the caller to access the underlying pointer directly.

        The caller must not free the pointer manually, allow the wrapper type to be dropped (resulting in a dangling pointer),
        or use the pointer to perform interior mutability unless first ensuring that the pointer is not currently in use elsewhere.
    */
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
    fn draw_shape<'t>(
        &mut self,
        shape: impl Into<Shape2D<'t>>,
        color: impl Into<Color>,
    ) -> Result<()> {
        let color = color.into();
        let img_ptr = addr_of_mut!(self.img);

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
                    unsafe { image_draw_line_ex(img_ptr, start, end, thickness as i32, color) }
                } else {
                    unsafe { image_draw_line_v(img_ptr, start, end, color) }
                };
                Ok(())
            }
            Shape2D::LineStrip(points) => {
                // there is no line strip function for images, so implement it manually
                for i in 0..points.len() - 1 {
                    let start = points[i];
                    let end = points[i + 1];
                    unsafe { image_draw_line_v(img_ptr, start, end, color) }
                }

                Ok(())
            }
            Shape2D::Circle { center, radius } => {
                unsafe { image_draw_circle_v(img_ptr, center, radius as i32, color) };
                Ok(())
            }
            Shape2D::Rectangle { rect, rotation } => {
                if rotation.is_some() {
                    Err(Error::OperationNotSupported {
                        verb: "shape drawing",
                        noun: "rotated rectangles",
                    })
                } else {
                    unsafe { image_draw_rectangle_rec(img_ptr, rect, color) };
                    Ok(())
                }
            }
            Shape2D::Triangle { v1, v2, v3 } => {
                unsafe { image_draw_triangle(img_ptr, v1, v2, v3, color) };
                Ok(())
            }
            Shape2D::TriangleFan(points) => {
                let ptr = points.as_ptr();
                // casting away the const-ness of the pointer is safe because the pointer is only being read
                // and the `*mut Vector2` pointer in the function signature was generated in error
                // because the C API doesn't mark the pointer as const even though it's not written to
                unsafe {
                    image_draw_triangle_fan(img_ptr, ptr as *mut _, points.len() as i32, color)
                };
                Ok(())
            }
            Shape2D::TriangleStrip(points) => {
                let ptr = points.as_ptr();
                // ditto
                unsafe {
                    image_draw_triangle_strip(img_ptr, ptr as *mut _, points.len() as i32, color)
                };
                Ok(())
            }

            Shape2D::Bezier { .. } => Err(Error::OperationNotSupported {
                verb: "shape drawing",
                noun: "bezier curves",
            }),
            Shape2D::Pie { .. } => Err(Error::OperationNotSupported {
                verb: "shape drawing",
                noun: "pie slices",
            }),
            Shape2D::Ellipse { .. } => Err(Error::OperationNotSupported {
                verb: "shape drawing",
                noun: "ellipses",
            }),
            Shape2D::Ring { .. } => Err(Error::OperationNotSupported {
                verb: "shape drawing",
                noun: "rings",
            }),
            Shape2D::RoundedRectangle { .. } => Err(Error::OperationNotSupported {
                verb: "shape drawing",
                noun: "rounded rectangles",
            }),
            Shape2D::Polygon { .. } => Err(Error::OperationNotSupported {
                verb: "shape drawing",
                noun: "polygons",
            }),
            Shape2D::Spline { .. } => Err(Error::OperationNotSupported {
                verb: "shape drawing",
                noun: "splines",
            }),
        }
    }

    fn draw_lines<'t>(
        &mut self,
        shape: impl Into<Shape2D<'t>>,
        line_thickness: Option<f32>,
        color: impl Into<Color>,
    ) -> Result<()> {
        let line_thickness = line_thickness.unwrap_or(1.0);
        let color = color.into();
        let img_ptr = addr_of_mut!(self.img);
        match shape.into() {
            Shape2D::Pixel(p) => {
                unsafe { image_draw_pixel(img_ptr, p.x as i32, p.y as i32, color) };
                Ok(())
            }
            Shape2D::Line {
                start,
                end,
                thickness,
            } => {
                if let Some(thickness) = thickness {
                    unsafe { image_draw_line_ex(img_ptr, start, end, thickness as i32, color) }
                } else {
                    unsafe { image_draw_line_v(img_ptr, start, end, color) }
                };
                Ok(())
            }
            Shape2D::LineStrip(points) => {
                // there is no line strip function for images, so implement it manually
                for i in 0..points.len() - 1 {
                    let start = points[i];
                    let end = points[i + 1];
                    unsafe { image_draw_line_v(img_ptr, start, end, color) }
                }

                Ok(())
            }
            Shape2D::Circle { center, radius } => {
                unsafe { image_draw_circle_lines_v(img_ptr, center, radius as i32, color) };
                Ok(())
            }
            Shape2D::Rectangle { rect, rotation } => {
                if rotation.is_some() {
                    Err(Error::OperationNotSupported {
                        verb: "line drawing",
                        noun: "rotated rectangles",
                    })
                } else {
                    unsafe {
                        image_draw_rectangle_lines(img_ptr, rect, line_thickness as i32, color)
                    };
                    Ok(())
                }
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
            Shape2D::Bezier { .. } => Err(Error::OperationNotSupported {
                verb: "line drawing",
                noun: "bezier curves",
            }),
            Shape2D::Pie { .. } => Err(Error::OperationNotSupported {
                verb: "line drawing",
                noun: "pie slices",
            }),
            Shape2D::Ellipse { .. } => Err(Error::OperationNotSupported {
                verb: "line drawing",
                noun: "ellipses",
            }),
            Shape2D::Ring { .. } => Err(Error::OperationNotSupported {
                verb: "line drawing",
                noun: "rings",
            }),
            Shape2D::RoundedRectangle { .. } => Err(Error::OperationNotSupported {
                verb: "line drawing",
                noun: "rounded rectangles",
            }),
            Shape2D::Polygon { .. } => Err(Error::OperationNotSupported {
                verb: "line drawing",
                noun: "polygons",
            }),
            Shape2D::Spline { .. } => Err(Error::OperationNotSupported {
                verb: "line drawing",
                noun: "splines",
            }),
        }
    }

    fn draw_gradient_h<'t>(
        &mut self,
        _shape: impl Into<Shape2D<'t>>,
        _start_color: impl Into<Color>,
        _end_color: impl Into<Color>,
    ) -> Result<()> {
        Err(Error::OperationNotSupported {
            verb: "gradient drawing",
            noun: "horizontal gradients",
        })
    }

    fn draw_gradient_v<'t>(
        &mut self,
        _shape: impl Into<Shape2D<'t>>,
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
        _texture: &Texture,
        _position: impl Into<Vector2>,
        _tint: impl Into<Color>,
    ) -> Result<()> {
        Err(Error::OperationNotSupported {
            verb: "texture drawing",
            noun: "textures",
        })
    }

    fn draw_texture_ex(
        &mut self,
        _texture: &Texture,
        _position: impl Into<Vector2>,
        _rotation: f32,
        _scale: f32,
        _tint: impl Into<Color>,
    ) -> Result<()> {
        Err(Error::OperationNotSupported {
            verb: "texture drawing",
            noun: "images",
        })
    }

    fn draw_texture_clipped(
        &mut self,
        _texture: &Texture,
        _src: impl Into<Rectangle>,
        _pos: impl Into<Vector2>,
        _tint: impl Into<Color>,
    ) -> Result<()> {
        Err(Error::OperationNotSupported {
            verb: "texture drawing",
            noun: "images",
        })
    }

    fn draw_texture_rotated(
        &mut self,
        _texture: &Texture,
        _src: impl Into<Rectangle>,
        _dst: impl Into<Rectangle>,
        _origin: impl Into<Vector2>,
        _rotation: f32,
        _tint: impl Into<Color>,
    ) -> Result<()> {
        Err(Error::OperationNotSupported {
            verb: "texture drawing",
            noun: "images",
        })
    }

    fn draw_texture_npatched(
        &mut self,
        _texture: &Texture,
        _info: impl Into<NPatchInfo>,
        _dst: impl Into<Rectangle>,
        _origin: impl Into<Vector2>,
        _rotation: f32,
        _tint: impl Into<Color>,
    ) -> Result<()> {
        Err(Error::OperationNotSupported {
            verb: "texture drawing",
            noun: "images",
        })
    }

    fn draw_text(
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

        unsafe { image_draw_text(ptr, text.as_ptr(), x, y, font_size as i32, color.into()) };
        Ok(())
    }

    fn draw_text_ex(
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

        unsafe {
            image_draw_text_ex(
                ptr,
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
        _font: &Font,
        _text: impl AsRef<str>,
        _pos: impl Into<Vector2>,
        _origin: impl Into<Vector2>,
        _rotation: f32,
        _font_size: f32,
        _spacing: f32,
        _tint: impl Into<Color>,
    ) -> Result<()> {
        Err(Error::OperationNotSupported {
            verb: "rotated text drawing",
            noun: "images",
        })
    }

    fn clear_background(&mut self, color: impl Into<Color>) {
        let ptr = addr_of_mut!(self.img);
        unsafe { image_clear_background(ptr, color.into()) }
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

    pub fn set_texture(&mut self, loc: impl AsRef<str>, texture: &Texture) -> Result<()> {
        let loc = self.location(loc)?;
        unsafe { set_shader_value_texture(self.as_raw(), loc, texture.as_raw()) }
        Ok(())
    }
}

guarded!(Drawing, drawing, shaded, viewport, blended);

impl Drawing<'_> {
    pub fn begin_shader_mode(&mut self, shader: &Shader) -> Result<DrawingShaded> {
        let guard = try_lock!(self.shaded).ok_or(Error::ThreadAlreadyLocked("drawing"))?;
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
        let guard = try_lock!(self.blended).ok_or(Error::ThreadAlreadyLocked("drawing"))?;
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
        let guard = try_lock!(self.viewport).ok_or(Error::ThreadAlreadyLocked("drawing"))?;
        unsafe { begin_scissor_mode(x, y, width, height) }
        Ok(DrawingViewport(guard))
    }

    pub fn begin_viewport_mode_rect(
        &mut self,
        viewport: impl Into<Rectangle>,
    ) -> Result<DrawingViewport> {
        let viewport = viewport.into();
        self.begin_viewport_mode(
            viewport.x as i32,
            viewport.y as i32,
            viewport.width as i32,
            viewport.height as i32,
        )
    }

    pub fn begin_viewport_mode_vec4(
        &mut self,
        viewport: impl Into<Vector4>,
    ) -> Result<DrawingViewport> {
        let viewport = viewport.into();
        self.begin_viewport_mode(
            viewport.x as i32,
            viewport.y as i32,
            viewport.z as i32,
            viewport.w as i32,
        )
    }

    pub fn begin_viewport_mode_vec2(
        &mut self,
        location: impl Into<Vector2>,
        size: impl Into<Vector2>,
    ) -> Result<DrawingViewport> {
        let location = location.into();
        let size = size.into();
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

    pub fn draw_viewport_rect<F>(&mut self, viewport: impl Into<Rectangle>, f: F) -> Result<()>
    where
        F: FnOnce(&mut DrawingViewport),
    {
        let viewport = viewport.into();
        self.draw_viewport(
            viewport.x as i32,
            viewport.y as i32,
            viewport.width as i32,
            viewport.height as i32,
            f,
        )
    }

    pub fn draw_viewport_vec4<F>(&mut self, viewport: impl Into<Vector4>, f: F) -> Result<()>
    where
        F: FnOnce(&mut DrawingViewport),
    {
        let viewport = viewport.into();
        self.draw_viewport(
            viewport.x as i32,
            viewport.y as i32,
            viewport.z as i32,
            viewport.w as i32,
            f,
        )
    }

    pub fn draw_viewport_vec2<F>(
        &mut self,
        location: impl Into<Vector2>,
        size: impl Into<Vector2>,
        f: F,
    ) -> Result<()>
    where
        F: FnOnce(&mut DrawingViewport),
    {
        let location = location.into();
        let size = size.into();
        self.draw_viewport(
            location.x as i32,
            location.y as i32,
            size.x as i32,
            size.y as i32,
            f,
        )
    }

    pub fn line_spacing(&mut self, spacing: i32) {
        unsafe { set_text_line_spacing(spacing) }
    }

    pub fn measure_text(&mut self, text: impl AsRef<str>, font_size: u32) -> i32 {
        let text = text.as_ref().as_bytes();
        let text = CString::new(text).unwrap();
        unsafe { measure_text(text.as_ptr(), font_size as i32) }
    }

    pub fn measure_text_ex(
        &mut self,
        font: &Font,
        text: impl AsRef<str>,
        font_size: f32,
        spacing: f32,
    ) -> Vector2 {
        let text = text.as_ref().as_bytes();
        let text = CString::new(text).unwrap();
        unsafe { measure_text_ex(font.as_raw(), text.as_ptr(), font_size, spacing) }
    }
}

impl Drop for Drawing<'_> {
    fn drop(&mut self) {
        unsafe { end_drawing() }
    }
}

impl Drawables2D for Drawing<'_> {}

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

impl From<Color> for i32 {
    fn from(val: Color) -> Self {
        unsafe { color_to_int(val) }
    }
}

impl From<u32> for Color {
    fn from(color: u32) -> Self {
        unsafe { get_color(color) }
    }
}

impl From<Color> for u32 {
    fn from(val: Color) -> Self {
        unsafe { color_to_int(val) as u32 }
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        Self { r, g, b, a }
    }
}

impl From<Color> for (u8, u8, u8, u8) {
    fn from(val: Color) -> Self {
        (val.r, val.g, val.b, val.a)
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self { r, g, b, a: 255 }
    }
}

impl From<Color> for (u8, u8, u8) {
    fn from(val: Color) -> Self {
        (val.r, val.g, val.b)
    }
}

impl From<[u8; 3]> for Color {
    fn from([r, g, b]: [u8; 3]) -> Self {
        Self { r, g, b, a: 255 }
    }
}

impl From<Color> for [u8; 3] {
    fn from(val: Color) -> Self {
        [val.r, val.g, val.b]
    }
}

impl From<[u8; 4]> for Color {
    fn from([r, g, b, a]: [u8; 4]) -> Self {
        Self { r, g, b, a }
    }
}

impl From<Color> for [u8; 4] {
    fn from(val: Color) -> Self {
        [val.r, val.g, val.b, val.a]
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
