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
#[allow(dead_code)]
pub struct Posts {
    pub posts: Vec<Post>
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct PostFile {
    pub width: u32,
    pub height: u32,
    pub ext: String,
    pub size: u32,
    pub md5: String,
    pub url: String
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct PostPreview {
    pub width: u32,
    pub height: u32,
    pub url: String
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct PostSample {
    pub has: bool,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub url: Option<String>
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct PostScore {
    pub up: i32,
    pub down: i32,
    pub total: f32
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct PostFlags {
    pub pending: bool,
    pub flagged: bool,
    pub note_locked: bool,
    pub status_locked: bool,
    pub rating_locked: bool,
    pub deleted: bool
}

type PostTagList = Vec<String>;
type PostTags = HashMap<String, PostTagList>;

type PostSources = Vec<String>;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Post {
    pub id: u32,
    pub created_at: String,
    pub updated_at: String,
    pub file: PostFile,
    pub preview: PostPreview,
    pub sample: PostSample,
    pub score: PostScore,
    pub tags: PostTags,
    pub change_seq: u32,
    pub rating: String,
    pub fav_count: u32,
    pub sources: PostSources,
    pub approver_id: Option<u32>,
    pub uploader_id: u32,
    pub description: String,
    pub comment_count: u32,
    pub is_favorited: bool,
    pub has_notes: bool,
    pub duration: Option<f32>,
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