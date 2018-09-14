
pub struct ClubhousePostCategory {
  pub(crate) path: burgundy::Path,
}

impl ClubhousePostCategory {
  /// See https://clubhouse.io/api/rest/v2/#Create-Category
  pub fn run(self) -> burgundy::Result<crate::types::Category> {
    self.path
        .execute_as_json::<crate::types::Category>()
  }
}
