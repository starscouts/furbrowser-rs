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
    MissingQuery,
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
            FurbrowserError::SQL(e) => write!(f, "Error running SQL query: {e}"),
            FurbrowserError::Environment(e) => write!(f, "Error with environment variable: {e}"),
            FurbrowserError::IO(e) => write!(f, "Error with I/O: {e}"),
            FurbrowserError::TOML(e) => write!(f, "Error decoding TOML data: {e}"),
            FurbrowserError::HTTP(e) => write!(f, "Error running HTTP request: {e}"),
            FurbrowserError::SyncSQLFetch => write!(f, "Error processing SQL data"),
            FurbrowserError::Readline => write!(f, "Error reading line"),
            FurbrowserError::NoSuchProfile => write!(f, "No such profile defined in the configuration file"),
            FurbrowserError::NoValidHome => write!(f, "Could not find the current user's home directory"),
            FurbrowserError::MissingQuery => write!(f, "Either 'query' or 'queries' needs to be defined in the profile")
        }
    }
}

pub type FurbrowserResult<T> = Result<T, Box<FurbrowserError>>;
