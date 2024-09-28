use std::io::Write;
use std::io;
use crate::models::config::{get_blacklist, Config};
pub use crate::models::error;
use crate::models::error::FurbrowserResult;
use crate::models::vote::ImageVote;
use crate::util::ui;

mod models;
mod util;
mod core;

const USER_AGENT: &str = "Mozilla/5.0 (+furbrowser-rs; by Starscouts on e621)";

pub fn interactive() -> FurbrowserResult<()> {
    let connection = core::database::open_database("./history.db")?;
    let config = Config::build()?;

    let mut page = 1;
    loop {
        let blacklist = get_blacklist("./blacklist_safe.txt")?;

        ui::clear()?;
        println!("Downloading page {page}...");
        let mut data = core::e621::page(
            "score:>=0 rating:safe status:active -fav:Starscouts -voted:anything zootopia -fan_character",
            page, &config.secrets)?;
        page += 1;

        data = core::e621::filter_page(data, &blacklist, &connection)?;

        for image in data.posts {
            let likes = core::database::get_likes(&connection)?;
            let dislikes = core::database::get_dislikes(&connection)?;

            let blacklist = get_blacklist("./blacklist_safe.txt")?;
            let tags = image.tags();
            if tags.intersection(&blacklist).count() > 0 {
                println!("Blacklist was updated and image {} is now ignored", image.id);
                continue;
            }

            ui::clear()?;
            println!("{likes}/{dislikes}");
            println!("{}", image);

            image.inline_view()?;

            let like = ImageVote::from(ui::yesno("Upvote or downvote?", "u", "d")?);

            ui::clear()?;
            println!("Publishing vote...");
            io::stdout().flush()?;

            connection.execute("INSERT INTO images(id, liked, disliked) VALUES (?, ?, ?)",
               (image.id, String::from(&like), String::from(&like.reverse()))
            )?;

            image.vote(&like, &config.secrets)?;

            connection.execute("INSERT INTO published VALUES (?, TRUE, ?)",
                (image.id, String::from(&like))
            )?;

            connection.cache_flush()?;
        }
    }
}