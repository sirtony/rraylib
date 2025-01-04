use std::ffi::CString;
use std::sync::Mutex;
use typed_builder::TypedBuilder;

/// Monitor and Window handling.
pub mod display;

/// Drawing and rendering.
pub mod graphics;

/// Math functions and structures.
pub mod math;

/// Unsafe bindings to raylib, raymath, rlgl, raygui (if enabled), and Physac (if enabled).
pub mod sys;

use crate::display::Window;
use crate::graphics::Drawing;
use crate::sys::*;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Nul(#[from] std::ffi::NulError),

    #[error("{0} subsystem has not yet been initialized")]
    SubsystemNotInitialized(&'static str),

    #[error("{0} subsystem has already been initialized")]
    SubsystemAlreadyInitialized(&'static str),

    #[error("Unable to load {0}")]
    UnableToLoad(&'static str),

    #[error(transparent)]
    Utf8(#[from] std::str::Utf8Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("Can't acquire {0} lock as another thread already holds it")]
    ThreadAlreadyLocked(&'static str),

    #[error("{0}")]
    Generic(String),
}

impl<'a> From<&'a str> for Error {
    fn from(s: &'a str) -> Self {
        Self::Generic(s.to_string())
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Self::Generic(s)
    }
}

#[derive(Debug, TypedBuilder)]
pub struct InitOptions {
    #[builder(setter( transform = | x: impl ToString | x.to_string() ), default = "rraylib".to_string())]
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

impl Default for InitOptions {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[macro_export]
macro_rules! lock {
    ( $lock: expr ) => {
        $lock
            .lock()
            .unwrap_or_else(::std::sync::TryLockError::PoisonError::into_inner)
    };
}

#[macro_export]
macro_rules! try_lock {
    ( $lock: expr ) => {
        match $lock.try_lock() {
            Ok(guard) => Some(guard),
            Err(::std::sync::TryLockError::Poisoned(guard)) => Some(guard.into_inner()),
            Err(_) => None,
        }
    };
}

pub struct Context {
    window: Mutex<()>,
    drawing: Mutex<()>,
}

impl Context {
    pub fn window(&self) -> Result<Window> {
        if !unsafe { is_window_ready() } {
            return Err(Error::SubsystemNotInitialized("window"));
        }

        let guard = try_lock!(self.window).ok_or(Error::ThreadAlreadyLocked("window"))?;
        Ok(Window::new(guard))
    }

    pub fn begin_drawing(&self) -> Result<Drawing> {
        let guard = try_lock!(self.drawing).ok_or(Error::ThreadAlreadyLocked("drawing"))?;
        Ok(Drawing::new(guard, Mutex::new(())))
    }
}

pub fn init(options: InitOptions) -> Result<Context> {
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
            options.max_width.unwrap_or(0) as i32,
            options.max_height.unwrap_or(0) as i32,
        )
    }

    Ok(Context {
        window: Mutex::new(()),
        drawing: Mutex::new(()),
    })
}
