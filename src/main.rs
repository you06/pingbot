mod providers;
mod config;

use clap::Clap;
use providers::github::GitHub;
use config::Config;

#[derive(Clap)]
#[clap(version = "1.0", author = "you06")]
struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short = "c", long = "config", default_value = "config.toml")]
    config: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    let conf = Config::new(opts.config).unwrap();

    let client = GitHub::new(conf.github_token.to_owned());
    let user = client.get_user_result();
    println!("user: {:?}", user);
}
