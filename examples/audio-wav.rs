extern crate sdl2;

use sdl2::audio::{AudioCallback, AudioSpecDesired,AudioSpecWAV,AudioCVT};
use std::time::Duration;
use std::borrow::Cow;
use std::path::{PathBuf, Path};

// NOTE: You probably want to investigate the
// mixer feature for real use cases.

struct Sound {
    data: Vec<u8>,
    volume: f32,
    pos: usize,
}

impl AudioCallback for Sound {
    type Channel = u8;

    fn callback(&mut self, out: &mut [u8]) {
        for dst in out.iter_mut() {
            *dst = (*self.data.get(self.pos).unwrap_or(&0) as f32 * self.volume) as u8;
            self.pos += 1;
        }
    }
}

fn main() {
    let wav_file : Cow<'static, Path> = match std::env::args().nth(1) {
        None => Cow::from(Path::new("./assets/sine.wav")),
        Some(s) => Cow::from(PathBuf::from(s))
    };
    let sdl_context = sdl2::init().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();

    let desired_spec = AudioSpecDesired {
        freq: Some(44_100),
        channels: Some(1), // mono
        samples: None      // default
    };

    let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        let wav = AudioSpecWAV::load_wav(wav_file)
            .expect("Could not load test WAV file");

        let cvt = AudioCVT::new(
                wav.format, wav.channels, wav.freq,
                spec.format, spec.channels, spec.freq)
            .expect("Could not convert WAV file");

        let data = cvt.convert(wav.buffer().to_vec());

        // initialize the audio callback
        Sound {
            data: data,
            volume: 0.25,
            pos: 0,
        }
    }).unwrap();

    // Start playback
    device.resume();

    // Play for a second
    std::thread::sleep(Duration::from_millis(1_000));

    // Device is automatically closed when dropped
}
