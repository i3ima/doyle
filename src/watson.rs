use serde::de::{Error as SerdeError, Visitor};
use serde::{Deserialize, Deserializer};

#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    Found,
    NotFound,
}

#[derive(Debug)]
pub struct ErrorMsg {
    pub msgs: Vec<String>,
}

struct ErrorMsgVisitor;

impl<'de> Visitor<'de> for ErrorMsgVisitor {
    type Value = ErrorMsg;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("vec or string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: SerdeError,
    {
        Ok(ErrorMsg {
            msgs: vec![v.to_string()],
        })
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut sequence: Vec<String> = vec![];
        while let Some(value) = seq.next_element()? {
            sequence.push(value);
        }

        Ok(ErrorMsg { msgs: sequence })
    }
}

impl<'de> Deserialize<'de> for ErrorMsg {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ErrorMsgVisitor)
    }
}

#[derive(Debug, Deserialize)]
pub enum ErrorType {
    #[serde(alias = "status_code")]
    StatusCode,
    #[serde(alias = "message")]
    Msg,
    #[serde(alias = "response_url")]
    ResponseUrl,
}

#[derive(Deserialize, Debug)]
pub struct HostDetails {
    #[serde(alias = "errorMsg")]
    pub error_msg: Option<ErrorMsg>,

    #[serde(alias = "errorType")]
    pub error_type: ErrorType,

    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Host {
    pub name: HostDetails,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CheckResult {
    pub url: String,
    pub status: Status,
    pub execution_time: u128,
}

#[derive(Default)]
pub struct WatsonData {
    pub username: String,
    pub hosts: Vec<(String, HostDetails)>,
}

pub trait Watson {
    fn check_host(&self, host: &HostDetails) -> CheckResult;

    fn check_hosts(&self, hosts: &[(String, HostDetails)]) -> Vec<CheckResult>;

    fn builder() -> WatsonBuilder;
}

#[derive(Default)]
pub struct WatsonBuilder {
    pub username: String,
    pub hosts: Vec<(String, HostDetails)>,
}
