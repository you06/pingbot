mod config;
mod providers;

use clap::Clap;
use config::Config;
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

    let client = GitHub::new(conf.github_token.to_owned());
    let user = client.get_user_result().await;
    println!("Current user: {}", user.unwrap());

    let issues = client.get_opened_issues(conf.repos.clone()).await?;
    println!("{} no-reply issues in 3 days", issues.len());
    for issue in issues {
        println!("{}", issue);
    }

    Ok(())
}
