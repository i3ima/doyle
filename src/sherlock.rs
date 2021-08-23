pub enum Status {
    Found,
    NotFound,
}

pub struct CheckResult {
    pub url: String,
    pub status: Status,
}

pub struct SherlockData {
    pub username: String,
    pub hosts: Vec<String>,
}

pub trait Sherlock {
    fn check_host(&self, host: &str) -> CheckResult;

    fn check_hosts(&self) -> Vec<CheckResult>;

    fn new(username: &str) -> Self;
}