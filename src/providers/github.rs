use std::{
    fmt,
    convert::From
};

use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::error::Error as JsonError;

const GITHUB_BASE_URL: &str = "https://api.github.com";
const PER_PAGE: usize = 100;

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

pub struct GitHub {
    pub token: String,
    client: reqwest::Client,
}

struct Header {
    key: String,
    value: String,
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
pub struct User {
    login: String,
}

#[derive(Serialize, Deserialize)]
pub struct Pull {
    html_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Issue {
    number: i32,
    title: String,
    #[serde(skip_deserializing)]
    owner: String,
    #[serde(skip_deserializing)]
    repo: String,
    pull_request: Option<Pull>,
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

    async fn request(&self, url: &str, headers: Vec<Header>) -> Result<String> {
        let mut req = self.client.get(url)
            .header(reqwest::header::USER_AGENT, "pingbot")
            .header(reqwest::header::AUTHORIZATION, &self.token[..]);
        for header in headers {
            req = req.header(&header.key[..], &header.value[..]);
        }
        let res = req.send()
            .await?
            .text()
            .await?;
        Ok(res)
    }

    pub async fn get_user_result(&self) -> Result<String> {
        let url = format!("{}/user", GITHUB_BASE_URL);
        let res = self.request(&url[..], vec!()).await?;
        let u: User = serde_json::from_str(&res[..])?;
        Ok(u.login.to_owned())
    }

    pub async fn get_opened_issues(&self, raw: Vec<String>) -> Result<Vec<Issue>> {
        let repos = parse_repos(raw);
        let mut opened_all = vec!(); 
        for repo in repos {
            println!("process {}/{}", repo.owner, repo.repo);
            let issues = self.get_opened_issues_by_repo(&repo).await?;
            opened_all.extend(issues);
        }
        println!("{} opened issues & pulls at all", opened_all.len());
        let opened_issues: Vec<Issue> = opened_all.into_iter().filter(|issue| issue.pull_request.is_none()).collect();
        println!("{} opened issues at all", opened_issues.len());
        Ok(opened_issues)
    }

    async fn get_opened_issues_by_repo(&self, repo: &Repo) -> Result<Vec<Issue>> {
        let mut all = Vec::<Issue>::new();
        let mut page = 0;

        while all.len() == page * PER_PAGE {
            page += 1;
            let url = format!("{}/repos/{}/{}/issues?page={}&per_page={}",
                GITHUB_BASE_URL, repo.owner, repo.repo, page, PER_PAGE);
            let headers = vec!(Header{key: "Accept".to_owned(), value: "application/vnd.github.machine-man-preview".to_owned()});
            let res = self.request(&url[..], headers).await?;
            let batch: Vec<Issue> = serde_json::from_str(&res[..])?;
            all.extend(batch);
        }
        println!("{} opened issues in {}/{}", all.len(), repo.owner, repo.repo);

        Ok(all.into_iter().map(|mut issue| {
            issue.owner = repo.owner.to_owned();
            issue.repo = repo.repo.to_owned();
            issue
        }).collect())
    }
}

fn parse_repos(raw: Vec<String>) -> Vec<Repo> {
    raw.into_iter().map(Into::into).collect()
}
