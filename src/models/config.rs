use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::{env, fs};

use serde::Deserialize;

use crate::models::error::FurbrowserResult;

#[derive(Deserialize)]
pub struct Secrets {
    #[serde(alias = "id", alias = "user_name")]
    pub username: String,
    #[serde(alias = "key")]
    pub api_key: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub backward_compatibility: bool,
    pub user_agent: String,
    pub posts_per_page: usize,
    pub domain: String,
    pub database: String,
    pub secrets: Secrets,
    pub profiles: HashMap<String, Profile>,
}

#[derive(Deserialize)]
pub struct Profile {
    #[serde(alias = "blacklist")]
    pub blacklist_file: PathBuf,
    pub query: String,
}

impl Config {
    pub fn build() -> FurbrowserResult<Self> {
        println!("Loading configuration...");

        let config = Self::get_config_path()?;
        let config = fs::read_to_string(config)?;
        let config: Self = toml::from_str(&config)?;

        println!("Hello, {}!", config.secrets.username);

        Ok(config)
    }

    pub fn get_config_path() -> FurbrowserResult<PathBuf> {
        let local_path = Path::new("./config.toml");
        let path = if local_path.exists() {
            local_path.to_owned()
        } else {
            format!("{}/.furbrowserrc", env::var("HOME")?).into()
        };

        Ok(path)
    }
}

#[derive(Debug)]
pub struct Blacklist(pub(crate) HashSet<String>);

pub fn get_blacklist(file_path: &PathBuf) -> FurbrowserResult<Blacklist> {
    let blacklist = fs::read_to_string(file_path)?;
    Ok(Blacklist(
        blacklist
            .trim()
            .split("\n")
            .filter_map(|i| {
                if !i.is_empty() && !i.trim().starts_with("#") {
                    Some(i.trim().to_owned())
                } else {
                    None
                }
            })
            .collect(),
    ))
}
