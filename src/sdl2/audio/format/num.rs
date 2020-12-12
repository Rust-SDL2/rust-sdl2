use super::AudioFormat;

/// A phantom type for retrieving the `SDL_AudioFormat` of a given generic type.
/// All format types are returned as native-endian.
pub trait AudioFormatNum {
    fn audio_format() -> AudioFormat;

    /// The appropriately typed silence value for the audio format used.
    ///
    /// # Examples
    ///
    /// ```
    /// // The AudioFormatNum trait has to be imported for the Channel::SILENCE part to work.
    /// use sdl2::audio::{AudioCallback, AudioFormatNum};
    ///
    /// struct Silence;
    ///
    /// impl AudioCallback for Silence {
    ///     type Channel = u16;
    ///
    ///     fn callback(&mut self, out: &mut [u16]) {
    ///         for dst in out.iter_mut() {
    ///             *dst = Self::Channel::SILENCE;
    ///         }
    ///     }
    /// }
    /// ```
    const SILENCE: Self;
}

/// `AUDIO_S8`
impl AudioFormatNum for i8 {
    fn audio_format() -> AudioFormat {
        AudioFormat::S8
    }
    const SILENCE: i8 = 0;
}
/// `AUDIO_U8`
impl AudioFormatNum for u8 {
    fn audio_format() -> AudioFormat {
        AudioFormat::U8
    }
    const SILENCE: u8 = 0x80;
}
/// `AUDIO_S16`
impl AudioFormatNum for i16 {
    fn audio_format() -> AudioFormat {
        AudioFormat::s16_sys()
    }
    const SILENCE: i16 = 0;
}
/// `AUDIO_U16`
impl AudioFormatNum for u16 {
    fn audio_format() -> AudioFormat {
        AudioFormat::u16_sys()
    }
    const SILENCE: u16 = 0x8000;
}
/// `AUDIO_S32`
impl AudioFormatNum for i32 {
    fn audio_format() -> AudioFormat {
        AudioFormat::s32_sys()
    }
    const SILENCE: i32 = 0;
}
/// `AUDIO_F32`
impl AudioFormatNum for f32 {
    fn audio_format() -> AudioFormat {
        AudioFormat::f32_sys()
    }
    const SILENCE: f32 = 0.0;
}
