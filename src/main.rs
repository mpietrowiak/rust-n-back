mod letters;
mod sound;

fn main() {
    println!("Hello, world!");
    let letter: letters::Letter = rand::random();
    println!("Random letter: {:?}", letter);
    sound::play_sound(&format!("{:?}.mp3", letter));
}
