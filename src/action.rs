use rand::RngExt;

pub fn roll(d: u8) -> u8 {
    let mut rng = rand::rng();
    rng.random_range(1..=d) as u8
}