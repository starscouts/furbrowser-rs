use clap::Parser;

/// Browse and vote on images and videos from e621
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The configuration profile to use
    #[arg(short, long)]
    pub profile: String,

    /// Additional tags to add to the search query
    #[arg(short, long)]
    pub append: Option<String>
}