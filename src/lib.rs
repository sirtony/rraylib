extern crate alloc;

use crate::error::{Error, Result};
use std::ffi::{CStr, CString};
use typed_builder::TypedBuilder;

/// Monitor and Window handling.
pub mod display;

/// Drawing and rendering.
pub mod graphics;

/// Math functions and structures.
pub mod math;

/// Keyboard, mouse, gamepad, touch, and gestures input.
pub mod input;

/// Record and playback events.
pub mod automation;

/// Unsafe bindings to raylib, raymath, rlgl, raygui (if enabled), and Physac (if enabled).
pub mod sys;

pub(crate) mod utils;

pub mod error;
/// Provides a wrapper for raylib's logging functionality.
pub mod log;

/// Audio device and sound management.
pub mod audio;

/// 2D physics engine.
#[cfg(feature = "physac")]
pub mod physac;

use crate::audio::AudioDevice;
use crate::display::Window;
use crate::graphics::Drawing;
use crate::sys::*;

#[derive(Debug, Clone, TypedBuilder)]
pub struct Options {
    #[builder(
        setter( transform = | x: impl ToString | x.to_string() ),
        default = format!(
            "rraylib v{}.{}",
            crate::sys::RAYLIB_VERSION_MAJOR,
            crate::sys::RAYLIB_VERSION_MINOR
        )
        .to_string()
    )]
    pub title: String,
    #[builder(default = 800)]
    pub width: u32,
    #[builder(default = 600)]
    pub height: u32,
    #[builder(default = false)]
    pub msaa: bool,
    #[builder(default = true)]
    pub vsync: bool,
    #[builder(default = false)]
    pub borderless: bool,
    #[builder(default = false)]
    pub resizable: bool,
    #[builder(default = true)]
    pub decorated: bool,
    #[builder(default = false)]
    pub always_on_top: bool,
    #[builder(default = false)]
    pub fullscreen: bool,
    #[builder(default = false)]
    pub minimized: bool,
    #[builder(default = false)]
    pub maximized: bool,
    #[builder(default = false)]
    pub high_dpi: bool,
    #[builder(setter(strip_option), default = None)]
    pub target_fps: Option<u32>,
    #[cfg_attr(debug_assertions, builder(default = LogLevel::Debug))]
    #[cfg_attr(not(debug_assertions), builder(default = LogLevel::Warning))]
    pub log_level: LogLevel,
    #[builder(setter(strip_option), default = None)]
    pub max_width: Option<u32>,
    #[builder(setter(strip_option), default = None)]
    pub max_height: Option<u32>,
}

impl Default for Options {
    fn default() -> Self {
        Self::builder().build()
    }
}

guarded!(base Context, window, drawing, audio, physics);

impl Context {
    pub fn window(&self) -> Result<Window> {
        if !unsafe { is_window_ready() } {
            return Err(Error::SubsystemNotInitialized("window"));
        }

        let guard = try_lock!(self.window).ok_or(Error::ThreadAlreadyLocked("window"))?;
        Ok(Window::new(guard))
    }

    pub fn audio(&self) -> Result<AudioDevice<'_>> {
        let guard = try_lock!(self.audio).ok_or(Error::ThreadAlreadyLocked("audio"))?;
        Ok(AudioDevice::get(guard))
    }

    #[cfg(feature = "physac")]
    pub fn physics(&self) -> Result<physac::Physics<'_>> {
        let guard = try_lock!(self.physics).ok_or(Error::ThreadAlreadyLocked("physics"))?;
        Ok(physac::Physics::get(guard))
    }

    pub fn begin_drawing(&self) -> Result<Drawing> {
        let guard = try_lock!(self.drawing).ok_or(Error::ThreadAlreadyLocked("drawing"))?;
        Ok(Drawing::new(guard))
    }

    pub fn exit_key(&mut self, key: Key) {
        unsafe {
            set_exit_key(key as i32);
        }
    }

    /// Gets the time in milliseconds for the last frame drawn.
    pub fn delta_time() -> f32 {
        unsafe { get_frame_time() * 1000.0 }
    }

    pub fn elapsed_time() -> f64 {
        unsafe { get_time() }
    }

    pub fn fps() -> u32 {
        unsafe { get_fps() as u32 }
    }

    pub fn clipboard_text(&self) -> Result<String> {
        let text = unsafe { get_clipboard_text() };

        if text.is_null() {
            return Ok(String::new());
        }

        let text = unsafe { CStr::from_ptr(text) };
        Ok(text.to_string_lossy().into_owned())
    }

    pub fn set_clipboard_text(&self, text: impl AsRef<str>) -> Result<()> {
        let text = CString::new(text.as_ref())?;
        unsafe {
            set_clipboard_text(text.as_ptr());
        }
        Ok(())
    }
}

pub fn init(options: Options) -> Result<Context> {
    if unsafe { is_window_ready() } {
        return Err(Error::SubsystemAlreadyInitialized("window"));
    }

    let title = CString::new(options.title.as_bytes())?;
    let mut flags = Vec::new();

    if options.msaa {
        flags.push(WindowFlags::Msaa4X);
    }

    if options.vsync {
        flags.push(WindowFlags::Vsync);
    }

    if options.borderless {
        flags.push(WindowFlags::Borderless);
    }

    if options.resizable {
        flags.push(WindowFlags::Resizable);
    }

    if !options.decorated {
        flags.push(WindowFlags::Undecorated);
    }

    if options.always_on_top {
        flags.push(WindowFlags::AlwaysRun);
    }

    if options.fullscreen {
        flags.push(WindowFlags::Fullscreen);
    }

    if options.minimized {
        flags.push(WindowFlags::Minimized);
    }

    if options.maximized {
        flags.push(WindowFlags::Maximized);
    }

    if options.high_dpi {
        flags.push(WindowFlags::HighDPI);
    }

    let flags = flags
        .into_iter()
        .map(move |x| x as u32)
        .fold(0, |acc, x| acc | x);

    unsafe {
        set_trace_log_level(options.log_level as i32);
        set_config_flags(flags);
        init_window(options.width as i32, options.height as i32, title.as_ptr());

        if let Some(fps) = options.target_fps {
            set_target_fps(fps as i32);
        }

        set_window_max_size(
            options.max_width.unwrap_or(u32::MAX) as i32,
            options.max_height.unwrap_or(u32::MAX) as i32,
        )
    }

    Ok(Context::new())
}
