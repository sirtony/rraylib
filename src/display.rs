use crate::sys::*;
use crate::{guarded, Result};
use paste::paste;
use std::ffi::{CStr, CString};
use std::path::Path;

macro_rules! window_flag {
    ( $name:ident => $value:expr ) => {
        paste! {
            pub fn [<has_ $name _flag>](&self) -> bool {
                self.has_flag(crate::sys::WindowFlags::$value)
            }

            pub fn [<set_ $name _flag>](&mut self, value: bool) {
                if value {
                    self.set_flag(crate::sys::WindowFlags::$value);
                } else {
                    self.clear_flag(crate::sys::WindowFlags::$value);
                }
            }

            pub fn [<clear_ $name _flag>](&mut self) {
                self.clear_flag(crate::sys::WindowFlags::$value);
            }
        }
    };
}

guarded!(Window);

impl<'a> Window<'a> {
    pub fn close_requested(&self) -> bool {
        unsafe { window_should_close() }
    }

    pub fn close(&self) -> crate::Result<()> {
        unsafe {
            if is_window_ready() {
                close_window();
            }
        }

        Ok(())
    }

    pub fn is_ready(&self) -> bool {
        unsafe { is_window_ready() }
    }

    pub fn is_fullscreen(&self) -> bool {
        unsafe { is_window_fullscreen() }
    }

    pub fn is_hidden(&self) -> bool {
        unsafe { is_window_hidden() }
    }

    pub fn is_minimized(&self) -> bool {
        unsafe { is_window_minimized() }
    }

    pub fn is_maximized(&self) -> bool {
        unsafe { is_window_maximized() }
    }

    pub fn is_focused(&self) -> bool {
        unsafe { is_window_focused() }
    }

    pub fn is_resized(&self) -> bool {
        unsafe { is_window_resized() }
    }

    pub fn has_flag(&self, flag: WindowFlags) -> bool {
        unsafe { is_window_state(flag as u32) }
    }

    pub fn set_flag(&mut self, flag: WindowFlags) {
        unsafe { set_window_state(flag as u32) }
    }

    pub fn clear_flag(&mut self, flag: WindowFlags) {
        unsafe { clear_window_state(flag as u32) }
    }

    window_flag!( vsync => Vsync );
    window_flag!( fullscreen => Fullscreen );
    window_flag!( resizable => Resizable );
    window_flag!( undecorated => Undecorated );
    window_flag!( hidden => Hidden );
    window_flag!( minimized => Minimized );
    window_flag!( maximized => Maximized );
    window_flag!( unfocused => Unfocused );
    window_flag!( always_on_top => AlwaysOnTop );
    window_flag!( always_run => AlwaysRun );
    window_flag!( transparent => Transparent );
    window_flag!( high_dpi => HighDPI );
    window_flag!( mouse_passthrough => MousePassthrough );
    window_flag!( borderless => Borderless );
    window_flag!( msaa_4x => Msaa4X );
    window_flag!( interlaced => Interlaced );

    pub fn toggle_fullscreen(&mut self) {
        unsafe { toggle_fullscreen() }
    }

    pub fn toggle_borderless(&mut self) {
        unsafe { toggle_borderless_windowed() }
    }

    pub fn minimize(&mut self) {
        unsafe { minimize_window() }
    }

    pub fn maximize(&mut self) {
        unsafe { maximize_window() }
    }

    pub fn restore(&mut self) {
        unsafe { restore_window() }
    }

    pub fn icon(&mut self, image: impl Into<Image>) {
        unsafe { set_window_icon(image.into()) }
    }

    pub fn icons(&mut self, images: impl AsRef<[Image]> + ExactSizeIterator) {
        unsafe { set_window_icons(images.as_ref().as_ptr() as *mut Image, images.len() as i32) }
    }

    pub fn title(&mut self, title: impl AsRef<str>) -> Result<()> {
        let title = std::ffi::CString::new(title.as_ref())?;
        unsafe { set_window_title(title.as_ptr()) }
        Ok(())
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        unsafe { set_window_position(x, y) }
    }

    pub fn width(&self) -> u32 {
        unsafe { get_screen_width() as u32 }
    }

