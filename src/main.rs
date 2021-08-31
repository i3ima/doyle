use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

pub use watson::*;
use std::time::Instant;

fn parse_from_file() -> Vec<String> {
    BufReader::new(File::open("./list.txt").expect("File not found"))
        .lines()
        .map(|line| line.unwrap())
        .collect()
}

#[tokio::main]
async fn main() {
    let now = Instant::now();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments")
    }
    let username = &args[1];
    let watson: WatsonData = Watson::new(username, parse_from_file());
    watson
        .check_hosts(&watson.hosts)
        .await
        .iter()
        .for_each(|result| println!("{}", result));
    println!("Total execution time is {}ms", now.elapsed().as_millis())
}
