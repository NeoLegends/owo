use std::env;

fn main() {
    match env::args().nth(1) {
        Some(val) => match val.as_str() {
            "owo" => println!("uwu"),
            "uwu" => println!("owo"),
            _ => println!("kawaii"),
        },
        _ => println!("q.q"),
    }
}
