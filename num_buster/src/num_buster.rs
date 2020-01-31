pub fn generate_cnonce() -> String {
    use rand::{distributions, Rng, thread_rng};

    let mut rng = thread_rng();
    let length = rng.gen_range(30, 50);
    rng.sample_iter(&distributions::Alphanumeric).take(length).collect()
}