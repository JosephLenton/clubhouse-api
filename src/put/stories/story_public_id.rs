pub mod tasks;
pub use self::tasks::ClubhousePutStoryStoryPublicIdTask;
pub mod comments;
pub use self::comments::ClubhousePutStoryStoryPublicIdComment;

pub struct ClubhousePutStoryStoryPublicId {
    pub(crate) path: burgundy::Path,
}

impl ClubhousePutStoryStoryPublicId {
    pub fn tasks(self) -> self::tasks::ClubhousePutStoryStoryPublicIdTask {
        self::tasks::ClubhousePutStoryStoryPublicIdTask {
            path: self.path.push(&"tasks"),
        }
    }

    pub fn comments(self) -> self::comments::ClubhousePutStoryStoryPublicIdComment {
        self::comments::ClubhousePutStoryStoryPublicIdComment {
            path: self.path.push(&"comments"),
        }
    }

    /// See https://clubhouse.io/api/rest/v2/#Update-Story
    pub fn run(self, body: crate::types::UpdateStory) -> crate::Result<crate::types::Story> {
        self.path
            .execute_as_json::<crate::types::UpdateStory, crate::types::Story>(Some(&body))
    }
}
