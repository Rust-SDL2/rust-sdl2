extern crate sdl2;
extern crate sdl2_mixer;

use std::env;
use std::path::Path;
use sdl2_mixer::{INIT_MP3, INIT_FLAC, INIT_MOD, INIT_FLUIDSYNTH, INIT_MODPLUG, INIT_OGG,
                 AUDIO_S16LSB};

fn main() {

    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: ./demo audio.[mp3|wav|ogg]")
    } else {
        demo(Path::new(&args[1]));
    }
}

fn demo(filename: &Path) {

    println!("linked version: {}", sdl2_mixer::get_linked_version());

    let sdl = sdl2::init().unwrap();
    let _audio = sdl.audio().unwrap();
    let mut timer = sdl.timer().unwrap();
    let _mixer_context = sdl2_mixer::init(INIT_MP3 | INIT_FLAC | INIT_MOD | INIT_FLUIDSYNTH |
                                          INIT_MODPLUG |
                                          INIT_OGG)
                            .unwrap();

    let frequency = 44100;
    let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
    let channels = 2; // Stereo
    let chunk_size = 1024;
    let _ = sdl2_mixer::open_audio(frequency, format, channels, chunk_size).unwrap();
    sdl2_mixer::allocate_channels(0);

    {
        let n = sdl2_mixer::get_chunk_decoders_number();
        println!("available chunk(sample) decoders: {}", n);
        for i in 0..n {
            println!("  decoder {} => {}", i, sdl2_mixer::get_chunk_decoder(i));
        }
    }

    {
        let n = sdl2_mixer::get_music_decoders_number();
        println!("available music decoders: {}", n);
        for i in 0..n {
            println!("  decoder {} => {}", i, sdl2_mixer::get_music_decoder(i));
        }
    }

    println!("query spec => {:?}", sdl2_mixer::query_spec());


    let music = sdl2_mixer::Music::from_file(filename).unwrap();

    fn hook_finished() {
        println!("play ends! from rust cb");
    }

    sdl2_mixer::Music::hook_finished(hook_finished);

    println!("music => {:?}", music);
    println!("music type => {:?}", music.get_type());
    println!("music volume => {:?}", sdl2_mixer::Music::get_volume());
    println!("play => {:?}", music.play(1));

    timer.delay(10000);

    println!("fading out ... {:?}", sdl2_mixer::Music::fade_out(4000));

    timer.delay(5000);

    println!("fading in from pos ... {:?}",
             music.fade_in_from_pos(1, 10000, 100.0));

    timer.delay(5000);
    sdl2_mixer::Music::halt();
    timer.delay(1000);


    println!("quitting sdl");
}
