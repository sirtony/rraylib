use crate::guarded;
use crate::sys::*;

guarded!(Ui);

impl Ui<'_> {
    pub fn lock(&mut self) {
        unsafe { gui_lock() }
    }

    pub fn unlock(&mut self) {
        unsafe { gui_unlock() }
    }

    pub fn is_locked(&self) -> bool {
        unsafe { gui_is_locked() }
    }
}

impl Drop for Ui<'_> {
    fn drop(&mut self) {
        unsafe { gui_disable() }
    }
}
