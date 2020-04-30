use std::{
    fmt,
    error::Error,
    convert::From
};

use github_rs::client::{Executor, Github};
use github_rs::errors::Error as OriginError;
use serde::{Deserialize, Serialize};
use serde_json::{
    Value,
    error::Error as JsonError
};

#[derive(Debug)]
pub struct GithubError {
    reason: String,
}

impl fmt::Display for GithubError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl Error for GithubError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl From<JsonError> for GithubError {
    fn from(err: JsonError) -> Self {
        GithubError{
            reason: err.to_string(),
        }
    }
}

impl From<OriginError> for GithubError {
    fn from(err: OriginError) -> Self {
        GithubError{
            reason: err.to_string(),
        }
    }
}

pub struct GitHub {
    pub token: String,
    client: Github,
}

#[derive(Serialize, Deserialize)]
struct User {
    login: String,
}

impl GitHub {
    pub fn new(token: String) -> GitHub {
        GitHub {
            token: token.to_owned(),
            client: Github::new(token).unwrap(),
        }
    }

    pub fn get_user_result(&self) -> Result<String, GithubError> {
        let me = self.client.get().user().execute::<Value>();
        match me {
            Ok((_, status, json)) => {
                if !status.is_success() {
                    return Err(GithubError{reason: format!("HTTP status error, {}", status)});
                }
                if let Some(v) = json.unwrap().get("name") {
                    Ok(v.to_string())
                } else {
                    Err(GithubError{reason: "get field failed".to_owned()})
                }
            }
            Err(e) => Err(e.into()),
        }
    }
}
