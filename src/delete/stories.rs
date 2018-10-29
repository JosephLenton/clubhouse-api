mod story_public_id;
pub use self::story_public_id::ClubhouseDeleteStoryStoryPublicId;
mod bulk;
pub use self::bulk::ClubhouseDeleteStoryBulk;

pub struct ClubhouseDeleteStory {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseDeleteStory {
    pub fn story_public_id(
        self,
        story_public_id: String,
    ) -> self::story_public_id::ClubhouseDeleteStoryStoryPublicId {
        self::story_public_id::ClubhouseDeleteStoryStoryPublicId {
            path: self.path.push(&story_public_id),
        }
    }

    pub fn bulk(self) -> self::bulk::ClubhouseDeleteStoryBulk {
        self::bulk::ClubhouseDeleteStoryBulk {
            path: self.path.push(&"bulk"),
        }
    }
}
