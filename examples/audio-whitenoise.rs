extern crate sdl2;
use sdl2::audio::{AudioCallback, AudioSpecDesired};
use std::rand::{Rng, StdRng};

struct MyCallback {
    /// Random number generator for white noise
    rng: StdRng,
    volume: f32
}
impl AudioCallback<f32> for MyCallback {
    fn callback(&mut self, out: &mut [f32]) {
        // Generate white noise
        for x in out.iter_mut() {
            *x = self.rng.next_f32() * self.volume;
        }
    }
}

fn main() {
    sdl2::init(sdl2::INIT_AUDIO);

    let desired_spec = AudioSpecDesired {
        freq: 44100,
        channels: 1,
        callback: box MyCallback { rng: StdRng::new().unwrap(), volume: 1.0 }
    };

    // None: use default device
    // false: Playback
    let mut device = match desired_spec.open_audio_device(None, false) {
        Ok(device) => device,
        Err(s) => panic!("{}", s)
    };

    // Show obtained AudioSpec
    println!("{}", device.get_spec());

    // Start playback
    device.resume();

    // Play for 1 second
    sdl2::timer::delay(1000);

    {
        // Acquire a lock. This lets us read and modify callback data.
        let mut lock = device.lock();
        (*lock).volume = 0.5;
        // Lock guard is dropped here
    }

    // Play for another second
    sdl2::timer::delay(1000);

    // Device is automatically closed when dropped
}
