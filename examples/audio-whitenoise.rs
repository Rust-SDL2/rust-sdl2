extern crate sdl2;
extern crate rand;

use sdl2::audio::{AudioCallback, AudioSpecDesired};
use std::time::Duration;

struct MyCallback {
    volume: f32
}
impl AudioCallback for MyCallback {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        use self::rand::{Rng, thread_rng};
        let mut rng = thread_rng();

        // Generate white noise
        for x in out.iter_mut() {
            *x = (rng.gen_range(0.0, 2.0) - 1.0) * self.volume;
        }
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let audio_subsystem = sdl_context.audio()?;

    let desired_spec = AudioSpecDesired {
        freq: Some(44_100),
        channels: Some(1),  // mono
        samples: None,      // default sample size
    };

    // None: use default device
    let mut device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        // Show obtained AudioSpec
        println!("{:?}", spec);

        MyCallback { volume: 0.5 }
    })?;

    // Start playback
    device.resume();

    // Play for 1 second
    std::thread::sleep(Duration::from_millis(1_000));

    {
        // Acquire a lock. This lets us read and modify callback data.
        let mut lock = device.lock();
        (*lock).volume = 0.25;
        // Lock guard is dropped here
    }

    // Play for another second
    std::thread::sleep(Duration::from_millis(1_000));

    // Device is automatically closed when dropped

    Ok(())
}
