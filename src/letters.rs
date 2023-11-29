use rand::{
    distributions::{Distribution, Standard},
    Rng,
}; // 0.8.0

#[derive(Debug)]
pub enum Letter {
    C,
    H,
    K,
    L,
    Q,
    R,
    S,
    T,
}

impl Distribution<Letter> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Letter {
        match rng.gen_range(0..=7) {
            0 => Letter::C,
            1 => Letter::H,
            2 => Letter::K,
            3 => Letter::L,
            4 => Letter::Q,
            5 => Letter::R,
            6 => Letter::S,
            7 => Letter::T,
            _ => Letter::T,
        }
    }
}
