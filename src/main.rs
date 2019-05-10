use std::env;

use warp::{self, Filter};

fn main() {
    match env::args().nth(1) {
        Some(val) => match val.as_str() {
            "owo" => println!("uwu"),
            "uwu" => println!("owo"),
            "OwO" => println!("UwU"),
            "UwU" => println!("OwO"),
            "serve" => {
                let handler = warp::any()
                    .map(|| "owo")
                    .map(|owo| warp::reply::with_header(owo, "Host", "Kawaii"));
                warp::serve(handler).run(([127, 0, 0, 1], 8080));
            },
            _ => println!("kawaii"),
        },
        _ => println!("q.q"),
    }
}
