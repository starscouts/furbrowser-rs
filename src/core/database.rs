use colored::Colorize;
use crate::models::error::FurbrowserResult;
use crate::models::vote::Score;
use crate::util::sql::SyncSQLFetch;
use rusqlite::Connection;

pub struct Database(pub(crate) Connection);

impl Database {
    pub fn new(file_path: &str, backward_compatibility: bool) -> FurbrowserResult<Self> {
        println!("{}", "Opening database...".bright_black());
        let connection = Connection::open(file_path)?;

        if !backward_compatibility {
            connection.execute("DROP TABLE IF EXISTS tags", ())?;
        }

        connection.execute("CREATE TABLE IF NOT EXISTS images (id INT NOT NULL, liked BOOL, disliked BOOL, PRIMARY KEY (id))", ())?;

        if backward_compatibility {
            connection.execute("CREATE TABLE IF NOT EXISTS published (id INT UNIQUE NOT NULL, processed BOOL NOT NULL, vote BOOL NOT NULL, PRIMARY KEY (id))", ())?;
        }

        Ok(Database(connection))
    }

    pub fn get_upvotes(&self) -> FurbrowserResult<i64> {
        self.0.fetch("SELECT COUNT(*) FROM images WHERE liked=1", 0)
    }

    pub fn get_downvotes(&self) -> FurbrowserResult<i64> {
        self.0
            .fetch("SELECT COUNT(*) FROM images WHERE disliked=1", 0)
    }

    pub fn get_score(&self) -> FurbrowserResult<Score> {
        let upvotes = self.get_upvotes()?;
        let downvotes = self.get_downvotes()?;

        Ok(Score { upvotes, downvotes })
    }
}
