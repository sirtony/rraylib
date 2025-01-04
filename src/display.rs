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
        unsafe { WindowShouldClose() }
    }

    pub fn close(&self) -> crate::Result<()> {
        unsafe {
            if IsWindowReady() {
                CloseWindow();
            }
        }

        Ok(())
    }
}

pub struct Monitor(pub u32);

impl Monitor {
    pub fn get_count() -> u32 {
        unsafe { GetMonitorCount() as u32 }
    }

    pub fn all() -> Vec<Monitor> {
        (0..Monitor::get_count()).map(Monitor).collect()
    }

    pub fn current() -> Monitor {
        Monitor(unsafe { GetCurrentMonitor() as u32 })
    }

    pub fn width(&self) -> u32 {
        unsafe { GetMonitorWidth(self.0 as i32) as u32 }
    }

    pub fn height(&self) -> u32 {
        unsafe { GetMonitorHeight(self.0 as i32) as u32 }
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width(), self.height())
    }

    pub fn render_width(&self) -> u32 {
        unsafe { GetRenderWidth() as u32 }
    }

    pub fn render_height(&self) -> u32 {
        unsafe { GetRenderHeight() as u32 }
    }

    pub fn render_size(&self) -> (u32, u32) {
        (self.render_width(), self.render_height())
    }

    pub fn physical_width(&self) -> u32 {
        unsafe { GetMonitorPhysicalWidth(self.0 as i32) as u32 }
    }

    pub fn physical_height(&self) -> u32 {
        unsafe { GetMonitorPhysicalHeight(self.0 as i32) as u32 }
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
        let vec = unsafe { GetMonitorPosition(self.0 as i32) };
        (vec.x as i32, vec.y as i32)
    }

    pub fn refresh_rate(&self) -> u32 {
        unsafe { GetMonitorRefreshRate(self.0 as i32) as u32 }
    }

    pub fn name(&self) -> Result<String> {
        let ptr = unsafe { GetMonitorName(self.0 as i32) };
        let str = unsafe { CStr::from_ptr(ptr).to_str()? };

        Ok(str.into())
    }
}
