use std::fmt::{Display, Formatter};
use std::io;

#[derive(Debug)]
pub enum FurbrowserError {
    SQL(rusqlite::Error),
    LocalIO(io::Error),
    JSON(serde_json::Error),
    HTTP(ureq::Error),
    SyncSQLFetch,
    Readline
}

impl From<rusqlite::Error> for FurbrowserError {
    fn from(value: rusqlite::Error) -> Self {
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
            FurbrowserError::HTTP(e) => write!(f, "HTTP Error: {e}"),
            FurbrowserError::SyncSQLFetch => write!(f, "SQL Processing Error"),
            FurbrowserError::Readline => write!(f, "Read Line Error")
        }
    }
}

pub type FurbrowserResult<T> = Result<T, FurbrowserError>;