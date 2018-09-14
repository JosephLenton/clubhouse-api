
pub struct ClubhouseDeleteLinkedFileLinkedFilePublicId {
  pub(crate) path: burgundy::Path,
}

impl ClubhouseDeleteLinkedFileLinkedFilePublicId {
  /// See https://clubhouse.io/api/rest/v2/#Delete-Linked-File
  pub fn run(self) -> burgundy::Result<()> {
    self.path
        .execute_as_json::<()>()
  }
}
