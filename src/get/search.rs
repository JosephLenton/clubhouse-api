mod stories;
pub use self::stories::ClubhouseGetSearchStory;

pub struct ClubhouseGetSearch {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseGetSearch {
    pub fn stories(self) -> self::stories::ClubhouseGetSearchStory {
        self::stories::ClubhouseGetSearchStory {
            path: self.path.push(&"stories"),
        }
    }
}
