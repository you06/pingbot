mod providers;

use providers::github::GitHub;

fn main() {
    let client = GitHub::new("".to_owned());
    let user = client.get_user_result();
    println!("user: {:?}", user);
}
