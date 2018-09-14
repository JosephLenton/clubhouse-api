
pub struct ClubhousePutCategoryCategoryPublicId {
  pub(crate) path: burgundy::Path,
}

impl ClubhousePutCategoryCategoryPublicId {
  /// See https://clubhouse.io/api/rest/v2/#Update-Category
  pub fn run(self) -> burgundy::Result<crate::types::Category> {
    self.path
        .execute_as_json::<crate::types::Category>()
  }
}
