pub use crate::watson::*;
use colored::Colorize;
pub use reqwest::{Response, StatusCode};
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;

mod watson;

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
    fn check_host(&self, host: &HostDetails) -> CheckResult {
        let check_url = host.url.replace("{}", &self.username);
        let now = Instant::now();
        let request = match reqwest::blocking::get(&check_url) {
            Ok(resp) => resp,
            Err(error) => {
                println!("Failed host check because of: {}", error);
                return CheckResult {
                    url: check_url,
                    status: Status::NotFound,
                    execution_time: 0,
                };
            }
        };
        let elapsed = now.elapsed().as_millis();
        let url = request.url().to_string();
        // rn this is stupid asf
        // TODO: Replace this with more clever solution
        match host {
            HostDetails {
                error_type: ErrorType::StatusCode,
                ..
            } => {
                if request.status().is_success() {
                    CheckResult {
                        execution_time: elapsed,
                        status: Status::Found,
                        url,
                    }
                } else {
                    CheckResult {
                        execution_time: elapsed,
                        status: Status::NotFound,
                        url,
                    }
                }
            }
            HostDetails {
                error_msg: m,
                error_type: ErrorType::Msg,
                ..
            } => {
                let m = match m {
                    Some(msg) => msg,
                    None => panic!("No msg"),
                };
                if request.text().unwrap().contains(&m.msgs[0]) {
                    CheckResult {
                        execution_time: elapsed,
                        status: Status::Found,
                        url,
                    }
                } else {
                    CheckResult {
                        execution_time: elapsed,
                        status: Status::NotFound,
                        url,
                    }
                }
            }
            _ => CheckResult {
                execution_time: elapsed,
                status: Status::NotFound,
                url,
            },
        }
    }

    fn check_hosts(&self, hosts: &[(String, HostDetails)]) -> Vec<CheckResult> {
        hosts
            .iter()
            .map(move |host| {
                let result = self.check_host(&host.1);
                println!("{}", result);
                result
            })
            .collect()
    }

    fn builder() -> WatsonBuilder {
        WatsonBuilder::default()
    }
}

impl WatsonBuilder {
    pub fn load_json(mut self, data: Option<Vec<(String, HostDetails)>>) -> Self {
        if let Some(json) = data {
            println!("Using JSON from user input");
            self.hosts = json;
        } else {
            let file = File::open("./data.json").unwrap();
            let reader = BufReader::new(file);
            let h: HashMap<String, HostDetails> =
                serde_json::from_reader(reader).expect("Cannot read json");
            println!("Using JSON from file");
            self.hosts = h.into_iter().collect();
        }
        self
    }

    pub fn new(username: &str) -> WatsonBuilder {
        WatsonBuilder {
            username: username.to_string(),
            hosts: vec![],
        }
    }

    pub fn build(self) -> WatsonData {
        WatsonData {
            hosts: self.hosts,
            username: self.username,
        }
    }
}
