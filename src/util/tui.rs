#[allow(deprecated)] // We don't support Windows
use std::env::home_dir;
use std::io::Write;
use std::process::{Command, ExitStatus, Stdio};
use std::{env, io, thread};
use std::time::Duration;
use colored::Colorize;
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
    
    let mut query = if let Some(query) = profile.query.clone() {
        vec![query]
    } else if let Some(queries) = profile.queries.clone() {
        queries
    } else {
        return Err(Box::new(FurbrowserError::MissingQuery));
    };

    if let Some(extra_query) = &extra_query {
        for part in &mut query {
            part.push(' ');
            part.push_str(extra_query);
        }
    }

    loop {
        let blacklist = get_blacklist(&profile.blacklist_file)?;

        clear()?;
        println!("{}", format!("Downloading page {page}...").bright_black());

        let mut data = loop {
            let result = crate::core::e621::page(&query, page, &config);
            if let Ok(page) = result {
                break page;
            } else {
                println!("{}", "Failed to download page, retrying...".bright_black());
                println!("{}", format!("Failed to download page ({}), retrying...", *result.unwrap_err()).bright_black());
                thread::sleep(Duration::from_secs(1));
            }
        };

        page += 1;
        data = crate::core::e621::filter_page(data, &blacklist, &database)?;

        for post in data.posts {
            let score = database.get_score()?;

            let blacklist = get_blacklist(&profile.blacklist_file)?;
            let tags = post.tags();

            if tags.intersection(&blacklist.0).next().is_some() {
                println!("{}", format!("Blacklist was updated and image {} is now ignored", post.id).bright_black());
                continue;
            }

            clear()?;
            println!("{}  {} up, {} down, {} total",
                     "Breakdown:".bold(),
                     score.upvotes,
                     score.downvotes,
                     score.upvotes + score.downvotes
            );
            println!("{}", post);

            post.inline_view(env::var("ITERM_PROFILE").is_ok())?;

            let vote = ImageVote::from(yes_no(&format!("{}", "Upvote or downvote?".yellow()), "u", "d")?);

            clear()?;
            println!("{}", "Publishing vote...".bright_black());

            io::stdout().flush()?;

            let mut attempt = 1;
            loop {
                if attempt > 5 {
                    break;
                }
                let result = post.vote(&vote, &config, &database);
                if result.is_ok() {
                    break;
                } else {
                    println!("{}", format!("Failed to publish vote ({}), retrying... (attempt {}/5)", *result.unwrap_err(), attempt).bright_black());
                    thread::sleep(Duration::from_secs(1));
                }
                attempt += 1;
            }

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
        print!("{prompt} {}", format!("({yes}/{no})").bright_black());
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

pub fn image(url: &str) -> FurbrowserResult<ExitStatus> {
    #[allow(deprecated)] // We don't support Windows
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
