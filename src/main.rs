mod config;
mod providers;

use clap::Clap;
use config::Config;
use providers::discourse::Discourse;
use providers::github::GitHub;
use providers::slack::Slack;

#[derive(Clap)]
#[clap(version = "1.0", author = "you06")]
struct Opts {
    #[clap(short = "c", long = "config", default_value = "config.toml")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();
    let conf = Config::new(opts.config).unwrap();
    let mut report = "".to_owned();

    let github_client = GitHub::new(conf.github_token.to_owned());
    let user = github_client.get_user_result().await;
    println!("Current user: {}", user.unwrap());

    let issues = github_client.get_opened_issues(conf.repos.clone()).await?;
    report.push_str(&format!("{} no-reply issues in 3 days\n", issues.len())[..]);
    for issue in issues {
        report.push_str(&format!("{}\n", issue)[..]);
    }

    let discourse_client = Discourse::new(
        conf.discourse_base_url.to_owned(),
        conf.discourse_members.clone(),
    );
    let topics = discourse_client
        .find_no_reply_topics_by_categories(conf.discourse_categories.clone())
        .await?;

    report.push_str(&format!("\n\n{} no-reply topics in TUG\n", topics.len())[..]);
    for topic in topics {
        report.push_str(&format!("{}\n", topic)[..]);
    }

    if conf.slack_token != "" && conf.slack_channel != "" {
        let slack_client = Slack::new(conf.slack_token.clone());
        let _ = slack_client.send_message(conf.slack_channel.clone(), report).await?;
    } else {
        println!("{}", report);
    }
    Ok(())
}
