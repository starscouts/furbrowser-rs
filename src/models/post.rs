use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::time::Duration;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use serde::Deserialize;
use crate::error::FurbrowserResult;
use crate::models::config::Config;
use crate::models::vote::ImageVote;
use crate::util::ui;
use crate::VERSION;

#[derive(Deserialize)]
pub struct Posts(pub Vec<Post>);

#[derive(Deserialize, Debug)]
pub struct PostFile {
    pub width: u32,
    pub height: u32,
    pub ext: String,
    pub url: String
}

type PostTagList = Vec<String>;
type PostTags = HashMap<String, PostTagList>;

#[derive(Deserialize, Debug)]
pub struct Post {
    pub id: u32,
    pub created_at: String,
    pub file: PostFile,
    pub tags: PostTags
}

pub type Tags = HashSet<String>;

impl Post {
    pub fn tags(&self) -> Tags {
        self.tags.values()
            .flatten()
            .map(|s| s.to_owned())
            .collect()
    }

    pub fn inline_view(&self, iterm2: bool) -> FurbrowserResult<()> {
        if self.file.ext == "webm" || self.file.ext == "swf" {
            println!("[{esc}]8;;{}{esc}\\Video; please view externally or click here{esc}]8;;{esc}\\]",
                self.file.url, esc = 27 as char);
            println!();
        } else {
            if iterm2 {
                println!();
                ui::image(&self.file.url)?;
            } else {
                println!("[{esc}]8;;{}{esc}\\Image; please view externally or click here{esc}]8;;{esc}\\]",
                    self.file.url, esc = 27 as char);
                println!();
            }
        }

        Ok(())
    }

    pub fn vote(&self, vote: &ImageVote, config: &Config) -> FurbrowserResult<()> {
        ureq::post(&format!("https://{}/posts/{}/votes.json?no_unvote=true&score={}", config.domain, self.id, match vote {
            ImageVote::Up => 1,
            ImageVote::Down => -1
        }))
            .timeout(Duration::from_millis(5000))
            .set("User-Agent", &config.user_agent.replace("VERSION", VERSION))
            .set("Authorization", &format!("Basic {}",
                BASE64_STANDARD.encode(format!("{}:{}", config.secrets.user_name, config.secrets.api_key))))
            .call()?;

        match vote {
            ImageVote::Up => {
                ureq::post(&format!("https://{}/favorites.json?post_id={}", config.domain, self.id))
                    .timeout(Duration::from_millis(5000))
                    .set("User-Agent", &config.user_agent.replace("VERSION", VERSION))
                    .set("Authorization", &format!("Basic {}",
                        BASE64_STANDARD.encode(format!("{}:{}", config.secrets.user_name, config.secrets.api_key))))
                    .call()?;
            }
            ImageVote::Down => {
                ureq::delete(&format!("https://{}/favorites/{}.json", config.domain, self.id))
                    .timeout(Duration::from_millis(5000))
                    .set("User-Agent", &config.user_agent.replace("VERSION", VERSION))
                    .set("Authorization", &format!("Basic {}",
                        BASE64_STANDARD.encode(format!("{}:{}", config.secrets.user_name, config.secrets.api_key))))
                    .call()?;
            }
        }

        Ok(())
    }
}

impl Display for Post {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Image {esc}]8;;https://e621.net/posts/{id}{esc}\\{id}{esc}]8;;{esc}\\; {}; {}x{}",
            self.created_at, self.file.width, self.file.height, esc = 27 as char, id = self.id)?;
        writeln!(f, "Tags: {}", self.tags().into_iter().collect::<Vec<String>>().join(", "))?;
        Ok(())
    }
}