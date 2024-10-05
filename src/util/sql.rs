use rusqlite::types::FromSql;
use rusqlite::Connection;

use crate::core::database::Database;
use crate::error::{FurbrowserError, FurbrowserResult};
use crate::models::post::Posts;

pub trait SyncSQLFetch {
    fn fetch<T: FromSql>(&self, query: &str, column: usize) -> FurbrowserResult<T>;
}

impl SyncSQLFetch for Connection {
    fn fetch<T: FromSql>(&self, query: &str, column: usize) -> FurbrowserResult<T> {
        let mut statement = self.prepare(query)?;
        let mut data_iter = statement.query_map([], |row| {
            row.get::<_, T>(column)
        })?;

        Ok(data_iter.next().ok_or(FurbrowserError::SyncSQLFetch)??)
    }
}

pub fn remove_existing(database: &Database, posts: Posts) -> FurbrowserResult<Posts> {
    let mut new_posts = vec![];

    for post in posts.posts {
        let query = &format!("SELECT COUNT(*) FROM images WHERE id={}", post.id);
        let count = database.0.fetch::<i64>(query, 0)?;

        if count == 0 {
            new_posts.push(post);
        }
    }

    Ok(Posts { posts: new_posts })
}
