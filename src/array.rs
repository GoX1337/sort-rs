use rand::Rng;

pub fn build_random_array(size: u8, max_value: u8) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut values = Vec::new();
    for _i in 0..size {
        values.push(rng.gen_range(0, max_value));
    }
    values
}