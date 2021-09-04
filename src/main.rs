use std::env;

use std::time::Instant;
pub use watson::*;

fn main() {
    let now = Instant::now();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments")
    }
    let username = &args[1];
    let watson: WatsonData = WatsonBuilder::new(username).load_json(None).build();
    watson.check_hosts(&watson.hosts);
    println!("Total execution time is {}ms", now.elapsed().as_millis())
}
