pub enum ImageVote {
    Up,
    Down
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
            ImageVote::Down => String::from("FALSE")
        }
    }
}

impl ImageVote {
    pub fn reverse(&self) -> Self {
        match self {
            ImageVote::Up => ImageVote::Down,
            ImageVote::Down => ImageVote::Up
        }
    }
}