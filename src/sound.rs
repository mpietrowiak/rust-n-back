use rodio::{source::Source, Decoder, OutputStream};
use std::env;
use std::fs::File;
use std::io::BufReader;

pub fn play_sound(filename: &str) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let current_dir = env::current_dir().unwrap();
    let relative_path = current_dir.join("assets").join("sounds").join(filename);

    println!("Playing sound: {}", filename);

    let file = BufReader::new(File::open(relative_path).unwrap());
    let source = Decoder::new(file).unwrap();
    let _ = stream_handle.play_raw(source.convert_samples());

    std::thread::sleep(std::time::Duration::from_secs(1));
}
