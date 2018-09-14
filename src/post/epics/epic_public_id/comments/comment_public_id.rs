
pub struct ClubhousePostEpicEpicPublicIdCommentCommentPublicId {
  pub(crate) path: burgundy::Path,
}

impl ClubhousePostEpicEpicPublicIdCommentCommentPublicId {
  /// See https://clubhouse.io/api/rest/v2/#Create-Epic-Comment-Comment
  pub fn run(self) -> burgundy::Result<crate::types::ThreadedComment> {
    self.path
        .execute_as_json::<crate::types::ThreadedComment>()
  }
}
