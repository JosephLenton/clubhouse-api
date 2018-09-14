
pub struct ClubhouseGetStoryStoryPublicIdCommentCommentPublicId {
  pub(crate) path: burgundy::Path,
}

impl ClubhouseGetStoryStoryPublicIdCommentCommentPublicId {
  /// See https://clubhouse.io/api/rest/v2/#Get-Comment
  pub fn run(self) -> burgundy::Result<crate::types::Comment> {
    self.path
        .execute_as_json::<crate::types::Comment>()
  }
}
