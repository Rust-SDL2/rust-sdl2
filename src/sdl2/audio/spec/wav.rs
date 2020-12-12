use std::path::Path;

use super::super::AudioFormat;
use crate::get_error;
use crate::rwops::RWops;

pub struct AudioSpecWAV {
    pub freq: i32,
    pub format: AudioFormat,
    pub channels: u8,
    audio_buf: *mut u8,
    audio_len: u32,
}

impl AudioSpecWAV {
    /// Loads a WAVE from the file path.
    pub fn load_wav<P: AsRef<Path>>(path: P) -> Result<AudioSpecWAV, String> {
        let mut file = RWops::from_file(path, "rb")?;
        AudioSpecWAV::load_wav_rw(&mut file)
    }

    /// Loads a WAVE from the data source.
    #[doc(alias = "SDL_LoadWAV_RW")]
    pub fn load_wav_rw(src: &mut RWops) -> Result<AudioSpecWAV, String> {
        use std::mem::MaybeUninit;
        use std::ptr::null_mut;

        let mut desired = MaybeUninit::uninit();
        let mut audio_buf: *mut u8 = null_mut();
        let mut audio_len: u32 = 0;
        unsafe {
            let ret = sys::SDL_LoadWAV_RW(
                src.raw(),
                0,
                desired.as_mut_ptr(),
                &mut audio_buf,
                &mut audio_len,
            );
            if ret.is_null() {
                Err(get_error())
            } else {
                let desired = desired.assume_init();
                Ok(AudioSpecWAV {
                    freq: desired.freq,
                    format: AudioFormat::from_ll(desired.format).unwrap(),
                    channels: desired.channels,
                    audio_buf,
                    audio_len,
                })
            }
        }
    }

    pub fn buffer(&self) -> &[u8] {
        use std::slice::from_raw_parts;
        unsafe {
            let ptr = self.audio_buf as *const u8;
            let len = self.audio_len as usize;
            from_raw_parts(ptr, len)
        }
    }
}

impl Drop for AudioSpecWAV {
    #[doc(alias = "SDL_FreeWAV")]
    fn drop(&mut self) {
        unsafe {
            sys::SDL_FreeWAV(self.audio_buf);
        }
    }
}
