extern crate sdl2;

use sdl2::audio::AudioSpecDesired;

use std::time::Duration;

fn gen_wave(bytes_to_write: i32) -> Vec<i16> {
    // Generate a square wave
    let tone_volume = 1_000i16;
    let period = 48_000 / 256;
    let sample_count = bytes_to_write;
    let mut result = Vec::new();

    for x in 0..sample_count {
        result.push(if (x / period) % 2 == 0 {
            tone_volume
        } else {
            -tone_volume
        });
    }
    result
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let audio_subsystem = sdl_context.audio()?;

    let desired_spec = AudioSpecDesired {
        freq: Some(48_000),
        channels: Some(2),
        // mono  -
        samples: None, // default sample size
    };

    let device = audio_subsystem.open_queue::<i16, _>(None, &desired_spec)?;

    let target_bytes = 48_000 * 4;
    let wave = gen_wave(target_bytes);
    device.queue_audio(&wave)?;
    // Start playback
    device.resume();

    // Play for 2 seconds
    std::thread::sleep(Duration::from_millis(2_000));

    // Device is automatically closed when dropped

    Ok(())
}
