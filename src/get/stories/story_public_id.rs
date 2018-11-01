pub mod tasks;
pub use self::tasks::ClubhouseGetStoryStoryPublicIdTask;
pub mod comments;
pub use self::comments::ClubhouseGetStoryStoryPublicIdComment;

pub struct ClubhouseGetStoryStoryPublicId {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseGetStoryStoryPublicId {
    pub fn tasks(self) -> self::tasks::ClubhouseGetStoryStoryPublicIdTask {
        self::tasks::ClubhouseGetStoryStoryPublicIdTask {
            path: self.path.push(&"tasks"),
        }
    }

    pub fn comments(self) -> self::comments::ClubhouseGetStoryStoryPublicIdComment {
        self::comments::ClubhouseGetStoryStoryPublicIdComment {
            path: self.path.push(&"comments"),
        }
    }

    /// See https://clubhouse.io/api/rest/v2/#Get-Story
    pub fn run(self) -> crate::Result<crate::types::Story> {
        self.path.execute_as_json::<(), crate::types::Story>(None)
    }
}
