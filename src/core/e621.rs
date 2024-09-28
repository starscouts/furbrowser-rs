use std::collections::HashSet;
use std::time::Duration;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use rusqlite::Connection;
use urlencoding::encode;
use crate::models::config::{Blacklist, Secrets};
use crate::models::error::FurbrowserResult;
use crate::models::post::Posts;
use crate::USER_AGENT;
use crate::util::sql;

pub fn page(tags: &str, page: usize, secrets: &Secrets) -> FurbrowserResult<Posts> {
    let tags = encode(tags);
    let response = ureq::get(&format!("https://e621.net/posts.json?limit=320&tags={tags}&page={page}"))
        .timeout(Duration::from_millis(5000))
        .set("User-Agent", USER_AGENT)
        .set("Authorization", &format!("Basic {}",
            BASE64_STANDARD.encode(format!("{}:{}", secrets.user_name, secrets.api_key))))
        .call()?;

    println!("Decoding data...");
    Ok(response.into_json()?)
}

pub fn filter_page(mut posts: Posts, blacklist: &Blacklist, connection: &Connection) -> FurbrowserResult<Posts> {
    posts.posts = posts.posts.into_iter()
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