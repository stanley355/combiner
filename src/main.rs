pub fn production_rate_per_hour(speed: u8) -> f64 {
    match speed {
        5..=8 => {
            println!("hello");
            let f = speed as f64;
            (f * 221.0) * 0.9
        },
        9..=10 => {
            let f = speed as f64;
            (f * 221.0) * 0.77
        },
        _ => speed as f64 * 221.0
    }
}

fn main() {
    println!("{}", production_rate_per_hour(8))
}
