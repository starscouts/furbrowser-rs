use rusqlite::Connection;
use crate::models::error::FurbrowserResult;
use crate::util::sql::SyncSQLFetch;

pub fn open_database(file_path: &str) -> FurbrowserResult<Connection> {
    println!("Opening database...");
    let connection = Connection::open(file_path)?;
    connection.execute("DROP TABLE IF EXISTS tags", ())?;
    connection.execute("CREATE TABLE IF NOT EXISTS images (id INT NOT NULL, liked BOOL, disliked BOOL, PRIMARY KEY (id))", ())?;
    connection.execute("CREATE TABLE IF NOT EXISTS published (id INT UNIQUE NOT NULL, processed BOOL NOT NULL, vote BOOL NOT NULL, PRIMARY KEY (id))", ())?;
    Ok(connection)
}

pub fn get_likes(connection: &Connection) -> FurbrowserResult<i64> {
    Ok(connection.fetch("SELECT COUNT(*) FROM images WHERE liked=1", 0)?)
}

pub fn get_dislikes(connection: &Connection) -> FurbrowserResult<i64> {
    Ok(connection.fetch("SELECT COUNT(*) FROM images WHERE disliked=1", 0)?)
}