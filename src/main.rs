mod letters;
mod sound;

fn main() {
    println!("Hello, world!");
    // sound::play_sound("A.mp3");

    let letter: letters::Letter = rand::random();
    println!("{:?}", letter);
}
