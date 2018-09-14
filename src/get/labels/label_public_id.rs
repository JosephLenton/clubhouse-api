
pub struct ClubhouseGetLabelLabelPublicId {
  pub(crate) path: burgundy::Path,
}

impl ClubhouseGetLabelLabelPublicId {
  /// See https://clubhouse.io/api/rest/v2/#Get-Label
  pub fn run(self) -> burgundy::Result<crate::types::Label> {
    self.path
        .execute_as_json::<crate::types::Label>()
  }
}
