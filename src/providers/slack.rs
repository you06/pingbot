use std::fmt;

use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;

const API_BASE_URL: &str = "https://slack.com/api";

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    reason: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error {
            reason: err.to_string(),
        }
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Self {
        Error {
            reason: err.to_string(),
        }
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error { reason: err }
    }
}

pub struct Slack {
    token: String,
    client: reqwest::Client,
}

struct Header {
    key: String,
    value: String,
}

#[derive(Serialize)]
struct Message {
    text: String,
    channel: String,
}

#[derive(Deserialize, Serialize)]
struct Response {
    ok: bool,
    error: Option<String>,
}

impl Slack {
    pub fn new(token: String) -> Self {
        let mut auth_header = "Bearer ".to_owned();
        auth_header.push_str(&token);
        Slack {
            token: auth_header,
            client: reqwest::Client::new(),
        }
    }

    async fn request(&self, url: &str, headers: Vec<Header>, body: String) -> Result<String> {
        let mut req = self
            .client
            .post(url)
            .header(reqwest::header::USER_AGENT, "pingbot")
            .header(reqwest::header::AUTHORIZATION, &self.token[..])
            .header(reqwest::header::CONTENT_TYPE, "application/json");
        for header in headers {
            req = req.header(&header.key[..], &header.value[..]);
        }
        let res = req.body(body).send().await?.text().await?;
        Ok(res)
    }

    pub async fn send_message(&self, channel: String, text: String) -> Result<()> {
        let url = format!("{}/{}", API_BASE_URL, "chat.postMessage");
        let message = Message { text, channel };
        let body = serde_json::to_string(&message)?;
        let res_text = self.request(&url[..], vec![], body).await?;
        let res: Response = serde_json::from_str(&res_text[..])?;
        match res.ok {
            true => Ok(()),
            false => match res.error {
                Some(e) => Err(e.into()),
                None => Err("unknown error".to_owned().into()),
            },
        }
    }
}
