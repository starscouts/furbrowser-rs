use std::env::VarError;
use std::fmt::{Display, Formatter};
use std::io;

#[derive(Debug)]
pub enum FurbrowserError {
    SQL(rusqlite::Error),
    LocalIO(io::Error),
    TOML(toml::de::Error),
    HTTP(ureq::Error),
    Environment(VarError),
    SyncSQLFetch,
    Readline,
    NoSuchProfile
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

impl From<toml::de::Error> for FurbrowserError {
    fn from(value: toml::de::Error) -> Self {
        Self::TOML(value)
    }
}

impl From<ureq::Error> for FurbrowserError {
    fn from(value: ureq::Error) -> Self {
        Self::HTTP(value)
    }
}

impl From<VarError> for FurbrowserError {
    fn from(value: VarError) -> Self {
        Self::Environment(value)
    }
}

impl Display for FurbrowserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FurbrowserError::SQL(e) => write!(f, "SQL Error: {e}"),
            FurbrowserError::Environment(e) => write!(f, "Environment Error: {e}"),
            FurbrowserError::LocalIO(e) => write!(f, "Local I/O Error: {e}"),
            FurbrowserError::TOML(e) => write!(f, "TOML Error: {e}"),
            FurbrowserError::HTTP(e) => write!(f, "HTTP Error: {e}"),
            FurbrowserError::SyncSQLFetch => write!(f, "SQL Processing Error"),
            FurbrowserError::Readline => write!(f, "Read Line Error"),
            FurbrowserError::NoSuchProfile => write!(f, "No Such Profile")
        }
    }
}

pub type FurbrowserResult<T> = Result<T, FurbrowserError>;