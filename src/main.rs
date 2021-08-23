use std::env;

pub use sherlock::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let username = &args[1];
    let sherlock: SherlockData = Sherlock::new(username);
    sherlock
        .check_hosts()
        .iter()
        .for_each(|result| println!("{}", result));
}
