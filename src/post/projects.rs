
pub struct ClubhousePostProject {
  pub(crate) path: burgundy::Path,
}

impl ClubhousePostProject {
  /// See https://clubhouse.io/api/rest/v2/#Create-Project
  pub fn run(self) -> burgundy::Result<crate::types::Project> {
    self.path
        .execute_as_json::<crate::types::Project>()
  }
}
