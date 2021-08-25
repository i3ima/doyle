use colored::Colorize;
use rayon::prelude::*;
use reqwest::StatusCode;

use std::fmt::Display;

mod watson;

pub use crate::watson::*;

impl Display for CheckResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Status::Found = self.status {
            write!(f, "{} {}", "[OK]".green(), self.url)
        } else {
            write!(f, "{} {}", "[ERR]".red(), self.url)
        }
    }
}
impl Watson for WatsonData {
    fn check_host(&self, host: &str) -> CheckResult {
        let check_url = host.to_string() + &self.username;
        println!("Checking - {}", &check_url);
        // rn this is stupid asf
        // TODO: Replace this with more clever solution
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

    fn check_hosts(&self, hosts: &Vec<String>) -> Vec<CheckResult> {
        hosts
            .par_iter()
            .map(move |host| self.check_host(host))
            .collect()
    }

    fn new(username: &str, hosts: Vec<String>) -> WatsonData {
        WatsonData {
            username: username.to_string(),
            hosts,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_host_test() {
        let watson: WatsonData = Watson::new("i3ima", vec![String::from("https://vk.com/")]);
        assert_eq!(
            CheckResult {
                status: Status::Found,
                url: String::from("https://vk.com/") + &watson.username
            },
            watson.check_host("https://vk.com/")
        )
    }

    #[test]
    fn chech_hosts_test() {
        let username = "i3ima";
        let hosts = vec![
            String::from("https://vk.com/"),
            String::from("https://github.com/"),
        ];

        let results: &Vec<CheckResult> = &hosts
            .iter()
            .map(|host| CheckResult {
                status: Status::Found,
                url: format!("{}{}", host, username),
            })
            .collect();
        let watson: WatsonData = Watson::new(username, hosts);

        assert_eq!(*results, watson.check_hosts(&watson.hosts))
    }
}
