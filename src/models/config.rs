use std::collections::HashSet;
use std::fs;
use serde::Deserialize;
use crate::models::error::FurbrowserResult;

#[derive(Deserialize)]
pub struct Secrets {
    #[serde(alias = "id")]
    pub user_name: String,
    #[serde(alias = "key")]
    pub api_key: String
}

pub struct Config {
    pub secrets: Secrets
}

impl Config {
    pub fn build() -> FurbrowserResult<Self> {
        println!("Opening secrets...");
        let secrets: Secrets = serde_json::from_str(&fs::read_to_string("./secrets.json")?)?;
        println!("Hello, {}!", secrets.user_name);

        Ok(Self {
            secrets
        })
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