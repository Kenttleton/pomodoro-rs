use clap::Parser;
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, Instant};

#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = "A simple command line Pomodoro timer built in rust that displays a countdown and plays audio on transitions",
    author
)]
struct Args {
    #[arg(
        short,
        long,
        default_value_t = 25,
        help = "Duration in minutes for the working period of the timer"
    )]
    work: u64,

    #[arg(
        short,
        long,
        default_value_t = 5,
        help = "Duration in minutes for the rest period of the timer"
    )]
    rest: u64,

    #[arg(short, long, default_value_t = String::from("audio/success-fanfare-trumpets-6185.mp3"), help = "File path relative to Cargo.toml for an audio file to play on transitions")]
    path: String,
}

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let args = Args::parse();
    let work_duration = args.work * 60;
    let rest_duration = args.rest * 60;
    let sound_path = args.path;

    println!("Welcome to the Pomodoro timer!");
    println!("Press enter to start timer");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    while running.load(Ordering::SeqCst) {
        println!(
            "Starting a Pomodoro for {} minute(s)...",
            work_duration / 60
        );
        let mut end = timer(work_duration, &running);
        if !end {
            play_sound(sound_path.clone());

            println!(
                "Pomodoro complete! Taking a {} minute break...",
                rest_duration / 60
            );
            end = timer(rest_duration, &running);

            if !end {
                play_sound(sound_path.clone());
            }
        }
    }
}

fn timer(duration: u64, running: &Arc<AtomicBool>) -> bool {
    let now = Instant::now();
    let end = now + Duration::from_secs(duration);

    io::stdout().flush().unwrap();
    while Instant::now() < end && running.load(Ordering::SeqCst) {
        let remaining = end - Instant::now();
        print!(
            "\r{:02}:{:02} remaining",
            remaining.as_secs() / 60,
            remaining.as_secs() % 60
        );
        io::stdout().flush().unwrap();
        sleep(Duration::from_secs(1));
    }
    !running.load(Ordering::SeqCst)
}

fn play_sound(sound_path: String) {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Load a sound from a file, using a path relative to Cargo.toml
    match File::open(sound_path.clone()) {
        Ok(file) => {
            let buffer = BufReader::new(file);
            // Decode that sound file into a source
            match Decoder::new(buffer) {
                Ok(source) => {
                    let duration = match source.total_duration() {
                        Some(duration) => duration,
                        None => Duration::from_secs(3),
                    };

                    let sample = source.convert_samples();

                    // Play the sound directly on the device
                    match stream_handle.play_raw(sample) {
                        Ok(_) => {
                            println!();
                            // The sound plays in a separate audio thread,
                            // so we need to keep the main thread alive while it's playing.
                            std::thread::sleep(duration);
                        }
                        Err(err) => {
                            // ASCII bell character, plays a beep sound
                            println!("\x07");
                            print!("{}", err);
                            println!();
                        }
                    };
                }
                Err(err) => {
                    // ASCII bell character, plays a beep sound
                    println!("\x07");
                    print!("{}", err);
                    println!();
                }
            }
        }
        Err(err) => {
            // ASCII bell character, plays a beep sound
            println!("\x07");
            print!("{}", err);
            println!();
        }
    };
}
