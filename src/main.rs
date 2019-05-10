use std::env;

use warp::{self, Filter};

fn main() {
    let mut args = env::args();
    match args.nth(1) {
        Some(val) => match val.as_str() {
            "owo" => println!("uwu"),
            "uwu" => println!("owo"),
            "OwO" => println!("UwU"),
            "UwU" => println!("OwO"),
            "serve" => {
                match args.next() {
                    Some(ref v) if v == "please" || v == "pwease" => {},
                    _ => {
                        println!("Say pwease!");
                        return;
                    },
                }

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
