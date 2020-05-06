mod config;
mod providers;

use clap::Clap;
use config::Config;
use providers::discourse::Discourse;
use providers::github::GitHub;

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

    let github_client = GitHub::new(conf.github_token.to_owned());
    let user = github_client.get_user_result().await;
    println!("Current user: {}", user.unwrap());

    let issues = github_client.get_opened_issues(conf.repos.clone()).await?;
    println!("{} no-reply issues in 3 days", issues.len());
    for issue in issues {
        println!("{}", issue);
    }

    let discourse_client = Discourse::new(
        conf.discourse_base_url.to_owned(),
        conf.discourse_members.clone(),
    );
    let topics = discourse_client
        .find_no_reply_topics_by_categories(conf.discourse_categories.clone())
        .await?;

    for topic in topics {
        println!("{}", topic);
    }
    Ok(())
}
