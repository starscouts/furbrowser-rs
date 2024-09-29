use std::fmt::Debug;
use rusqlite::Connection;
use rusqlite::types::FromSql;
use crate::error::{FurbrowserError, FurbrowserResult};
use crate::models::post::Posts;

pub trait SyncSQLFetch {
    fn fetch<T: FromSql + Debug>(&self, query: &str, column: usize) -> FurbrowserResult<T>;
}

impl SyncSQLFetch for Connection {
    fn fetch<T: FromSql + Debug>(&self, query: &str, column: usize) -> FurbrowserResult<T> {
        let mut stmt = self.prepare(query)?;
        let mut data_iter = stmt.query_map([], |row| {
            Ok(row.get::<_, T>(column)?)
        })?;

        Ok(data_iter.next().ok_or(FurbrowserError::SyncSQLFetch)??)
    }
}

pub fn remove_existing(connection: &Connection, posts: Posts) -> FurbrowserResult<Posts> {
    let mut new_posts = vec![];

    for post in posts.0 {
        if connection.fetch::<i64>(&format!("SELECT COUNT(*) FROM images WHERE id={}", post.id), 0)? == 0 {
            new_posts.push(post);
        }
    }

    Ok(Posts(new_posts))
}