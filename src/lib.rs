use async_trait::async_trait;
use colored::Colorize;
use rayon::prelude::*;
use reqwest::StatusCode;
pub use crate::watson::*;
use futures::future::join_all;
use std::future::Future;
use std::pin::Pin;
use std::time::Instant;
use std::fmt::Display;

mod watson;

pub use crate::watson::*;

impl Display for CheckResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result_str = format!("{} {}ms elapsed", self.url, self.execution_time);
        if let Status::Found = self.status {
            write!(f, "{} {}", "[OK]".green(), result_str)
        } else {
            write!(f, "{} {}", "[ERR]".red(), result_str)
        }
    }
}
impl Watson for WatsonData {
    async fn check_host(&self, host: &str) -> CheckResult {
        let check_url = host.to_string() + &self.username;
        println!("Checking - {}", &check_url);
        let now = Instant::now();
        // rn this is stupid asf
        // TODO: Replace this with more clever solution
        match reqwest::get(&check_url).await?.status() {
            StatusCode::OK => CheckResult {
                url: check_url,
                status: Status::Found,
                execution_time: now.elapsed().as_millis(),
            },
            StatusCode::NOT_FOUND => CheckResult {
                url: check_url,
                status: Status::NotFound,
                execution_time: now.elapsed().as_millis(),
            },
            _ => CheckResult {
                url: check_url,
                status: Status::NotFound,
                execution_time: now.elapsed().as_millis(),
            },
        }
    }

    async fn check_hosts(&self, hosts: &Vec<String>) -> Vec<CheckResult> {
        let futures: Vec<Pin<Box<dyn Future<Output = CheckResult> + Send + '_>>> = hosts
            .iter()
            .map(move |host| self.check_host(host))
            .collect();
        join_all(futures).await
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
                url: String::from("https://vk.com/") + &watson.username,
                execution_time: 0,
            },
            watson.check_host("https://vk.com/")
        )
    }

    #[test]
    fn check_hosts_test() {
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
                execution_time: 0,
            })
            .collect();
        let watson: WatsonData = Watson::new(username, hosts);

        assert_eq!(*results, watson.check_hosts(&watson.hosts))
    }
}
