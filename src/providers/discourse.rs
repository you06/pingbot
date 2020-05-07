use std::{
    collections::{HashMap, HashSet},
    convert::From,
    fmt,
};

use chrono::{DateTime, Utc};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::error::Error as JsonError;

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
        Error {
            reason: err.to_string(),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error {
            reason: err.to_string(),
        }
    }
}

pub struct Discourse {
    base_url: String,
    members: HashSet<String>,
    client: reqwest::Client,
}

struct Header {
    key: String,
    value: String,
}

#[derive(Serialize, Deserialize)]
pub struct Category {
    id: i32,
    name: String,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.id, self.name)
    }
}

#[derive(Serialize, Deserialize)]
pub struct CategoryList {
    categories: Vec<Category>,
}

#[derive(Serialize, Deserialize)]
pub struct Categories {
    category_list: CategoryList,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    id: i32,
    name: String,
    username: String,
}

#[derive(Serialize, Deserialize)]
pub struct Poster {
    user_id: i32,
    description: String,
}

#[derive(Serialize, Deserialize)]
pub struct Topic {
    id: i32,
    title: String,
    created_at: DateTime<Utc>,
    posters: Vec<Poster>,
    #[serde(skip_deserializing)]
    base_url: String,
}

impl fmt::Display for Topic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}, {}/t/topic/{}",
            self.id, self.title, self.base_url, self.id
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct TopicList {
    pub topics: Vec<Topic>,
}

#[derive(Serialize, Deserialize)]
pub struct CategoryContent {
    users: Vec<User>,
    topic_list: TopicList,
}

#[derive(Serialize, Deserialize)]
pub struct Comment {
    html_url: String,
    author_association: String,
}

impl Discourse {
    pub fn new(base_url: String, members: Vec<String>) -> Self {
        Discourse {
            base_url,
            members: members.into_iter().collect(),
            client: reqwest::Client::new(),
        }
    }

    fn if_member(&self, user: &String) -> bool {
        self.members.contains(user)
    }

    async fn request(&self, url: &str, headers: Vec<Header>) -> Result<String> {
        let mut req = self
            .client
            .get(url)
            .header(reqwest::header::USER_AGENT, "pingbot");
        for header in headers {
            req = req.header(&header.key[..], &header.value[..]);
        }
        let res = req.send().await?.text().await?;
        Ok(res)
    }

    async fn get_categories(&self) -> Result<Vec<Category>> {
        let url = format!("{}/categories.json", self.base_url);
        let res = self.request(&url[..], vec![]).await?;
        let c: Categories = serde_json::from_str(&res[..])?;
        Ok(c.category_list.categories)
    }

    async fn get_topics_by_cate(&self, cate: &Category) -> Result<CategoryContent> {
        let url = format!("{}/c/{}.json", self.base_url, cate.id);
        let res = self.request(&url[..], vec![]).await?;
        let c: CategoryContent = serde_json::from_str(&res[..])?;
        Ok(c)
    }

    async fn find_no_reply_topics_by_category(&self, cate: &Category) -> Result<Vec<Topic>> {
        let mut no_reply_topics = vec![];
        let category_content = self.get_topics_by_cate(&cate).await?;
        let pingcap_user_set: HashMap<i32, bool> = category_content
            .users
            .into_iter()
            .map(|user| {
                (
                    user.id,
                    self.if_member(&user.username)
                        || is_pingcap_user(&user.name)
                        || is_pingcap_user(&user.username),
                )
            })
            .collect();

        'outer: for topic in category_content.topic_list.topics {
            for poster in &topic.posters {
                match pingcap_user_set.get(&poster.user_id) {
                    Some(&u) => {
                        if u {
                            continue 'outer;
                        }
                    }
                    None => {
                        println!("user id {} not in list", &poster.user_id);
                    }
                }
            }
            no_reply_topics.push(topic);
        }
        Ok(no_reply_topics)
    }

    pub async fn find_no_reply_topics_by_categories(
        &self,
        categories: Vec<String>,
    ) -> Result<Vec<Topic>> {
        let base_url = self.base_url.to_owned();
        let mut no_reply_topics = vec![];
        let cate_set: HashSet<String> = categories.into_iter().collect();
        let categories = self.get_categories().await?;
        for cate in categories {
            if cate_set.contains(&cate.name) {
                println!("Finding no-reply topics in {}", cate);
                let cate_no_reply_topics = self.find_no_reply_topics_by_category(&cate).await?;
                no_reply_topics.extend(cate_no_reply_topics);
            }
        }
        Ok(no_reply_topics
            .into_iter()
            .map(|mut topic| {
                topic.base_url = base_url.to_owned();
                topic
            })
            .collect())
    }
}

fn is_pingcap_user(name: &String) -> bool {
    name.ends_with("-PingCAP") || name.ends_with("- PingCAP")
}
