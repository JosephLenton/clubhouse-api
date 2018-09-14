
pub struct ClubhousePostLinkedFile {
  pub(crate) path: burgundy::Path,
}

impl ClubhousePostLinkedFile {
  /// See https://clubhouse.io/api/rest/v2/#Create-Linked-File
  pub fn run(self) -> burgundy::Result<crate::types::LinkedFile> {
    self.path
        .execute_as_json::<crate::types::LinkedFile>()
  }
}
