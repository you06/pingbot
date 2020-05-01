use std::{fs::read_to_string, io::Error};

use serde::Deserialize;
use toml;

#[derive(Deserialize)]
pub struct Config {
    #[serde(rename = "github-token")]
    pub github_token: String,
    #[serde(default)]
    #[serde(rename = "repos")]
    pub repos: Vec<String>,
}

impl Config {
    pub fn new(filename: String) -> Result<Self, Error> {
        let contents = read_to_string(filename)?;
        let config: Config = toml::from_str(&contents[..]).unwrap();
        Ok(config)
    }
}
