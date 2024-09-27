use std::fmt::{Display, Formatter};
use std::{fs, io};
use std::collections::{HashMap, HashSet};
use std::time::Duration;
use log::{error, info, LevelFilter};
use serde::Deserialize;
use simple_logger::SimpleLogger;
use urlencoding::encode;
use base64::prelude::*;
use sqlite::Connection;

#[derive(Deserialize)]
struct Posts {
    pub posts: Vec<Post>
}

#[derive(Deserialize, Debug)]
struct PostFile {
    pub width: u32,
    pub height: u32,
    pub ext: String,
    pub size: u32,
    pub md5: String,
    pub url: String
}

#[derive(Deserialize, Debug)]
struct PostPreview {
    pub width: u32,
    pub height: u32,
    pub url: String
}

#[derive(Deserialize, Debug)]
struct PostSample {
    pub has: bool,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub url: Option<String>
}

#[derive(Deserialize, Debug)]
struct PostScore {
    pub up: i32,
    pub down: i32,
    pub total: f32
}

#[derive(Deserialize)]
struct PostFlags {
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
struct Post {
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

#[derive(Deserialize)]
struct Secrets {
    #[serde(alias = "id")]
    user_name: String,
    #[serde(alias = "key")]
    api_key: String
}

#[derive(Debug)]
enum FurbrowserError {
    SQL(sqlite::Error),
    LocalIO(io::Error),
    JSON(serde_json::Error),
    HTTP(ureq::Error)
}

impl From<sqlite::Error> for FurbrowserError {
    fn from(value: sqlite::Error) -> Self {
        Self::SQL(value)
    }
}

impl From<io::Error> for FurbrowserError {
    fn from(value: io::Error) -> Self {
        Self::LocalIO(value)
    }
}

impl From<serde_json::Error> for FurbrowserError {
    fn from(value: serde_json::Error) -> Self {
        Self::JSON(value)
    }
}

impl From<ureq::Error> for FurbrowserError {
    fn from(value: ureq::Error) -> Self {
        Self::HTTP(value)
    }
}

impl Display for FurbrowserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FurbrowserError::SQL(e) => write!(f, "SQL Error: {e}"),
            FurbrowserError::LocalIO(e) => write!(f, "Local I/O Error: {e}"),
            FurbrowserError::JSON(e) => write!(f, "JSON Error: {e}"),
            FurbrowserError::HTTP(e) => write!(f, "HTTP Error: {e}")
        }
    }
}

type FurbrowserResult<T> = Result<T, FurbrowserError>;

fn remove_existing(connection: &Connection, posts: Posts) -> FurbrowserResult<Posts> {
    let mut new_posts = vec![];

    for post in posts.posts {
        if connection.prepare(&format!("SELECT COUNT(*) FROM images WHERE id={}", post.id))?
            .read::<i64, _>("COUNT(*)")? == 0 {
            new_posts.push(post);
        }
    }

    Ok(Posts { posts: new_posts })
}

fn run() -> FurbrowserResult<()> {
    info!("Opening database...");
    let connection = sqlite::open("../historya.db")?;
    connection.execute("CREATE TABLE IF NOT EXISTS tags (name TEXT NOT NULL, likes INT, dislikes INT, total INT, PRIMARY KEY (name))")?;
    connection.execute("CREATE TABLE IF NOT EXISTS images (id INT NOT NULL, liked BOOL, disliked BOOL, tags LONGTEXT, PRIMARY KEY (id))")?;

    info!("Opening secrets...");
    let secrets: Secrets = serde_json::from_str(&fs::read_to_string("./secrets.json")?)?;
    info!("Authenticated as {}", secrets.user_name);

    let mut page = 1;
    loop {
        let blacklist_file = fs::read_to_string("./blacklist_safe.txt")?;
        let blacklist: HashSet<&str> = blacklist_file
            .trim().split("\n")
            .filter(|i| i.trim() != "" && !i.trim().starts_with("#"))
            .collect();

        info!("Downloading page {page}...");
        let likes = connection.prepare("SELECT COUNT(*) FROM images WHERE liked=1")?.read::<i64, _>("COUNT(*)")?;
        let dislikes = connection.prepare("SELECT COUNT(*) FROM images WHERE disliked=1")?.read::<i64, _>("COUNT(*)")?;

        let tags = encode("score:>=0 rating:safe status:active -fav:Starscouts -voted:anything zootopia -fan_character");
        let response = ureq::get(&format!("https://e621.net/posts.json?limit=320&tags={tags}&page={page}"))
            .timeout(Duration::from_millis(5000))
            .set("User-Agent", "Mozilla/5.0 (+furbrowser-rs; by Starscouts on e621)")
            .set("Authorization", &format!("Basic {}",
                BASE64_STANDARD.encode(format!("{}:{}", secrets.user_name, secrets.api_key))))
            .call()?;

        info!("Decoding data...");
        let mut data: Posts = response.into_json()?;
        page += 1;

        data.posts = data.posts.into_iter()
            .filter(|i| {
                let tags: HashSet<&str> = i.tags.values()
                    .flatten()
                    .map(|s| s.as_str())
                    .collect();
                tags.intersection(&blacklist).count() == 0
            })
            .collect();

        data = remove_existing(&connection, data)?;

        // TODO: See JS version (line 155)

        break;
    }

    Ok(())
}

fn main() {
    SimpleLogger::new()
        .with_module_level("rustls", LevelFilter::Info)
        .with_module_level("ureq", LevelFilter::Info)
        .init()
        .unwrap();

    if let Err(e) = run() {
        error!("An error has occurred: {e}")
    }
}
