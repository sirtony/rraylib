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

    #[error("{verb} operation not supported by {noun}")]
    OperationNotSupported {
        verb: &'static str,
        noun: &'static str,
    },

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
