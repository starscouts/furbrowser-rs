use std::collections::HashSet;
use std::time::Duration;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use rusqlite::Connection;
use urlencoding::encode;
use crate::models::config::{Blacklist, Config};
use crate::models::error::FurbrowserResult;
use crate::models::post::Posts;
use crate::VERSION;
use crate::util::sql;

pub fn page(tags: &str, page: usize, config: &Config) -> FurbrowserResult<Posts> {
    let tags = encode(tags);
    let url = &format!("https://{}/posts.json?limit={}&tags={tags}&page={page}", config.domain, config.posts_per_page);
    let response = ureq::get(url)
        .timeout(Duration::from_millis(5000))
        .set("User-Agent", &config.user_agent.replace("VERSION", VERSION))
        .set("Authorization", &format!("Basic {}",
            BASE64_STANDARD.encode(format!("{}:{}", config.secrets.user_name, config.secrets.api_key))))
        .call()?;

    println!("Decoding data...");
    Ok(response.into_json()?)
}

pub fn filter_page(mut posts: Posts, blacklist: &Blacklist, connection: &Connection) -> FurbrowserResult<Posts> {
    posts.0 = posts.0.into_iter()
        .filter(|i| {
            let tags: HashSet<String> = i.tags.values()
                .flatten()
                .map(|i| i.to_owned())
                .collect();
            tags.intersection(&blacklist).count() == 0
        })
        .collect();

    Ok(sql::remove_existing(&connection, posts)?)
}