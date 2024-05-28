#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecommendationType {
    Hashtag = 0,
    Url = 1,
    MetadataSize = 2,
    Tweet = 3,
    User = 4,
    Moment = 5,
}

impl RecommendationType {
    pub fn value(self) -> u32 {
        self as u32
    }

    pub fn at(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::Hashtag),
            1 => Some(Self::Url),
            2 => Some(Self::MetadataSize),
            3 => Some(Self::Tweet),
            4 => Some(Self::User),
            5 => Some(Self::Moment),
            _ => None,
        }
    }
}
