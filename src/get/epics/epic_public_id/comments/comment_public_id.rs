
pub struct ClubhouseGetEpicEpicPublicIdCommentCommentPublicId {
  pub(crate) path: burgundy::Path,
}

impl ClubhouseGetEpicEpicPublicIdCommentCommentPublicId {
  /// See https://clubhouse.io/api/rest/v2/#Get-Epic-Comment
  pub fn run(self) -> burgundy::Result<crate::types::ThreadedComment> {
    self.path
        .execute_as_json::<crate::types::ThreadedComment>()
  }
}
