use std::collections::HashSet;
use std::time::Duration;

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use colored::Colorize;
use urlencoding::encode;

use crate::core::database::Database;
use crate::models::config::{Blacklist, Config};
use crate::models::error::FurbrowserResult;
use crate::models::post::Posts;
use crate::util::sql;
use crate::VERSION;

pub fn page(tags: &str, page: usize, config: &Config) -> FurbrowserResult<Posts> {
    let tags = encode(tags);
    let url = &format!(
        "https://{}/posts.json?limit={}&tags={tags}&page={page}",
        config.domain, config.posts_per_page
    );
    let response = ureq::get(url)
        .timeout(Duration::from_millis(5000))
        .set("User-Agent", &config.user_agent.replace("VERSION", VERSION))
        .set(
            "Authorization",
            &format!(
                "Basic {}",
                BASE64_STANDARD.encode(format!(
                    "{}:{}",
                    config.secrets.username, config.secrets.api_key
                ))
            ),
        )
        .call()?;

    println!("{}", "Decoding data...".bright_black());
    Ok(response.into_json()?)
}

pub fn filter_page(
    mut posts: Posts,
    blacklist: &Blacklist,
    database: &Database,
) -> FurbrowserResult<Posts> {
    posts.posts.retain(|i| {
        let tags: HashSet<String> = i.tags.values().flatten().map(|i| i.to_owned()).collect();
        tags.intersection(&blacklist.0).count() == 0
    });

    sql::remove_existing(database, posts)
}
