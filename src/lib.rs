use colored::Colorize;
use rayon::prelude::*;
use reqwest::StatusCode;

use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

mod sherlock;

pub use sherlock::*;

fn parse_from_file() -> Vec<String> {
    BufReader::new(File::open("./list.txt").unwrap())
        .lines()
        .map(|line| line.unwrap())
        .collect()
}

impl Display for CheckResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Status::Found = self.status {
            write!(f, "{} {}", "[OK]".green(), self.url)
        } else {
            write!(f, "{} {}", "[ERR]".red(), self.url)
        }
    }
}

impl Sherlock for SherlockData {
    fn check_host(&self, host: &str) -> CheckResult {
        let check_url = host.to_string() + &self.username;
        println!("Checking - {}", &check_url);
        match reqwest::blocking::get(&check_url).unwrap().status() {
            StatusCode::OK => CheckResult {
                url: check_url,
                status: Status::Found,
            },
            StatusCode::NOT_FOUND => CheckResult {
                url: check_url,
                status: Status::NotFound,
            },
            _ => CheckResult {
                url: check_url,
                status: Status::NotFound,
            },
        }
    }

    fn check_hosts(&self) -> Vec<CheckResult> {
        self.hosts
            .par_iter()
            .map(move |host| self.check_host(host))
            .collect()
    }

    fn new(username: &str) -> Self {
        SherlockData {
            username: username.to_string(),
            hosts: parse_from_file(),
        }
    }
}
