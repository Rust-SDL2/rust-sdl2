mod desired;
pub use self::desired::AudioSpecDesired;
mod wav;
pub use self::wav::AudioSpecWAV;

use super::AudioFormat;

#[allow(missing_copy_implementations)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct AudioSpec {
    pub freq: i32,
    pub format: AudioFormat,
    pub channels: u8,
    /// The silence value calculated by SDL2. Note that it's inconvenient to use if your channel
    /// type is not u8 and [incorrect in case of u16](https://bugzilla.libsdl.org/show_bug.cgi?id=4805).
    /// You're likely to find [the `AudioFormatNum.SILENCE` associated constant](
    /// trait.AudioFormatNum.html#associatedconstant.SILENCE) more useful.
    pub silence: u8,
    pub samples: u16,
    pub size: u32,
}

impl AudioSpec {
    pub(super) fn convert_from_ll(spec: sys::SDL_AudioSpec) -> AudioSpec {
        AudioSpec {
            freq: spec.freq,
            format: AudioFormat::from_ll(spec.format).unwrap(),
            channels: spec.channels,
            silence: spec.silence,
            samples: spec.samples,
            size: spec.size,
        }
    }
}
