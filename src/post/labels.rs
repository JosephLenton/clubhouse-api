
pub struct ClubhousePostLabel {
  pub(crate) path: burgundy::Path,
}

impl ClubhousePostLabel {
  /// See https://clubhouse.io/api/rest/v2/#Create-Label
  pub fn run(self) -> burgundy::Result<crate::types::Label> {
    self.path
        .execute_as_json::<crate::types::Label>()
  }
}
