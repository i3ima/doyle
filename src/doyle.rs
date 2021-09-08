//! # doyle-inner
//! 
//! Contains a description of data model and core traits

#![allow(missing_docs)]
use serde::de::{Error as SerdeError, Visitor};
use serde::{Deserialize, Deserializer};



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

// Data model often contains situations where ErrorMsg is either Array of strings or
// a string. Serde by default cannot deserialize this, so I implemented Deserialize for
// ErrorMsg
impl<'de> Deserialize<'de> for ErrorMsg {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ErrorMsgVisitor)
    }
}

/// Represents result of check
#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    Found,
    NotFound,
}

/// Used if ErrorType is message to check presence of string in body
#[derive(Debug)]
pub struct ErrorMsg {
    pub msgs: Vec<String>,
}

/// Type of error that host should return
#[derive(Debug, Deserialize)]
pub enum ErrorType {
    /// HTTP Statuc code
    #[serde(alias = "status_code")]
    StatusCode,
    /// A string in the response body
    #[serde(alias = "message")]
    Msg,
    /// Specific url
    #[serde(alias = "response_url")]
    ResponseUrl,
}

/// Details about host
#[derive(Deserialize, Debug)]
pub struct HostDetails {
    #[serde(alias = "errorMsg")]
    pub error_msg: Option<ErrorMsg>,
    /// Optional. Used if host has a separate and known API. 
    #[serde(alias = "urlProbe")]
    pub url_probe: Option<String>,

    #[serde(alias = "errorType")]
    pub error_type: ErrorType,

    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Host {
    pub details: HostDetails,
}

/// Result of check
#[derive(Debug, PartialEq, Eq)]
pub struct CheckResult {
    pub url: String,
    pub status: Status,
    /// Time elapsed since start of check
    pub execution_time: u128,
}

#[derive(Default)]
pub struct DoyleData {
    pub username: String,
    pub hosts: Vec<(String, HostDetails)>,
}

pub trait Doyle {
    fn check_host(&self, host: &HostDetails) -> CheckResult;

    fn check_hosts(&self, hosts: &[(String, HostDetails)]) -> Vec<CheckResult>;

    fn builder() -> DoyleBuilder;
}

#[derive(Default)]
pub struct DoyleBuilder {
    pub username: String,
    pub hosts: Vec<(String, HostDetails)>,
}
