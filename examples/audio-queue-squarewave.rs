extern crate sdl2;

use sdl2::audio::{AudioCallback, AudioSpecDesired};

use std::time::Duration;

fn gen_wave(bytes_to_write: i32) -> Vec<i16> {
    // Generate a square wave
    let tone_volume = 1000i16;
    let period = 48000 / 256;
    let sample_count = bytes_to_write;
    let mut result = Vec::new();
  
    for x in 0..sample_count {
        result.push(
                if (x / period) % 2 == 0 {
                tone_volume
                }
                else {
                -tone_volume
                }
        );
    }
    result
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();

    let desired_spec = AudioSpecDesired {
        freq: Some(48000),
        channels: Some(2),
        // mono  -
        samples: Some(4) 
        // default sample size 
        };

    let device = audio_subsystem.open_queue::<i16>(None, &desired_spec).unwrap();

    let target_bytes = 48000 * 4;
    let wave = gen_wave(target_bytes);
    device.queue(&wave);
    // Start playback 
    device.resume();

    // Play for 2 seconds 
    std::thread::sleep(Duration::from_millis(2000));

    // Device is automatically closed when dropped 
}
