use std::{
    fmt,
    convert::From
};

use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{
    Value,
    error::Error as JsonError
};

const GITHUB_BASE_URL: &str = "https://api.github.com";

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

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Self {
        Error{
            reason: err.to_string(),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error{
            reason: err.to_string(),
        }
    }
}

// impl From<OriginError> for GithubError {
//     fn from(err: OriginError) -> Self {
//         GithubError{
//             reason: err.to_string(),
//         }
//     }
// }

pub struct GitHub {
    pub token: String,
    client: reqwest::Client,
}

struct Repo {
    owner: String,
    repo: String,
}

impl From<String> for Repo {
    fn from(r: String) -> Self {
        let parsed = r.split("/").map(Into::into).collect::<Vec<String>>();
        Repo{
            owner: parsed[0].to_owned(),
            repo: parsed[1].to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct User {
    login: String,
}


#[derive(Serialize, Deserialize)]
struct Issue {
    number: i32,
    title: String,
}

impl GitHub {
    pub fn new(token: String) -> Self {
        let mut auth_header = "token ".to_owned();
        auth_header.push_str(&token);
        GitHub {
            token: auth_header,
            client: reqwest::Client::new(),
        }
    }

    async fn request(&self, url: &str) -> Result<String> {
        let res = self.client.get(url)
            .header(reqwest::header::USER_AGENT, "pingbot")
            .header(reqwest::header::AUTHORIZATION, &self.token[..])
            .send()
            .await?
            .text()
            .await?;
        Ok(res)
    }

    pub async fn get_user_result(&self) -> Result<String> {
        let url = format!("{}/user", GITHUB_BASE_URL);
        let res = self.request(&url[..]).await?;
        let u: User = serde_json::from_str(&res[..])?;
        Ok(u.login.to_owned())
    }

    pub fn get_opened_issues(&self, raw: Vec<String>) {
        let repos = parse_repos(raw);
        // let opened_issues = repos.into_iter().map(|repo| self.get_opened_issues_by_repo(&repo)).collect();
    }

    fn get_opened_issues_by_repo(&self, repo: &Repo) -> Vec<Issue> {
        // let issues = self.client.get().issues().list().execute::<Value>();
        vec!()
    }
}

fn parse_repos(raw: Vec<String>) -> Vec<Repo> {
    raw.into_iter().map(Into::into).collect()
}
