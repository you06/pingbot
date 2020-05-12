use std::{fs::read_to_string, io::Error};

use serde::Deserialize;
use toml;

#[derive(Deserialize)]
pub struct Config {
    #[serde(rename = "slack-token")]
    pub slack_token: String,
    #[serde(rename = "slack-channel")]
    pub slack_channel: String,

    #[serde(rename = "github-token")]
    pub github_token: String,
    #[serde(default)]
    #[serde(rename = "repos")]
    pub repos: Vec<String>,
    #[serde(default)]
    #[serde(rename = "filter-labels")]
    pub filter_labels: Vec<String>,

    #[serde(rename = "discourse-base-url")]
    pub discourse_base_url: String,
    #[serde(default)]
    #[serde(rename = "discourse-categories")]
    pub discourse_categories: Vec<String>,
    #[serde(default)]
    #[serde(rename = "discourse-members")]
    pub discourse_members: Vec<String>,
}

impl Config {
    pub fn new(filename: String) -> Result<Self, Error> {
        let contents = read_to_string(filename)?;
        let config: Config = toml::from_str(&contents[..]).unwrap();
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_config() -> Result<Config, Error> {
        Config::new("config.example.toml".to_owned())
    }

    #[test]
    fn read_config() {
        let config = new_config().unwrap();
        // slack
        assert_eq!(config.slack_token, "slack-token");
        assert_eq!(config.slack_channel, "slack-channel");
        // github
        assert_eq!(config.github_token, "github-token");
        assert_eq!(config.repos, vec!("you06/pingbot"));
        assert_eq!(
            config.filter_labels,
            vec!("filter-label-1", "filter-label-2")
        );
        // discourse
        assert_eq!(config.discourse_base_url, "https://asktug.com");
        assert_eq!(
            config.discourse_categories,
            vec!("TiDB 用户问答", "TiDB 开发者")
        );
        assert_eq!(config.discourse_members, vec!("you06"));
    }
}
