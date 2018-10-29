mod tasks;
pub use self::tasks::ClubhousePostStoryStoryPublicIdTask;
mod comments;
pub use self::comments::ClubhousePostStoryStoryPublicIdComment;

pub struct ClubhousePostStoryStoryPublicId {
    pub(crate) path: burgundy::Path,
}

impl ClubhousePostStoryStoryPublicId {
    pub fn tasks(self) -> self::tasks::ClubhousePostStoryStoryPublicIdTask {
        self::tasks::ClubhousePostStoryStoryPublicIdTask {
            path: self.path.push(&"tasks"),
        }
    }

    pub fn comments(self) -> self::comments::ClubhousePostStoryStoryPublicIdComment {
        self::comments::ClubhousePostStoryStoryPublicIdComment {
            path: self.path.push(&"comments"),
        }
    }
}
