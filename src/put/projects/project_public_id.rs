
pub struct ClubhousePutProjectProjectPublicId {
  pub(crate) path: burgundy::Path,
}

impl ClubhousePutProjectProjectPublicId {
  /// See https://clubhouse.io/api/rest/v2/#Update-Project
  pub fn run(self) -> burgundy::Result<crate::types::Project> {
    self.path
        .execute_as_json::<crate::types::Project>()
  }
}
