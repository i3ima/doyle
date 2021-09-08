//! # Doyle
//!
//! `doyle` is a tool for quick search of social-media accounts by username

#![deny(missing_docs)]
#![deny(unreachable_pub)]

pub use crate::doyle::*;
use colored::Colorize;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use reqwest::header;
pub use reqwest::{Response, StatusCode};
use std::collections::HashMap;
use std::fmt::Display;
use std::time::Instant;

mod doyle;

impl Display for CheckResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result_str = format!("{} {}ms elapsed", self.url, self.execution_time);
        write!(f, "{} {}", "[OK]".green(), result_str)
    }
}

impl Doyle for DoyleData {
    /// Makes check for provided host
    ///
    /// # Arguments
    ///
    /// * `host` - struct of HostDetails type
    ///
    /// # Example
    /// ```
    /// use doyle::*;
    ///
    /// let doyle: DoyleData = DoyleBuilder::new("i3ima").load_json(None).build();
    /// doyle.check_host(&HostDetails {
    ///    error_type: ErrorType::StatusCode,
    ///    url: "https://vk.com/{}".to_string(),
    ///    url_probe: None,
    ///    error_msg: None
    /// }); 
    /// ```
    fn check_host(&self, host_details: &HostDetails) -> CheckResult {
        let check_url = match &host_details.url_probe {
            Some(url) => url.replace("{}", &self.username),
            None => host_details.url.replace("{}", &self.username),
        };
        let now = Instant::now();
        let mut headers = header::HeaderMap::new();
        // Insert user-agent, cuz some websites are retards
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64)"),
        );

        let client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let request = match client.get(&check_url).send() {
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
        match host_details {
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
                        status: Status::NotFound,
                        url,
                    }
                } else {
                    CheckResult {
                        execution_time: elapsed,
                        status: Status::Found,
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

    /// Performs check for all hosts
    ///
    /// Arguments
    ///
    /// * `hosts` - list of hosts
    ///
    /// Example
    /// ```
    /// use doyle::*;
    /// 
    /// let doyle: DoyleData = DoyleBuilder::new("i3ima").load_json(None).build();
    /// doyle.check_hosts(&doyle.hosts);
    /// ```
    fn check_hosts(&self, hosts: &[(String, HostDetails)]) -> Vec<CheckResult> {
        ThreadPoolBuilder::new()
            .num_threads(16)
            .build_global()
            .unwrap();
        hosts
            .par_iter()
            .map(|host| {
                let result = self.check_host(&host.1);
                if let Status::Found = result.status {
                    println!("{}", result);
                }
                result
            })
            .collect()
    }

    fn builder() -> DoyleBuilder {
        DoyleBuilder::default()
    }
}

impl DoyleBuilder {
    /// Load json either from file or from provided argument
    ///
    /// # Arguments 
    ///
    /// * `data` - vec with hosts, optional
    pub fn load_json(mut self, data: Option<Vec<(String, HostDetails)>>) -> Self {
        if let Some(json) = data {
            println!("Using JSON from user input");
            self.hosts = json;
        } else {
            let json = include_str!("../assets/data.json");
            let h: HashMap<String, HostDetails> =
                serde_json::from_str(json).expect("Cannot read json");
            println!("Using JSON from file");
            self.hosts = h.into_iter().collect();
        }
        self
    }

    /// Returns a new Doyle builder
    ///
    /// # Arguments
    ///
    /// * `username` - A string slice that holds username that gonna be searched
    ///
    /// # Examples
    ///
    /// ```
    /// use doyle::*;
    ///
    /// let doyle: DoyleData = DoyleBuilder::new("i3ima").load_json(None).build();
    /// doyle.check_hosts(&doyle.hosts); 
    /// ```
    pub fn new(username: &str) -> DoyleBuilder {
        DoyleBuilder {
            username: username.to_string(),
            hosts: vec![],
        }
    }
    /// Returns instance of doyle
    pub fn build(self) -> DoyleData {
        DoyleData {
            hosts: self.hosts,
            username: self.username,
        }
    }
}
