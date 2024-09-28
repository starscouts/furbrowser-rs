use std::collections::{HashMap, HashSet};
use std::{env, fs};
use std::path::Path;
use serde::Deserialize;
use crate::models::error::FurbrowserResult;

#[derive(Deserialize)]
pub struct Secrets {
    #[serde(alias = "id")]
    pub user_name: String,
    #[serde(alias = "key")]
    pub api_key: String
}

#[derive(Deserialize)]
pub struct Config {
    pub backward_compatibility: bool,
    pub user_agent: String,
    pub posts_per_page: usize,
    pub domain: String,
    pub database: String,
    pub secrets: Secrets,
    pub profiles: Profiles
}

type Profiles = HashMap<String, Profile>;

#[derive(Deserialize)]
pub struct Profile {
    pub blacklist: String,
    pub query: String
}

impl Config {
    pub fn build() -> FurbrowserResult<Self> {
        println!("Loading configuration...");
        let config: Self = if Path::new("./config.toml").exists() {
            toml::from_str(&fs::read_to_string("./config.toml")?)?
        } else {
            toml::from_str(&fs::read_to_string(&format!("{}/.furbrowserrc", env::var("HOME")?))?)?
        };
        println!("Hello, {}!", config.secrets.user_name);

        Ok(config)
    }
}

pub type Blacklist = HashSet<String>;

pub fn get_blacklist(file_path: &str) -> FurbrowserResult<Blacklist> {
    let blacklist = fs::read_to_string(file_path)?;
    Ok(blacklist
        .trim().split("\n")
        .filter_map(|i| {
            if i.trim() != "" && !i.trim().starts_with("#") {
                Some(i.trim().to_owned())
            } else {
                None
            }
        })
        .collect())
}