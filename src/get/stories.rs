pub mod story_public_id;
pub use self::story_public_id::ClubhouseGetStoryStoryPublicId;

pub struct ClubhouseGetStory {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseGetStory {
    pub fn story_public_id(
        self,
        story_public_id: u64,
    ) -> self::story_public_id::ClubhouseGetStoryStoryPublicId {
        self::story_public_id::ClubhouseGetStoryStoryPublicId {
            path: self.path.push(&story_public_id),
        }
    }
}
