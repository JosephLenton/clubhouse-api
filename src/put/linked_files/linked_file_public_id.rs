
pub struct ClubhousePutLinkedFileLinkedFilePublicId {
  pub(crate) path: burgundy::Path,
}

impl ClubhousePutLinkedFileLinkedFilePublicId {
  /// See https://clubhouse.io/api/rest/v2/#Update-Linked-File
  pub fn run(self) -> burgundy::Result<crate::types::LinkedFile> {
    self.path
        .execute_as_json::<crate::types::LinkedFile>()
  }
}
