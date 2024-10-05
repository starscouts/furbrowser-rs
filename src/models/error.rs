use std::env::VarError;
use std::fmt::{Display, Formatter};
use std::io;

#[derive(Debug)]
pub enum FurbrowserError {
    SQL(rusqlite::Error),
    IO(io::Error),
    TOML(toml::de::Error),
    HTTP(ureq::Error),
    Environment(VarError),
    SyncSQLFetch,
    Readline,
    NoSuchProfile,
    NoValidHome,
}

impl From<rusqlite::Error> for Box<FurbrowserError> {
    fn from(value: rusqlite::Error) -> Self {
        Self::new(FurbrowserError::SQL(value))
    }
}

impl From<io::Error> for Box<FurbrowserError> {
    fn from(value: io::Error) -> Self {
        Self::new(FurbrowserError::IO(value))
    }
}

impl From<toml::de::Error> for Box<FurbrowserError> {
    fn from(value: toml::de::Error) -> Self {
        Self::new(FurbrowserError::TOML(value))
    }
}

impl From<ureq::Error> for Box<FurbrowserError> {
    fn from(value: ureq::Error) -> Self {
        Self::new(FurbrowserError::HTTP(value))
    }
}

impl From<VarError> for Box<FurbrowserError> {
    fn from(value: VarError) -> Self {
        Self::new(FurbrowserError::Environment(value))
    }
}

impl Display for FurbrowserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FurbrowserError::SQL(e) => write!(f, "SQL Error: {e}"),
            FurbrowserError::Environment(e) => write!(f, "Environment Error: {e}"),
            FurbrowserError::IO(e) => write!(f, "Local I/O Error: {e}"),
            FurbrowserError::TOML(e) => write!(f, "TOML Error: {e}"),
            FurbrowserError::HTTP(e) => write!(f, "HTTP Error: {e}"),
            FurbrowserError::SyncSQLFetch => write!(f, "SQL Processing Error"),
            FurbrowserError::Readline => write!(f, "Read Line Error"),
            FurbrowserError::NoSuchProfile => write!(f, "No Such Profile"),
            FurbrowserError::NoValidHome => write!(f, "No Home Directory"),
        }
    }
}

pub type FurbrowserResult<T> = Result<T, Box<FurbrowserError>>;
