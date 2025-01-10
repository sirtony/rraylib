use crate::sys::*;
use crate::{guarded, newtype};
use std::ffi::{c_uint, c_void, CString};
use std::path::Path;
use std::ptr::{addr_of, addr_of_mut};
use std::sync::MutexGuard;
use std::time::Duration;

guarded!(AudioDevice);
newtype!(Wave, unload_wave);
newtype!(Sound, unload_sound);
newtype!(Music, unload_music_stream);
newtype!(AudioStream, unload_audio_stream);

pub struct Processor<'a> {
    stream: Option<&'a AudioStream>,
    ptr: AudioCallback,
}

impl Drop for Processor<'_> {
    fn drop(&mut self) {
        if let Some(stream) = self.stream {
            unsafe { detach_audio_stream_processor(stream.as_raw(), self.ptr) };
        } else {
            unsafe { detach_audio_mixed_processor(self.ptr) };
        }
    }
}

pub trait AudioControls {
    fn play(&mut self);
    fn stop(&mut self);
    fn pause(&mut self);
    fn resume(&mut self);
    fn is_playing(&self) -> bool;
    fn set_volume(&mut self, volume: f32);
    fn set_pitch(&mut self, pitch: f32);
    fn set_pan(&mut self, pan: f32);
}

pub struct SoundAlias<'a> {
    alias: crate::sys::Sound,
    _lifetime: std::marker::PhantomData<&'a ()>,
}

impl<'a> Drop for SoundAlias<'a> {
    fn drop(&mut self) {
        let ptr = addr_of!(self.alias);
        unsafe { unload_sound_alias(ptr.read()) };
    }
}

impl<'a> AudioDevice<'a> {
    pub(crate) fn get(guard: MutexGuard<'a, ()>) -> Self {
        if !unsafe { is_audio_device_ready() } {
            unsafe { init_audio_device() };
        }

        Self::new(guard)
    }
    pub fn is_ready(&self) -> bool {
        unsafe { is_audio_device_ready() }
    }

    pub fn master_volume(&self) -> f32 {
        unsafe { get_master_volume() }
    }

    pub fn set_master_volume(&mut self, volume: f32) {
        unsafe { set_master_volume(volume) }
    }
}

impl<'a> Drop for AudioDevice<'a> {
    fn drop(&mut self) {
        if self.is_ready() {
            unsafe { close_audio_device() };
        }
    }
}

impl Wave {
    pub fn from_file(file_name: impl AsRef<Path>) -> crate::Result<Self> {
        let file_name = file_name.as_ref().as_os_str().as_encoded_bytes();
        let file_name = CString::new(file_name)?;
        let wav = unsafe { load_wave(file_name.as_ptr()) };
        let ptr = addr_of!(wav);
        if !unsafe { is_wave_valid(ptr.read()) } {
            return Err(crate::Error::UnableToLoad("wave"));
        }
        Ok(Self(wav))
    }

    pub fn crop(&mut self, first_frame: u32, final_frame: u32) {
        let ptr = addr_of_mut!(self.0);
        unsafe { wave_crop(ptr, first_frame as i32, final_frame as i32) }
    }

    pub fn format(&mut self, sample_rate: u32, sample_size: u32, channels: u32) {
        let ptr = addr_of_mut!(self.0);
        unsafe { wave_format(ptr, sample_rate as i32, sample_size as i32, channels as i32) }
    }

    pub fn samples(&self) -> Vec<f32> {
        let samples = unsafe { load_wave_samples(self.as_raw()) };
        let len = size_of_val(&samples) / size_of::<f32>();
        let mut vec = Vec::with_capacity(len);

        for i in 0..len {
            let sample = unsafe { samples.offset(i as isize) };
            if sample.is_null() {
                break;
            }

            vec.push(unsafe { *sample });
        }

        unsafe { unload_wave_samples(samples) };

        vec
    }

