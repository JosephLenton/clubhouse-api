
pub struct ClubhousePostStoryStoryPublicIdComment {
  pub(crate) path: burgundy::Path,
}

impl ClubhousePostStoryStoryPublicIdComment {
  /// See https://clubhouse.io/api/rest/v2/#Create-Comment
  pub fn run(self) -> burgundy::Result<crate::types::Comment> {
    self.path
        .execute_as_json::<crate::types::Comment>()
  }
}
