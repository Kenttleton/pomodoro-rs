use std::env;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};

fn main() {
    let mut duration_pomodoro = 25 * 60;
    let mut duration_break = 5 * 60;
    let mut sound_path = String::from("audio/success-fanfare-trumpets-6185.mp3");

    let mut args = env::args();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-d" => {
                if let Some(n) = args.next() {
                    duration_pomodoro = n.parse::<u64>().unwrap_or(duration_pomodoro) * 60;
                }
            }
            "-b" => {
                if let Some(n) = args.next() {
                    duration_break = n.parse::<u64>().unwrap_or(duration_break) * 60;
                }
            }
            "-s" => {
                if let Some(s) = args.next() {
                    sound_path = s;
                }
            }
            _ => {}
        }
    }

    println!("Welcome to the Pomodoro timer!");
    println!("Press enter to start timer");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    loop {
        println!("Starting a Pomodoro for {} minute(s)...", duration_pomodoro / 60);
        timer(duration_pomodoro);
        play_sound(sound_path);

        println!("Pomodoro complete! Taking a {} minute break...", duration_break / 60);
        timer(duration_break);
        play_sound(sound_path);
    }
}

fn timer(duration: u64) {
    let now = Instant::now();
    let end = now + Duration::from_secs(duration);

    io::stdout().flush().unwrap();
    while Instant::now() < end {
        let remaining = end - Instant::now();
        print!("\r{:02}:{:02} remaining", remaining.as_secs() / 60, remaining.as_secs() % 60);
        io::stdout().flush().unwrap();
        sleep(Duration::from_secs(1));
    }
}

fn play_sound(sound_path: String) {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(sound_path).unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples());

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(source.total_duration().unwrap());

    //println!("\x07"); // ASCII bell character, plays a beep sound
}