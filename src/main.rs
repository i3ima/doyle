use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

pub use watson::*;

fn parse_from_file() -> Vec<String> {
    BufReader::new(File::open("./list.txt").expect("File not found"))
        .lines()
        .map(|line| line.unwrap())
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments")
    }
    let username = &args[1];
    let watson: WatsonData = Watson::new(username, parse_from_file());
    watson
        .check_hosts(&watson.hosts)
        .iter()
        .for_each(|result| println!("{}", result));
}
