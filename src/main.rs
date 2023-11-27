/*
fn main() {
  println!("Hello, World!");
}
*/

use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};
use std::env;

fn main() {
  // Get a output stream handle to the default physical sound device
  let (_stream, stream_handle) = OutputStream::try_default().unwrap();

  // Get the current directory of the Rust program
  let current_dir = env::current_dir().unwrap();

  // Construct the relative path to the file
  let relative_path = current_dir
      .join("assets")
      .join("sounds")
      .join("A.mp3");

  // Load a sound from a file, using a path relative to Cargo.toml
  let file = BufReader::new(File::open(relative_path).unwrap());
  // Decode that sound file into a source
  let source = Decoder::new(file).unwrap();
  // Play the sound directly on the device
  let _ = stream_handle.play_raw(source.convert_samples());

  // The sound plays in a separate audio thread,
  // so we need to keep the main thread alive while it's playing.
  std::thread::sleep(std::time::Duration::from_secs(5));
}