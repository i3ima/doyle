use async_trait::async_trait;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    Found,
    NotFound,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CheckResult {
    pub url: String,
    pub status: Status,
    pub execution_time: u128,
}

pub struct WatsonData {
    pub username: String,
    pub hosts: Vec<String>,
}

#[async_trait]
pub trait Watson {
    async fn check_host(&self, host: &str) -> CheckResult;

    async fn check_hosts(&self, hosts: &Vec<String>) -> Vec<CheckResult>;

    fn new(username: &str, hosts: Vec<String>) -> Self;
}