    pub fn save(&self, file_name: impl AsRef<Path>) -> crate::Result<()> {
        let file_name = file_name.as_ref().as_os_str().as_encoded_bytes();
        let file_name = CString::new(file_name)?;
        unsafe { export_wave(self.as_raw(), file_name.as_ptr()) };
        Ok(())
    }
}

impl Clone for Wave {
    fn clone(&self) -> Self {
        Self(unsafe { wave_copy(self.as_raw()) })
    }
}

impl Sound {
    pub fn from_file(file_name: impl AsRef<Path>) -> crate::Result<Self> {
        let file_name = file_name.as_ref().as_os_str().as_encoded_bytes();
        let file_name = CString::new(file_name)?;
        let sound = unsafe { load_sound(file_name.as_ptr()) };
        let ptr = addr_of!(sound);
        if !unsafe { is_sound_valid(ptr.read()) } {
            return Err(crate::Error::UnableToLoad("sound"));
        }
        Ok(Self(sound))
    }

    pub fn update(&mut self, samples: impl AsRef<[f32]> + ExactSizeIterator) {
        let ptr = samples.as_ref().as_ptr();
        unsafe { update_sound(self.as_raw(), ptr as *mut c_void, samples.len() as i32) }
    }

    pub fn alias(&self) -> SoundAlias {
        let alias = unsafe { load_sound_alias(self.as_raw()) };
        SoundAlias {
            alias,
            _lifetime: std::marker::PhantomData,
        }
    }
}

impl AudioControls for Sound {
    fn play(&mut self) {
        unsafe { play_sound(self.as_raw()) };
    }

    fn stop(&mut self) {
        unsafe { stop_sound(self.as_raw()) };
    }

    fn pause(&mut self) {
        unsafe { pause_sound(self.as_raw()) };
    }

    fn resume(&mut self) {
        unsafe { resume_sound(self.as_raw()) };
    }

    fn is_playing(&self) -> bool {
        unsafe { is_sound_playing(self.as_raw()) }
    }

    fn set_volume(&mut self, volume: f32) {
        unsafe { set_sound_volume(self.as_raw(), volume) };
    }

    fn set_pitch(&mut self, pitch: f32) {
        unsafe { set_sound_pitch(self.as_raw(), pitch) };
    }

    fn set_pan(&mut self, pan: f32) {
        unsafe { set_sound_pan(self.as_raw(), pan) };
    }
}

impl Music {
    pub fn from_file(file_name: impl AsRef<Path>) -> crate::Result<Self> {
        let file_name = file_name.as_ref().as_os_str().as_encoded_bytes();
        let file_name = CString::new(file_name)?;
        let music = unsafe { load_music_stream(file_name.as_ptr()) };
        let ptr = addr_of!(music);
        if !unsafe { is_music_valid(ptr.read()) } {
            return Err(crate::Error::UnableToLoad("music"));
        }
        Ok(Self(music))
    }

    pub fn seek(&mut self, position: Duration) {
        unsafe { seek_music_stream(self.as_raw(), position.as_secs_f32()) };
    }

    pub fn update(&mut self) {
        unsafe { update_music_stream(self.as_raw()) };
    }

    pub fn length(&self) -> Duration {
        let secs = unsafe { get_music_time_length(self.as_raw()) };
        Duration::from_secs_f32(secs)
    }

    pub fn playtime(&self) -> Duration {
        let secs = unsafe { get_music_time_played(self.as_raw()) };
        Duration::from_secs_f32(secs)
    }
}

impl AudioControls for Music {
    fn play(&mut self) {
        unsafe { play_music_stream(self.as_raw()) };
    }

    fn stop(&mut self) {
        unsafe { stop_music_stream(self.as_raw()) };
    }

    fn pause(&mut self) {
        unsafe { pause_music_stream(self.as_raw()) };
    }

    fn resume(&mut self) {
        unsafe { resume_music_stream(self.as_raw()) };
    }

    fn is_playing(&self) -> bool {
        unsafe { is_music_stream_playing(self.as_raw()) }
    }

