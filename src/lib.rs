use std::env;

pub use crate::models::error;
pub use crate::util::tui::start_tui;

mod core;
mod models;
mod util;

const VERSION: &str = env!("CARGO_PKG_VERSION");
