#[macro_export]
macro_rules! trace {
        ( $msg: expr $(, $args: expr)* ) => {
            {
                let msg = format!($msg $(, $args)*);
                let msg = std::ffi::CString::new(msg)?;
                unsafe {
                    crate::sys::trace_log(crate::sys::LogLevel::Trace, msg.as_ptr());
                }
            }
        };
    }

#[macro_export]
macro_rules! debug {
        ( $msg: expr $(, $args: expr)* ) => {
            {
                let msg = format!($msg $(, $args)*);
                let msg = std::ffi::CString::new(msg)?;
                unsafe {
                    crate::sys::trace_log(crate::sys::LogLevel::Debug, msg.as_ptr());
                }
            }
        };
    }

#[macro_export]
macro_rules! info {
        ( $msg: expr $(, $args: expr)* ) => {
            {
                let msg = format!($msg $(, $args)*);
                let msg = std::ffi::CString::new(msg)?;
                unsafe {
                    crate::sys::trace_log(crate::sys::LogLevel::Info, msg.as_ptr());
                }
            }
        };
    }

#[macro_export]
macro_rules! warning {
        ( $msg: expr $(, $args: expr)* ) => {
            {
                let msg = format!($msg $(, $args)*);
                let msg = std::ffi::CString::new(msg)?;
                unsafe {
                    crate::sys::trace_log(crate::sys::LogLevel::Warning, msg.as_ptr());
                }
            }
        };
    }

#[macro_export]
macro_rules! error {
        ( $msg: expr $(, $args: expr)* ) => {
            {
                let msg = format!($msg $(, $args)*);
                let msg = std::ffi::CString::new(msg)?;
                unsafe {
                    crate::sys::trace_log(crate::sys::LogLevel::Error, msg.as_ptr());
                }
            }
        };
    }

#[allow(unused_imports)]
pub use trace;

#[allow(unused_imports)]
pub use debug;

#[allow(unused_imports)]
pub use info;

#[allow(unused_imports)]
pub use warning;

#[allow(unused_imports)]
pub use error;
