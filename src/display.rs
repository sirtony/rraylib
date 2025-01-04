use crate::sys::*;
use crate::Result;
use std::ffi::CStr;
use std::sync::MutexGuard;

pub struct Window<'a>(MutexGuard<'a, ()>);

impl<'a> Window<'a> {
    pub(crate) fn new(guard: MutexGuard<'a, ()>) -> Self {
        Self(guard)
    }

    pub fn should_close(&self) -> bool {
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