    fn set_volume(&mut self, volume: f32) {
        unsafe { set_music_volume(self.as_raw(), volume) };
    }

    fn set_pitch(&mut self, pitch: f32) {
        unsafe { set_music_pitch(self.as_raw(), pitch) };
    }

    fn set_pan(&mut self, pan: f32) {
        unsafe { set_music_pan(self.as_raw(), pan) };
    }
}

impl AudioStream {
    pub fn new(sample_rate: u32, sample_size: u32, channels: u32) -> crate::Result<Self> {
        let stream = unsafe { load_audio_stream(sample_rate, sample_size, channels) };
        let ptr = addr_of!(stream);
        if !unsafe { is_audio_stream_valid(ptr.read()) } {
            return Err(crate::Error::UnableToLoad("audio stream"));
        }

        Ok(Self(stream))
    }

    pub fn update(&mut self, samples: impl AsRef<[f32]> + ExactSizeIterator) {
        let ptr = samples.as_ref().as_ptr();
        unsafe { update_audio_stream(self.as_raw(), ptr as *mut c_void, samples.len() as i32) }
    }

    pub fn is_processing_finished(&self) -> bool {
        unsafe { is_audio_stream_processed(self.as_raw()) }
    }

    pub fn set_data_request_callback<F>(&mut self, func: F)
    where
        F: FnMut(&mut [f32], u32),
    {
        let wrapper = Self::wrap_fn_ptr(func);
        unsafe { set_audio_stream_callback(self.as_raw(), wrapper) };
    }

    pub fn attach_stream_processor<F>(&mut self, processor: F) -> Processor
    where
        F: FnMut(&mut [f32], u32),
    {
        let wrapper = Self::wrap_fn_ptr(processor);
        unsafe { attach_audio_stream_processor(self.as_raw(), wrapper.clone()) };

        Processor {
            stream: Some(self),
            ptr: wrapper,
        }
    }

    pub fn attach_mixed_processor<F>(processor: F) -> Processor<'static>
    where
        F: FnMut(&mut [f32], u32),
    {
        let wrapper = Self::wrap_fn_ptr(processor);
        unsafe { attach_audio_mixed_processor(wrapper.clone()) };

        Processor {
            stream: None,
            ptr: wrapper,
        }
    }

    pub fn set_default_buffer_size(size: u32) {
        unsafe { set_audio_stream_buffer_size_default(size as i32) }
    }

    fn wrap_fn_ptr<F>(mut func: F) -> AudioCallback
    where
        F: FnMut(&mut [f32], u32),
    {
        let wrapper = move |samples: *mut c_void, frames: c_uint| {
            let slice =
                unsafe { std::slice::from_raw_parts_mut(samples as *mut f32, frames as usize) };
            func(slice, frames)
        };

        let wrapper = Box::leak(Box::new(wrapper));
        let wrapper: unsafe extern "C" fn(*mut c_void, c_uint) =
            unsafe { std::mem::transmute(wrapper) };
        Some(wrapper)
    }
}

impl AudioControls for AudioStream {
    fn play(&mut self) {
        unsafe { play_audio_stream(self.as_raw()) };
    }

    fn stop(&mut self) {
        unsafe { stop_audio_stream(self.as_raw()) };
    }

    fn pause(&mut self) {
        unsafe { pause_audio_stream(self.as_raw()) };
    }

    fn resume(&mut self) {
        unsafe { resume_audio_stream(self.as_raw()) };
    }

    fn is_playing(&self) -> bool {
        unsafe { is_audio_stream_playing(self.as_raw()) }
    }

    fn set_volume(&mut self, volume: f32) {
        unsafe { set_audio_stream_volume(self.as_raw(), volume) };
    }

    fn set_pitch(&mut self, pitch: f32) {
        unsafe { set_audio_stream_pitch(self.as_raw(), pitch) };
    }

    fn set_pan(&mut self, pan: f32) {
        unsafe { set_audio_stream_pan(self.as_raw(), pan) };
    }
}
