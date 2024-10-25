use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use colored::Colorize;
use serde::Deserialize;

use crate::core::database::Database;
use crate::error::FurbrowserResult;
use crate::models::config::Config;
use crate::models::request;
use crate::models::vote::ImageVote;
use crate::util::tui;

#[derive(Deserialize, Debug)]
pub struct Posts {
    pub posts: Vec<Post>,
}

#[derive(Deserialize, Debug)]
pub struct PostFile {
    pub width: u32,
    pub height: u32,
    pub ext: String,
    pub url: String,
}

type PostTags = HashMap<String, Vec<String>>;

#[derive(Deserialize, Debug)]
pub struct Post {
    pub id: u32,
    pub created_at: String,
    pub file: PostFile,
    pub tags: PostTags,
}

pub type Tags = HashSet<String>;

impl Post {
    pub fn tags(&self) -> Tags {
        self.tags.values().flatten().map(|s| s.to_owned()).collect()
    }

    pub fn inline_view(&self, iterm2: bool) -> FurbrowserResult<()> {
        if self.file.ext == "webm" || self.file.ext == "swf" {
            println!(
                "[{}]",
                tui::link(
                    &self.file.url,
                    "Video; please view externally or click here"
                ).cyan()
            );
            println!();
        } else if iterm2 {
            println!();
            tui::image(&self.file.url)?;
        } else {
            println!(
                "[{}]",
                tui::link(
                    &self.file.url,
                    "Image; please view externally or click here"
                ).cyan()
            );
            println!();
        }

        Ok(())
    }

    pub fn vote(
        &self,
        vote: &ImageVote,
        config: &Config,
        database: &Database,
    ) -> FurbrowserResult<()> {
        request::post(
            &format!(
                "https://{}/posts/{}/votes.json?no_unvote=true&score={}",
                config.domain, self.id, vote
            ),
            &config.user_agent,
            &config.secrets,
        )?;

        match vote {
            ImageVote::Up => {
                request::post(
                    &format!(
                        "https://{}/favorites.json?post_id={}",
                        config.domain, self.id
                    ),
                    &config.user_agent,
                    &config.secrets,
                )?;
            }
            ImageVote::Down => {
                request::delete(
                    &format!("https://{}/favorites/{}.json", config.domain, self.id),
                    &config.user_agent,
                    &config.secrets,
                )?;
            }
        }

        database.0.execute(
            "INSERT INTO images(id, liked, disliked) VALUES (?, ?, ?)",
            (self.id, String::from(vote), String::from(&vote.reverse())),
        )?;

        Ok(())
    }
}

impl Display for Post {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}      #{}, {}, {}x{}",
            "Image:".bold(),
            tui::link(
                &format!("https://e621.net/posts/{}", self.id),
                &self.id.to_string()
            ),
            self.created_at,
            self.file.width,
            self.file.height
        )?;
        let tags = self.tags().into_iter().collect::<Vec<String>>().join(", ");
        writeln!(f, "{}       {}", "Tags:".bold(), tags)?;
        Ok(())
    }
}
