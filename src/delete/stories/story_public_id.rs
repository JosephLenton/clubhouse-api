mod comments;
pub use self::comments::ClubhouseDeleteStoryStoryPublicIdComment;
mod tasks;
pub use self::tasks::ClubhouseDeleteStoryStoryPublicIdTask;

pub struct ClubhouseDeleteStoryStoryPublicId {
  pub(crate) path: burgundy::Path,
}

impl ClubhouseDeleteStoryStoryPublicId {
  pub fn comments(self) -> self::comments::ClubhouseDeleteStoryStoryPublicIdComment {
    self::comments::ClubhouseDeleteStoryStoryPublicIdComment {
      path: self.path.push(&"comments"),
    }
  }

  pub fn tasks(self) -> self::tasks::ClubhouseDeleteStoryStoryPublicIdTask {
    self::tasks::ClubhouseDeleteStoryStoryPublicIdTask {
      path: self.path.push(&"tasks"),
    }
  }

  /// See https://clubhouse.io/api/rest/v2/#Delete-Story
  pub fn run(self) -> burgundy::Result<()> {
    self.path
        .execute_as_json::<()>()
  }
}
