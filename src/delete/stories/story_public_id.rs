mod tasks;
pub use self::tasks::ClubhouseDeleteStoryStoryPublicIdTask;
mod comments;
pub use self::comments::ClubhouseDeleteStoryStoryPublicIdComment;

pub struct ClubhouseDeleteStoryStoryPublicId {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseDeleteStoryStoryPublicId {
    pub fn tasks(self) -> self::tasks::ClubhouseDeleteStoryStoryPublicIdTask {
        self::tasks::ClubhouseDeleteStoryStoryPublicIdTask {
            path: self.path.push(&"tasks"),
        }
    }

    pub fn comments(self) -> self::comments::ClubhouseDeleteStoryStoryPublicIdComment {
        self::comments::ClubhouseDeleteStoryStoryPublicIdComment {
            path: self.path.push(&"comments"),
        }
    }

    /// See https://clubhouse.io/api/rest/v2/#Delete-Story
    pub fn run(self) -> burgundy::Result<()> {
        self.path.execute_as_json::<(), ()>(None)
    }
}
