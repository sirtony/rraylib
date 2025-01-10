#[macro_export]
macro_rules! lock {
    ( $lock: expr ) => {
        $lock
            .lock()
            .unwrap_or_else(::std::sync::PoisonError::into_inner)
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

#[allow(unused_imports)]
pub use lock;
#[allow(unused_imports)]
pub use try_lock;

#[macro_export]
macro_rules! newtype {
    ( $name: ident ) => {
        crate::utils::newtype!($name as $name);
    };

    ( $name: ident as $alias: ident ) => {
        #[derive(Debug)]
        pub struct $alias(crate::sys::$name);

        impl $alias {
            pub unsafe fn as_raw(&self) -> crate::sys::$name {
                let ptr = ::std::ptr::addr_of!(self.0);
                ::std::ptr::read(ptr)
            }
        }

        impl From<crate::sys::$name> for $alias {
            fn from(inner: crate::sys::$name) -> Self {
                Self(inner)
            }
        }
    };

    ( $name: ident, $drop: ident ) => {
        crate::utils::newtype!($name as $name, $drop);
    };

    ( $name: ident as $alias: ident, $drop: ident ) => {
        crate::utils::newtype!($name as $alias);

        impl Drop for $alias {
            fn drop(&mut self) {
                unsafe {
                    let ptr = ::std::ptr::addr_of_mut!(self.0);
                    let ptr = ptr.read();
                    crate::sys::$drop(ptr);
                }
            }
        }
    };
}

pub use newtype;

#[macro_export]
macro_rules! guarded {
    ( $tn: ident $( ,$name: ident )* ) => {
        pub struct $tn<'a> {
            _guard: ::std::sync::MutexGuard<'a, ()>,
            $( $name: ::std::sync::Mutex<()> ),*
        }

        impl<'a> $tn<'a> {
            pub fn new(guard: ::std::sync::MutexGuard<'a, ()>) -> Self {
                Self {
                    _guard: guard,
                    $( $name: ::std::sync::Mutex::new(()) ),*
                }
            }
        }
    };

    ( base $tn: ident $( ,$name: ident )* ) => {
        pub struct $tn {
            $( $name: ::std::sync::Mutex<()> ),*
        }

        impl $tn {
            pub fn new() -> Self {
                Self {
                    $( $name: ::std::sync::Mutex::new(()) ),*
                }
            }
        }
    };
}

pub use guarded;
