use std::ffi::CString;
use std::sync::Mutex;
use typed_builder::TypedBuilder;

pub mod display;
pub mod graphics;
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

    #[cfg_attr(debug_assertions, builder(default = TraceLogLevel::LOG_DEBUG))]
    #[cfg_attr(not(debug_assertions), builder(default = TraceLogLevel::LOG_WARNING))]
    pub log_level: TraceLogLevel,
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
        if !unsafe { IsWindowReady() } {
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
    if unsafe { IsWindowReady() } {
        return Err(Error::SubsystemAlreadyInitialized("window"));
    }

    let title = CString::new(options.title.as_bytes())?;
    let mut flags = 0;

    if options.msaa {
        flags |= ConfigFlags::FLAG_MSAA_4X_HINT as u32;
    }

    if options.vsync {
        flags |= ConfigFlags::FLAG_VSYNC_HINT as u32;
    }

    if options.borderless {
        flags |= ConfigFlags::FLAG_BORDERLESS_WINDOWED_MODE as u32;
    }

    if options.resizable {
        flags |= ConfigFlags::FLAG_WINDOW_RESIZABLE as u32;
    }

    if !options.decorated {
        flags |= ConfigFlags::FLAG_WINDOW_UNDECORATED as u32;
    }

    if options.always_on_top {
        flags |= ConfigFlags::FLAG_WINDOW_TOPMOST as u32;
    }

    if options.fullscreen {
        flags |= ConfigFlags::FLAG_FULLSCREEN_MODE as u32;
    }

    if options.minimized {
        flags |= ConfigFlags::FLAG_WINDOW_MINIMIZED as u32;
    }

    if options.maximized {
        flags |= ConfigFlags::FLAG_WINDOW_MAXIMIZED as u32;
    }

    if options.high_dpi {
        flags |= ConfigFlags::FLAG_WINDOW_HIGHDPI as u32;
    }

    unsafe {
        SetTraceLogLevel(options.log_level as i32);
        SetConfigFlags(flags);
        InitWindow(options.width as i32, options.height as i32, title.as_ptr());

        if let Some(fps) = options.target_fps {
            SetTargetFPS(fps as i32);
        }
    }

    Ok(Context {
        window: Mutex::new(()),
        drawing: Mutex::new(()),
    })
}
