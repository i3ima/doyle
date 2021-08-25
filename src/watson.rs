#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    Found,
    NotFound,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CheckResult {
    pub url: String,
    pub status: Status,
}

pub struct WatsonData {
    pub username: String,
    pub hosts: Vec<String>,
}

pub trait Watson {
    fn check_host(&self, host: &str) -> CheckResult;

    fn check_hosts(&self, hosts: &Vec<String>) -> Vec<CheckResult>;

    fn new(username: &str, hosts: Vec<String>) -> Self;
}
