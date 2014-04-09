#![crate_type = "bin"]
#![crate_id="demo"]

extern crate sdl2;
extern crate sdl2_mixer;

use std::os;
use mix = sdl2_mixer;


fn main() {
    let args = os::args();
    if args.len() < 2 {
        println!("Usage: ./demo audio.[mp3|wav|ogg]")
    } else {
        match dump_info(&Path::new(args[1])) {
            _ => ()
        }
    }
}

fn dump_info(filename: &Path) -> Result<(), ~str> {
    println!("linked version: {}", mix::get_linked_version());
    sdl2::init([sdl2::InitAudio, sdl2::InitTimer]);
    println!("inited => {}", mix::init([mix::InitMp3, mix::InitFlac]));

    // TODO: 0x8010 is SDL_audio flag
    try!(mix::open_audio(mix::DEFAULT_FREQUENCY, 0x8010u16, 2, 1024));
    mix::allocate_channels(0);

    {
        let n = mix::get_chunk_decoders_number();
        println!("available chunk(sample) decoders: {}", n);
        for i in range(0, n) {
            println!("| decoder {} => {:?}", i, mix::get_chunk_decoder(i));
        }
    }

    {
        let n = mix::get_music_decoders_number();
        println!("available music decoders: {}", n);
        for i in range(0, n) {
            println!("| decoder {} => {:?}", i, mix::get_music_decoder(i));
        }
    }

    println!("query spec => {}", mix::query_spec());

    let music = mix::Music::from_file(filename).unwrap();
    println!("music => {:?}", music);
    println!("music type => {}", music.get_type());

    println!("music volume => {}", mix::Music::get_volume());

    println!("play => {}", music.play(1));

    sdl2::timer::delay(10000);

    println!("fading out ... {}", mix::Music::fade_out(4000));

    sdl2::timer::delay(5000);

    println!("fading in from pos ... {}", music.fade_in_from_pos(1, 10000, 100.0));

    sdl2::timer::delay(5000);

    mix::quit();
    sdl2::quit();

    Ok(())
}
