use rand::Rng;
use rand::rngs::SmallRng;

pub fn random_number(rng: &mut SmallRng) -> i32 {
    (rng.gen::<u32>() % 9999 + 1) as i32
}