    pub fn height(&self) -> u32 {
        unsafe { get_screen_height() as u32 }
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width(), self.height())
    }

    pub fn sizev(&self) -> Vector2 {
        self.size().into()
    }

    pub fn render_width(&self) -> u32 {
        unsafe { get_render_width() as u32 }
    }

    pub fn render_height(&self) -> u32 {
        unsafe { get_render_height() as u32 }
    }

    pub fn render_size(&self) -> (u32, u32) {
        (self.render_width(), self.render_height())
    }

    pub fn render_sizev(&self) -> Vector2 {
        self.render_size().into()
    }

    pub fn set_positionv(&mut self, pos: impl Into<Vector2>) {
        let pos = pos.into();
        unsafe { set_window_position(pos.x as i32, pos.y as i32) }
    }

    pub fn set_monitor(&mut self, monitor: &Monitor) {
        unsafe { set_window_monitor(monitor.0 as i32) }
    }

    pub fn monitor(&self) -> Monitor {
        Monitor::current()
    }

    pub fn set_min_size(&mut self, width: u32, height: u32) {
        unsafe { set_window_min_size(width as i32, height as i32) }
    }

    pub fn set_min_sizev(&mut self, size: impl Into<Vector2>) {
        let size = size.into();
        unsafe { set_window_min_size(size.x as i32, size.y as i32) }
    }

    pub fn set_min_width(&mut self, width: u32) {
        self.set_min_size(width, 0);
    }

    pub fn set_min_height(&mut self, height: u32) {
        self.set_min_size(0, height);
    }

    pub fn set_max_size(&mut self, width: u32, height: u32) {
        unsafe { set_window_max_size(width as i32, height as i32) }
    }

    pub fn set_max_sizev(&mut self, size: impl Into<Vector2>) {
        let size = size.into();
        unsafe { set_window_max_size(size.x as i32, size.y as i32) }
    }

    pub fn set_max_width(&mut self, width: u32) {
        self.set_max_size(width, 0);
    }

    pub fn set_max_height(&mut self, height: u32) {
        self.set_max_size(0, height);
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        unsafe { set_window_size(width as i32, height as i32) }
    }

    pub fn set_sizev(&mut self, size: impl Into<Vector2>) {
        let size = size.into();
        unsafe { set_window_size(size.x as i32, size.y as i32) }
    }

    pub fn opacity(&mut self, opacity: f32) {
        unsafe { set_window_opacity(opacity) }
    }

    pub fn focus(&mut self) {
        unsafe { set_window_focused() }
    }

    pub fn dpi(&self) -> Vector2 {
        unsafe { get_window_scale_dpi() }
    }

    pub fn position(&self) -> (i32, i32) {
        let vec = unsafe { get_window_position() };
        (vec.x as i32, vec.y as i32)
    }

    pub fn positionv(&self) -> Vector2 {
        unsafe { get_window_position() }
    }

    pub fn show_cursor(&mut self) {
        unsafe { show_cursor() }
    }

    pub fn hide_cursor(&mut self) {
        unsafe { hide_cursor() }
    }

    pub fn is_cursor_hidden(&self) -> bool {
        unsafe { is_cursor_hidden() }
    }

    pub fn unlock_cursor(&mut self) {
        unsafe { enable_cursor() }
    }

    pub fn lock_cursor(&mut self) {
        unsafe { disable_cursor() }
    }

    pub fn is_cursor_on_screen(&self) -> bool {
        unsafe { is_cursor_on_screen() }
    }

    pub fn screenshot(&self, filename: impl AsRef<Path>) -> Result<()> {
        let filename = filename.as_ref().as_os_str().as_encoded_bytes();
        let filename = CString::new(filename.as_ref())?;
        unsafe { take_screenshot(filename.as_ptr()) }
        Ok(())
    }
}

pub struct Monitor(pub u32);

impl Monitor {
    pub fn get_count() -> u32 {
        unsafe { get_monitor_count() as u32 }
    }

    pub fn all() -> Vec<Monitor> {
        (0..Monitor::get_count()).map(Monitor).collect()
    }

    pub fn current() -> Monitor {
        Monitor(unsafe { get_current_monitor() as u32 })
    }

    pub fn width(&self) -> u32 {
        unsafe { get_monitor_width(self.0 as i32) as u32 }
    }

    pub fn height(&self) -> u32 {
        unsafe { get_monitor_height(self.0 as i32) as u32 }
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width(), self.height())
    }

    pub fn render_width(&self) -> u32 {
        unsafe { get_render_width() as u32 }
    }

    pub fn render_height(&self) -> u32 {
        unsafe { get_render_height() as u32 }
    }

    pub fn render_size(&self) -> (u32, u32) {
        (self.render_width(), self.render_height())
    }

    pub fn physical_width(&self) -> u32 {
        unsafe { get_monitor_physical_width(self.0 as i32) as u32 }
    }

    pub fn physical_height(&self) -> u32 {
        unsafe { get_monitor_physical_height(self.0 as i32) as u32 }
    }

    pub fn physical_size(&self) -> (u32, u32) {
        (self.physical_width(), self.physical_height())
    }

    pub fn x(&self) -> i32 {
        self.position().0
    }

    pub fn y(&self) -> i32 {
        self.position().1
    }

    pub fn position(&self) -> (i32, i32) {
        let vec = unsafe { get_monitor_position(self.0 as i32) };
        (vec.x as i32, vec.y as i32)
    }

    pub fn refresh_rate(&self) -> u32 {
        unsafe { get_monitor_refresh_rate(self.0 as i32) as u32 }
    }

    pub fn name(&self) -> Result<String> {
        let ptr = unsafe { get_monitor_name(self.0 as i32) };
        let str = unsafe { CStr::from_ptr(ptr).to_str()? };

        Ok(str.into())
    }
}
