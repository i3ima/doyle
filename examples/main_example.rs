use std::env;

use std::time::Instant;
use doyle::*;

fn main() {
    let now = Instant::now();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments")
    }
    let username = &args[1];
    let doyle = DoyleBuilder::new(username).load_json(None).build();
    doyle.check_hosts(&doyle.hosts);
    println!("Total execution time is {}ms", now.elapsed().as_millis())
}