fn rand_int(nmin: i32, nmax: i32, seed: u32) -> (i32, u32) {
    let mut seed: u32 = seed;
    seed ^= seed << 13;
    seed ^= seed >> 17;
    seed ^= seed << 5;
    let range = (nmax + 1 - nmin) as u32;
    let val = nmin + (seed % range) as i32;
    (val, seed)
}

fn time_seed() -> u32 {
    use std::time::SystemTime as st;
    let now = st::now().duration_since(st::UNIX_EPOCH).unwrap();
    now.as_millis() as u32
}

fn main() {
    let seed = time_seed();
    let rand = rand_int(1, 100, seed);
    println!("Random number from 1 to 100: {} - seed {}", rand.0, rand.1);
}
