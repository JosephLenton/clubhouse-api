pub mod story_public_id;
pub use self::story_public_id::ClubhousePutStoryStoryPublicId;
pub mod bulk;
pub use self::bulk::ClubhousePutStoryBulk;

pub struct ClubhousePutStory {
    pub(crate) path: burgundy::Path,
}

impl ClubhousePutStory {
    pub fn story_public_id(
        self,
        story_public_id: u64,
    ) -> self::story_public_id::ClubhousePutStoryStoryPublicId {
        self::story_public_id::ClubhousePutStoryStoryPublicId {
            path: self.path.push(&story_public_id),
        }
    }

    pub fn bulk(self) -> self::bulk::ClubhousePutStoryBulk {
        self::bulk::ClubhousePutStoryBulk {
            path: self.path.push(&"bulk"),
        }
    }
}
