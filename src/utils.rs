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
        $crate::utils::newtype!($name as $name);
    };

    ( $name: ident as $alias: ident ) => {
        #[derive(Debug)]
        pub struct $alias {
            inner: $crate::sys::$name,
            owned: bool,
        }

        impl $alias {
            pub fn owned(inner: $crate::sys::$name) -> Self {
                Self { inner, owned: true }
            }

            pub fn unowned(inner: $crate::sys::$name) -> Self {
                Self {
                    inner,
                    owned: false,
                }
            }

            /**
                Returns a copy of the underlying raylib struct.
                # Safety
                This method is unsafe because it allows the caller to access the wrapped struct directly.

                The caller must not free the struct manually, allow the wrapper type to be dropped (resulting in a dangling pointer),
                or use the pointer to perform interior mutability unless first ensuring that the pointer is not currently in use elsewhere.
            */
            pub unsafe fn as_raw(&self) -> $crate::sys::$name {
                let ptr = ::std::ptr::addr_of!(self.inner);
                ::std::ptr::read(ptr)
            }

            /**
            Returns the raw pointer to the underlying raylib type wrapped by this wrapper type.
            # Safety
            This method is unsafe because it allows the caller to access the underlying pointer directly.

            The caller must not free the pointer manually, allow the wrapper type to be dropped (resulting in a dangling pointer),
            or use the pointer to perform interior mutability unless first ensuring that the pointer is not currently in use elsewhere.
            */
            pub unsafe fn as_ptr(&self) -> *const $crate::sys::$name {
                let ptr = ::std::ptr::addr_of!(self.inner);
                ptr as *const _
            }

            /**
            Returns the raw pointer to the underlying raylib type wrapped by this wrapper type.
            # Safety
            This method is unsafe because it allows the caller to access the underlying pointer directly.

            The caller must not free the pointer manually, allow the wrapper type to be dropped (resulting in a dangling pointer),
            or use the pointer to perform interior mutability unless first ensuring that the pointer is not currently in use elsewhere.
            */
            pub unsafe fn as_mut_ptr(&mut self) -> *mut $crate::sys::$name {
                let ptr = ::std::ptr::addr_of_mut!(self.inner);
                ptr as *mut _
            }
        }

        impl From<$crate::sys::$name> for $alias {
            fn from(inner: $crate::sys::$name) -> Self {
                Self { inner, owned: true }
            }
        }
    };

    ( $name: ident, $drop: ident ) => {
        $crate::utils::newtype!($name as $name, $drop);
    };

    ( $name: ident as $alias: ident, $drop: ident ) => {
        $crate::utils::newtype!($name as $alias);

        impl Drop for $alias {
            fn drop(&mut self) {
                if !self.owned {
                    return;
                }
                unsafe {
                    let obj = self.as_raw();
                    $crate::sys::$drop(obj);
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
            $( pub(crate) $name: ::std::sync::Mutex<()> ),*
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

        impl Default for $tn {
            fn default() -> Self {
                Self::new()
            }
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

#[allow(unused_imports)]
pub use guarded;
