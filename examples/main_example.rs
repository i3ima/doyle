use std::env;

use doyle::*;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments")
    }
    let username = &args[1];
    DoyleBuilder::new(username)
        .load_json(None)
        .build()
        .check_hosts();
    println!("Total execution time is {}ms", now.elapsed().as_millis())
}