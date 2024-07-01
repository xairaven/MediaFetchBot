use rand::{distributions::Alphanumeric, Rng};

pub fn alphanumeric_string(symbols_amount: i32) -> String {
    let mut rng = rand::thread_rng();
    let random_string: String = (0..symbols_amount)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect();

    random_string
}