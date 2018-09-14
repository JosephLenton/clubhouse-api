
pub struct ClubhousePutFileFilePublicId {
  pub(crate) path: burgundy::Path,
}

impl ClubhousePutFileFilePublicId {
  /// See https://clubhouse.io/api/rest/v2/#Update-File
  pub fn run(self) -> burgundy::Result<crate::types::File> {
    self.path
        .execute_as_json::<crate::types::File>()
  }
}
