/// Demonstrates the simultaneous mixing of music and sound effects.
extern crate sdl2;

use sdl2::mixer::{InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS};
use std::env;
use std::path::Path;

fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: ./demo music.[mp3|wav|ogg] [sound-effect.[mp3|wav|ogg]]")
    } else {
        let sound_file = args.get(2).map(|sound_file| Path::new(sound_file));
        demo(Path::new(&args[1]), sound_file)?;
    }

    Ok(())
}

fn demo(music_file: &Path, sound_file: Option<&Path>) -> Result<(), String> {
    println!("linked version: {}", sdl2::mixer::get_linked_version());

    let sdl = sdl2::init()?;
    let _audio = sdl.audio()?;
    let mut timer = sdl.timer()?;

    let frequency = 44_100;
    let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
    let channels = DEFAULT_CHANNELS; // Stereo
    let chunk_size = 1_024;
    sdl2::mixer::open_audio(frequency, format, channels, chunk_size)?;
    let _mixer_context =
        sdl2::mixer::init(InitFlag::MP3 | InitFlag::FLAC | InitFlag::MOD | InitFlag::OGG)?;

    // Number of mixing channels available for sound effect `Chunk`s to play
    // simultaneously.
    sdl2::mixer::allocate_channels(4);

    {
        let n = sdl2::mixer::get_chunk_decoders_number();
        println!("available chunk(sample) decoders: {}", n);
        for i in 0..n {
            println!("  decoder {} => {}", i, sdl2::mixer::get_chunk_decoder(i));
        }
    }

    {
        let n = sdl2::mixer::get_music_decoders_number();
        println!("available music decoders: {}", n);
        for i in 0..n {
            println!("  decoder {} => {}", i, sdl2::mixer::get_music_decoder(i));
        }
    }

    println!("query spec => {:?}", sdl2::mixer::query_spec());

    let music = sdl2::mixer::Music::from_file(music_file)?;

    fn hook_finished() {
        println!("play ends! from rust cb");
    }

    sdl2::mixer::Music::hook_finished(hook_finished);

    println!("music => {:?}", music);
    println!("music type => {:?}", music.get_type());
    println!("music volume => {:?}", sdl2::mixer::Music::get_volume());
    println!("play => {:?}", music.play(1));

    {
        let sound_chunk = match sound_file {
            Some(sound_file_path) => sdl2::mixer::Chunk::from_file(sound_file_path)
                .map_err(|e| format!("Cannot load sound file: {:?}", e))?,
            None => {
                // One second of 500Hz sine wave using equation A * sin(2 * PI * f * t)
                // (played at half the volume to save people's ears).
                let buffer = (0..frequency)
                    .map(|i| {
                        (0.1 * i16::max_value() as f32
                            * (2.0 * 3.14 * 500.0 * (i as f32 / frequency as f32)).sin())
                            as i16
                    })
                    .collect();
                sdl2::mixer::Chunk::from_raw_buffer(buffer)
                    .map_err(|e| format!("Cannot get chunk from buffer: {:?}", e))?
            }
        };

        println!("chunk volume => {:?}", sound_chunk.get_volume());
        println!("playing sound twice");
        sdl2::mixer::Channel::all().play(&sound_chunk, 1)?;

        // This delay is needed because when the `Chunk` goes out of scope,
        // the sound effect stops playing. Delay long enough to hear the
        // sound.
        timer.delay(5_000);
        println!("played sound");
    }

    timer.delay(10_000);

    println!("fading out ... {:?}", sdl2::mixer::Music::fade_out(4_000));

    timer.delay(5_000);

    println!(
        "fading in from pos ... {:?}",
        music.fade_in_from_pos(1, 10_000, 100.0)
    );

    timer.delay(5_000);
    sdl2::mixer::Music::halt();
    timer.delay(1_000);

    println!("quitting sdl");

    Ok(())
}
