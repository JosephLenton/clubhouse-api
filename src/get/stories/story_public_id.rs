mod comments;
pub use self::comments::ClubhouseGetStoryStoryPublicIdComment;
mod tasks;
pub use self::tasks::ClubhouseGetStoryStoryPublicIdTask;

pub struct ClubhouseGetStoryStoryPublicId {
  pub(crate) path: burgundy::Path,
}

impl ClubhouseGetStoryStoryPublicId {
  pub fn comments(self) -> self::comments::ClubhouseGetStoryStoryPublicIdComment {
    self::comments::ClubhouseGetStoryStoryPublicIdComment {
      path: self.path.push(&"comments"),
    }
  }

  pub fn tasks(self) -> self::tasks::ClubhouseGetStoryStoryPublicIdTask {
    self::tasks::ClubhouseGetStoryStoryPublicIdTask {
      path: self.path.push(&"tasks"),
    }
  }

  /// See https://clubhouse.io/api/rest/v2/#Get-Story
  pub fn run(self) -> burgundy::Result<crate::types::Story> {
    self.path
        .execute_as_json::<crate::types::Story>()
  }
}
