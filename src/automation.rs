use crate::sys::*;
use crate::Result;
use std::ffi::CString;
use std::path::Path;

pub struct AutomationPlayback<'a> {
    automation: &'a mut Automation,
    current_frame: u32,
}

impl AutomationPlayback<'_> {
    pub fn update(&mut self) {
        if self.is_finished() {
            return;
        }

        unsafe {
            let evt = self
                .automation
                .inner
                .events
                .wrapping_add(self.current_frame as usize);
            play_automation_event(*evt);
        }

        self.current_frame += 1;
    }

    pub fn is_playing(&self) -> bool {
        !self.is_finished()
    }

    pub fn is_finished(&self) -> bool {
        self.current_frame >= self.automation.inner.count
    }

    pub fn reset(&mut self) {
        self.current_frame = 0;
    }
}

crate::utils::newtype!(
    AutomationEventList as Automation,
    unload_automation_event_list
);

impl Default for Automation {
    fn default() -> Self {
        Self::new()
    }
}

impl Automation {
    pub fn new() -> Self {
        let ptr = unsafe { load_automation_event_list(std::ptr::null()) };
        Self::owned(ptr)
    }

    pub fn from_file(file_name: impl AsRef<Path>) -> Result<Self> {
        let file_name = file_name.as_ref().as_os_str().as_encoded_bytes();
        let file_name = CString::new(file_name)?;
        let ptr = unsafe { load_automation_event_list(file_name.as_ptr()) };

        if ptr.events.is_null() {
            return Err(crate::Error::UnableToLoad("automation events"));
        }

        Ok(Self::owned(ptr))
    }

    pub fn save(&self, file_name: impl AsRef<Path>) -> Result<()> {
        let file_name = file_name.as_ref().as_os_str().as_encoded_bytes();
        let file_name = CString::new(file_name)?;

        unsafe {
            export_automation_event_list(self.as_raw(), file_name.as_ptr());
        }

        Ok(())
    }

    pub fn start_recording(&mut self) {
        unsafe {
            set_automation_event_list(self.as_mut_ptr());
            start_automation_event_recording();
        }
    }

    pub fn start_recording_at(&mut self, frame: u32) {
        unsafe {
            set_automation_event_base_frame(frame as i32);
        }

        self.start_recording();
    }

    pub fn stop_recording(&mut self) {
        unsafe {
            stop_automation_event_recording();
            set_automation_event_list(std::ptr::null_mut());
        }
    }

    pub fn play(&mut self) -> AutomationPlayback {
        AutomationPlayback {
            automation: self,
            current_frame: 0,
        }
    }
}
