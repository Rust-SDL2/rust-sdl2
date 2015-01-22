#![crate_type = "bin"]
#![crate_name="demo"]

extern crate sdl2;
extern crate sdl2_mixer;

use std::os;
use std::borrow::ToOwned;

fn main() {
    let args = os::args();
    if args.len() < 2 {
        println!("Usage: ./demo audio.[mp3|wav|ogg]")
    } else {
        match dump_info(&Path::new(args[1].to_owned())) {
            _ => ()
        }
    }
}

fn dump_info(filename: &Path) -> Result<(), String> {
    println!("linked version: {:?}", sdl2_mixer::get_linked_version());
    sdl2::init(sdl2::INIT_AUDIO | sdl2::INIT_TIMER);
    println!("inited => {}", sdl2_mixer::init(sdl2_mixer::INIT_MP3 | sdl2_mixer::INIT_FLAC |
                                              sdl2_mixer::INIT_MOD | sdl2_mixer::INIT_FLUIDSYNTH |
                                              sdl2_mixer::INIT_MODPLUG | sdl2_mixer::INIT_OGG).bits());
    // TODO: 0x8010 is SDL_audio flag
    try!(sdl2_mixer::open_audio(sdl2_mixer::DEFAULT_FREQUENCY, 0x8010u16, 2, 1024));
    sdl2_mixer::allocate_channels(0);

    {
        let n = sdl2_mixer::get_chunk_decoders_number();
        println!("available chunk(sample) decoders: {}", n);
        for i in (0..n) {
            println!("| decoder {} => {:}", i, sdl2_mixer::get_chunk_decoder(i));
        }
    }

    {
        let n = sdl2_mixer::get_music_decoders_number();
        println!("available music decoders: {}", n);
        for i in (0..n) {
            println!("| decoder {} => {:}", i, sdl2_mixer::get_music_decoder(i));
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

    sdl2::timer::delay(10000);

    println!("fading out ... {:?}", sdl2_mixer::Music::fade_out(4000));

    sdl2::timer::delay(5000);

    println!("fading in from pos ... {:?}", music.fade_in_from_pos(1, 10000, 100.0));

    sdl2::timer::delay(5000);

    sdl2_mixer::Music::halt();

    sdl2::timer::delay(1000);
    // here will print hook_finished

    sdl2_mixer::quit();
    sdl2::quit();

    Ok(())
}
