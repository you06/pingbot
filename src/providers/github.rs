use std::{
    fmt,
    error::Error,
    convert::From
};

use github_rs::client::{Executor, Github};
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
            reason: "".to_owned(),
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
                if status.is_success() {
                    return Err(GithubError{reason: "HTTP status error".to_owned()});
                }
                println!("json: {:?}", json);
                if let Some(v) = json.unwrap().get("name") {
                    Ok(v.to_string())
                } else {
                    Err(GithubError{reason: "get field failed".to_owned()})
                }
                // Ok(json.unwrap().get("name").unwrap())
            }
            Err(e) => Err(GithubError{reason: "Unknown error".to_owned()}),
        }
    }
}
