#[allow(deprecated)]
use std::env::home_dir; // We don't support Windows
use std::io::Write;
use std::process::{Command, ExitStatus, Stdio};
use std::{env, io};

use crate::core::database::Database;
use crate::error::{FurbrowserError, FurbrowserResult};
use crate::models::config::{get_blacklist, Config};
use crate::models::vote::ImageVote;

pub fn start_tui(profile: &str, extra_query: Option<String>) -> FurbrowserResult<()> {
    let config = Config::build()?;
    let profile = config
        .profiles
        .get(profile)
        .ok_or(FurbrowserError::NoSuchProfile)?;
    let database = Database::new(&config.database, config.backward_compatibility)?;

    let mut page = 1;
    let mut query = profile.query.clone();

    if let Some(extra_query) = &extra_query {
        query.push_str(extra_query);
    }

    let query = &query;

    loop {
        let blacklist = get_blacklist(&profile.blacklist_file)?;

        clear()?;
        println!("Downloading page {page}...");
        let mut data = crate::core::e621::page(query, page, &config)?;

        page += 1;
        data = crate::core::e621::filter_page(data, &blacklist, &database)?;

        for post in data.posts {
            let score = database.get_score()?;

            let blacklist = get_blacklist(&profile.blacklist_file)?;
            let tags = post.tags();

            if tags.intersection(&blacklist.0).next().is_some() {
                println!("Blacklist was updated and image {} is now ignored", post.id);
                continue;
            }

            clear()?;
            println!("{}/{}", score.upvotes, score.downvotes);
            println!("{}", post);

            post.inline_view(env::var("ITERM_PROFILE").is_ok())?;

            let vote = ImageVote::from(yes_no("Upvote or downvote?", "u", "d")?);

            clear()?;
            println!("Publishing vote...");

            io::stdout().flush()?;

            post.vote(&vote, &config, &database)?;

            if config.backward_compatibility {
                database.0.execute(
                    "INSERT INTO published VALUES (?, TRUE, ?)",
                    (post.id, String::from(&vote)),
                )?;
            }

            database.0.cache_flush()?;
        }
    }
}

pub fn yes_no(prompt: &str, yes: &str, no: &str) -> FurbrowserResult<bool> {
    loop {
        print!("{}[2K\r", 27 as char);
        print!("{prompt} ({yes}/{no})");
        io::stdout().flush()?;

        let mut line = String::new();
        io::stdin().read_line(&mut line)?;

        if line.trim() == yes {
            return Ok(true);
        } else if line.trim() == no {
            return Ok(false);
        }
    }
}

pub fn clear() -> FurbrowserResult<()> {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Moves the cursor to the top left corner
    io::stdout().flush()?;
    Ok(())
}

#[allow(deprecated)] // We don't support Windows
pub fn image(url: &str) -> FurbrowserResult<ExitStatus> {
    let mut imgcat_path = home_dir().ok_or(FurbrowserError::NoValidHome)?;
    imgcat_path.push(".iterm2");
    imgcat_path.push("imgcat");

    Ok(Command::new(imgcat_path)
        .arg("-W")
        .arg("100%")
        .arg("-H")
        .arg("90%")
        .arg("-u")
        .arg(url)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?)
}

pub fn link(url: &str, text: &str) -> String {
    format!(
        "{esc}]8;;{url}{esc}\\{text}{esc}]8;;{esc}\\",
        esc = 27 as char
    )
}
