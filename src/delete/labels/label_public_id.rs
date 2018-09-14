
pub struct ClubhouseDeleteLabelLabelPublicId {
  pub(crate) path: burgundy::Path,
}

impl ClubhouseDeleteLabelLabelPublicId {
  /// See https://clubhouse.io/api/rest/v2/#Delete-Label
  pub fn run(self) -> burgundy::Result<()> {
    self.path
        .execute_as_json::<()>()
  }
}
