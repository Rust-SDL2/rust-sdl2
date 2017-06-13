extern crate sdl2;

use sdl2::audio::{AudioCallback, AudioSpecDesired};
use sdl2::AudioSubsystem;
use std::time::Duration;
use std::sync::mpsc;
use std::i16;

const RECORDING_LENGTH_SECONDS: usize = 3;


struct Recording {
    record_buffer: Vec<i16>,
    pos: usize,
    done_sender: mpsc::Sender<Vec<i16>>,
    done: bool,
}

// Append the input of the callback to the record_buffer.
// When the record_buffer is full, send it to the main thread via done_sender.
impl AudioCallback for Recording {
    type Channel = i16;

    fn callback(&mut self, input: &mut [i16]) {
        if self.done {
            return;
        }

        for x in input {
            self.record_buffer[self.pos] = *x;
            self.pos += 1;
            if self.pos >= self.record_buffer.len() {
                self.done = true;
                self.done_sender.send(self.record_buffer.clone()).unwrap();
                break;
            }
        }
    }
}

fn record(audio_subsystem: &AudioSubsystem, desired_spec: &AudioSpecDesired) -> Vec<i16> {
    println!("Capturing {:} seconds... Please rock!", RECORDING_LENGTH_SECONDS);

    let (done_sender, done_receiver) = mpsc::channel();

    let capture_device = audio_subsystem.open_capture(None, &desired_spec, |spec| {
        println!("Capture Spec = {:?}", spec);
        Recording {
            record_buffer: vec![0; spec.freq as usize * RECORDING_LENGTH_SECONDS * spec.channels as usize],
            pos: 0,
            done_sender,
            done: false
        }
    }).unwrap();

    println!("AudioDriver: {:?}", capture_device.subsystem().current_audio_driver());
    capture_device.resume();

    // Wait until the recording is done.
    let recorded_vec = done_receiver.recv().unwrap();

    capture_device.pause();

    // Device is automatically closed when dropped.
    // Depending on your system it might be even important that the capture_device is dropped
    // before the playback starts.

    recorded_vec
}


/// Returns a percent value
fn calculate_average_volume(recorded_vec: &[i16]) -> f32 {
    let sum: i64 = recorded_vec.iter().map(|&x| (x as i64).abs()).sum();
    (sum as f32) / (recorded_vec.len() as f32) / (i16::MAX as f32) * 100.0
}

/// Returns a percent value
fn calculate_max_volume(recorded_vec: &[i16]) -> f32 {
    let max: i64 = recorded_vec.iter().map(|&x| (x as i64).abs()).max().unwrap();
    (max as f32) / (i16::MAX as f32) * 100.0
}


struct SoundPlayback {
    data: Vec<i16>,
    pos: usize,
}

impl AudioCallback for SoundPlayback {
    type Channel = i16;

    fn callback(&mut self, out: &mut [i16]) {
        for dst in out.iter_mut() {
            *dst = *self.data.get(self.pos).unwrap_or(&0);
            self.pos += 1;
        }
    }
}

fn replay_recorded_vec(audio_subsystem: &AudioSubsystem, desired_spec: &AudioSpecDesired, recorded_vec: Vec<i16>) {
    println!("Playing...");

    let playback_device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        println!("Playback Spec = {:?}", spec);
        SoundPlayback {
            data: recorded_vec,
            pos: 0,
        }
    }).unwrap();

    // Start playback
    playback_device.resume();

    std::thread::sleep(Duration::from_secs(RECORDING_LENGTH_SECONDS as u64));
    // Device is automatically closed when dropped
}


fn main() {
    let sdl_context = sdl2::init().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();

    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: None,
        samples: None
    };

    let recorded_vec = record(&audio_subsystem, &desired_spec);

    println!("Average Volume of your Recording = {:?}%", calculate_average_volume(&recorded_vec));
    println!("Max Volume of your Recording = {:?}%", calculate_max_volume(&recorded_vec));

    replay_recorded_vec(&audio_subsystem, &desired_spec, recorded_vec);
}
