use std::io::Write;
use std::{env, io};
use crate::error::FurbrowserError;
use crate::models::config::{get_blacklist, Config};
pub use crate::models::error;
use crate::models::error::FurbrowserResult;
use crate::models::vote::ImageVote;
use crate::util::ui;

mod models;
mod util;
mod core;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn interactive(profile: &str, append: Option<String>) -> FurbrowserResult<()> {
    let config = Config::build()?;
    let profile = config.profiles.get(profile).ok_or(FurbrowserError::NoSuchProfile)?;
    let connection = core::database::open_database(&config.database, config.backward_compatibility)?;

    let mut page = 1;
    loop {
        let blacklist = get_blacklist(&profile.blacklist)?;
        let query = if let Some(append) = &append {
            &format!("{} {append}", &profile.query)
        } else {
            &profile.query
        };

        ui::clear()?;
        println!("Downloading page {page}...");
        let mut data = core::e621::page(
            query,
            page, &config)?;
        page += 1;

        data = core::e621::filter_page(data, &blacklist, &connection)?;

        for image in data.posts {
            let likes = core::database::get_likes(&connection)?;
            let dislikes = core::database::get_dislikes(&connection)?;

            let blacklist = get_blacklist(&profile.blacklist)?;
            let tags = image.tags();
            if tags.intersection(&blacklist).count() > 0 {
                println!("Blacklist was updated and image {} is now ignored", image.id);
                continue;
            }

            ui::clear()?;
            println!("{likes}/{dislikes}");
            println!("{}", image);

            image.inline_view(match env::var("ITERM_PROFILE") {
                Ok(_) => true,
                _ => false
            })?;

            let like = ImageVote::from(ui::yesno("Upvote or downvote?", "u", "d")?);

            ui::clear()?;
            println!("Publishing vote...");
            io::stdout().flush()?;

            connection.execute("INSERT INTO images(id, liked, disliked) VALUES (?, ?, ?)",
               (image.id, String::from(&like), String::from(&like.reverse()))
            )?;

            image.vote(&like, &config)?;

            if config.backward_compatibility {
                connection.execute("INSERT INTO published VALUES (?, TRUE, ?)",
                    (image.id, String::from(&like))
                )?;
            }

            connection.cache_flush()?;
        }
    }
}