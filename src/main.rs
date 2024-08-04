use rand::rngs::OsRng;
use rand::{thread_rng, Rng};

fn x<R: Rng>(rng: &mut R) -> (u32, u32) {
    (rng.gen(), rng.gen())
}

fn main() {
    dbg!(x(&mut OsRng));
    dbg!(x(&mut thread_rng()));
}
