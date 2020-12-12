use std::mem;

use libc::c_int;

use crate::get_error;

use super::AudioFormat;

#[derive(Copy, Clone)]
pub struct AudioCVT {
    raw: sys::SDL_AudioCVT,
}

impl AudioCVT {
    #[doc(alias = "SDL_BuildAudioCVT")]
    pub fn new(
        src_format: AudioFormat,
        src_channels: u8,
        src_rate: i32,
        dst_format: AudioFormat,
        dst_channels: u8,
        dst_rate: i32,
    ) -> Result<AudioCVT, String> {
        use std::mem::MaybeUninit;

        let mut raw: MaybeUninit<sys::SDL_AudioCVT> = mem::MaybeUninit::uninit();

        unsafe {
            let ret = sys::SDL_BuildAudioCVT(
                raw.as_mut_ptr(),
                src_format.to_ll(),
                src_channels,
                src_rate as c_int,
                dst_format.to_ll(),
                dst_channels,
                dst_rate as c_int,
            );
            if ret == 1 || ret == 0 {
                let raw = raw.assume_init();
                Ok(AudioCVT { raw })
            } else {
                Err(get_error())
            }
        }
    }

    #[doc(alias = "SDL_ConvertAudio")]
    pub fn convert(&self, mut src: Vec<u8>) -> Vec<u8> {
        //! Convert audio data to a desired audio format.
        //!
        //! The `src` vector is adjusted to the capacity necessary to perform
        //! the conversion in place; then it is passed to the SDL library.
        //!
        //! Certain conversions may cause buffer overflows. See AngryLawyer/rust-sdl2 issue #270.
        unsafe {
            if self.raw.needed != 0 {
                let mut raw = self.raw;

                // calculate the size of the dst buffer
                use std::convert::TryInto;
                raw.len = src.len().try_into().expect("Buffer length overflow");
                let dst_size = self.capacity(src.len());
                let needed = dst_size - src.len();
                src.reserve_exact(needed);

                // perform the conversion in place
                raw.buf = src.as_mut_ptr();
                let ret = sys::SDL_ConvertAudio(&mut raw);
                // There's no reason for SDL_ConvertAudio to fail.
                // The only time it can fail is if buf is NULL, which it never is.
                if ret != 0 {
                    panic!(get_error())
                }

                // return original buffer back to caller
                debug_assert!(raw.len_cvt > 0);
                debug_assert!(raw.len_cvt as usize <= src.capacity());

                src.set_len(raw.len_cvt as usize);
                src
            } else {
                // The buffer remains unmodified
                src
            }
        }
    }

    /// Checks if any conversion is needed. i.e. if the buffer that goes
    /// into `convert()` is unchanged from the result.
    pub fn is_conversion_needed(&self) -> bool {
        self.raw.needed != 0
    }

    /// Gets the buffer capacity that can contain both the original and
    /// converted data.
    pub fn capacity(&self, src_len: usize) -> usize {
        src_len
            .checked_mul(self.raw.len_mult as usize)
            .expect("Integer overflow")
    }
}
