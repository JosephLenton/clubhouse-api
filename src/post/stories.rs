pub mod bulk;
pub use self::bulk::ClubhousePostStoryBulk;
pub mod story_public_id;
pub use self::story_public_id::ClubhousePostStoryStoryPublicId;

pub struct ClubhousePostStory {
    pub(crate) path: burgundy::Path,
}

impl ClubhousePostStory {
    pub fn bulk(self) -> self::bulk::ClubhousePostStoryBulk {
        self::bulk::ClubhousePostStoryBulk {
            path: self.path.push(&"bulk"),
        }
    }

    pub fn story_public_id(
        self,
        story_public_id: u64,
    ) -> self::story_public_id::ClubhousePostStoryStoryPublicId {
        self::story_public_id::ClubhousePostStoryStoryPublicId {
            path: self.path.push(&story_public_id),
        }
    }

    /// See https://clubhouse.io/api/rest/v2/#Create-Story
    pub fn run(self, body: crate::types::CreateStory) -> crate::Result<crate::types::Story> {
        self.path
            .execute_as_json::<crate::types::CreateStory, crate::types::Story>(Some(&body))
    }
}
