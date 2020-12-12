mod num;
pub use self::num::AudioFormatNum;

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum AudioFormat {
    /// Unsigned 8-bit samples
    U8 = sys::AUDIO_U8 as i32,
    /// Signed 8-bit samples
    S8 = sys::AUDIO_S8 as i32,
    /// Unsigned 16-bit samples, little-endian
    U16LSB = sys::AUDIO_U16LSB as i32,
    /// Unsigned 16-bit samples, big-endian
    U16MSB = sys::AUDIO_U16MSB as i32,
    /// Signed 16-bit samples, little-endian
    S16LSB = sys::AUDIO_S16LSB as i32,
    /// Signed 16-bit samples, big-endian
    S16MSB = sys::AUDIO_S16MSB as i32,
    /// Signed 32-bit samples, little-endian
    S32LSB = sys::AUDIO_S32LSB as i32,
    /// Signed 32-bit samples, big-endian
    S32MSB = sys::AUDIO_S32MSB as i32,
    /// 32-bit floating point samples, little-endian
    F32LSB = sys::AUDIO_F32LSB as i32,
    /// 32-bit floating point samples, big-endian
    F32MSB = sys::AUDIO_F32MSB as i32,
}

impl AudioFormat {
    pub(super) fn from_ll(raw: sys::SDL_AudioFormat) -> Option<AudioFormat> {
        use self::AudioFormat::*;
        match raw as u32 {
            sys::AUDIO_U8 => Some(U8),
            sys::AUDIO_S8 => Some(S8),
            sys::AUDIO_U16LSB => Some(U16LSB),
            sys::AUDIO_U16MSB => Some(U16MSB),
            sys::AUDIO_S16LSB => Some(S16LSB),
            sys::AUDIO_S16MSB => Some(S16MSB),
            sys::AUDIO_S32LSB => Some(S32LSB),
            sys::AUDIO_S32MSB => Some(S32MSB),
            sys::AUDIO_F32LSB => Some(F32LSB),
            sys::AUDIO_F32MSB => Some(F32MSB),
            _ => None,
        }
    }

    #[doc(alias = "SDL_AudioFormat")]
    pub(super) fn to_ll(self) -> sys::SDL_AudioFormat {
        self as sys::SDL_AudioFormat
    }
}

#[cfg(target_endian = "little")]
impl AudioFormat {
    /// Unsigned 16-bit samples, native endian
    #[inline]
    pub const fn u16_sys() -> AudioFormat {
        AudioFormat::U16LSB
    }
    /// Signed 16-bit samples, native endian
    #[inline]
    pub const fn s16_sys() -> AudioFormat {
        AudioFormat::S16LSB
    }
    /// Signed 32-bit samples, native endian
    #[inline]
    pub const fn s32_sys() -> AudioFormat {
        AudioFormat::S32LSB
    }
    /// 32-bit floating point samples, native endian
    #[inline]
    pub const fn f32_sys() -> AudioFormat {
        AudioFormat::F32LSB
    }
}

#[cfg(target_endian = "big")]
impl AudioFormat {
    /// Unsigned 16-bit samples, native endian
    #[inline]
    pub const fn u16_sys() -> AudioFormat {
        AudioFormat::U16MSB
    }
    /// Signed 16-bit samples, native endian
    #[inline]
    pub const fn s16_sys() -> AudioFormat {
        AudioFormat::S16MSB
    }
    /// Signed 32-bit samples, native endian
    #[inline]
    pub const fn s32_sys() -> AudioFormat {
        AudioFormat::S32MSB
    }
    /// 32-bit floating point samples, native endian
    #[inline]
    pub const fn f32_sys() -> AudioFormat {
        AudioFormat::F32MSB
    }
}
