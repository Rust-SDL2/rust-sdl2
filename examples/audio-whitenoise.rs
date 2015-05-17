extern crate sdl2;
extern crate rand;

use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};

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
            *x = (rng.next_f32()*2.0 - 1.0) * self.volume;
        }
    }
}

fn main() {
    let _sdl_context = sdl2::init().audio().unwrap();

    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),  // mono
        samples: None,      // default sample size
    };

    // None: use default device
    let mut device = AudioDevice::open_playback(None, desired_spec, |spec| {
        // Show obtained AudioSpec
        println!("{:?}", spec);

        MyCallback { volume: 0.5 }
    }).unwrap();

    // Start playback
    device.resume();

    // Play for 1 second
    sdl2::timer::delay(1000);

    {
        // Acquire a lock. This lets us read and modify callback data.
        let mut lock = device.lock();
        (*lock).volume = 0.25;
        // Lock guard is dropped here
    }

    // Play for another second
    sdl2::timer::delay(1000);

    // Device is automatically closed when dropped
}
