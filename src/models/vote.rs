use std::fmt::{Display, Formatter};

pub enum ImageVote {
    Up,
    Down,
}

pub struct Score {
    pub(crate) upvotes: i64,
    pub(crate) downvotes: i64,
}

impl From<bool> for ImageVote {
    fn from(value: bool) -> Self {
        if value {
            Self::Up
        } else {
            Self::Down
        }
    }
}

impl From<&ImageVote> for String {
    fn from(value: &ImageVote) -> Self {
        match value {
            ImageVote::Up => String::from("TRUE"),
            ImageVote::Down => String::from("FALSE"),
        }
    }
}

impl From<&ImageVote> for i32 {
    fn from(value: &ImageVote) -> Self {
        match value {
            ImageVote::Up => 1,
            ImageVote::Down => -1,
        }
    }
}

impl Display for ImageVote {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ImageVote::Up => "1",
                ImageVote::Down => "-1",
            }
        )
    }
}

impl ImageVote {
    pub fn reverse(&self) -> Self {
        match self {
            ImageVote::Up => ImageVote::Down,
            ImageVote::Down => ImageVote::Up,
        }
    }
}
