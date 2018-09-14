
pub struct ClubhouseDeleteProjectProjectPublicId {
  pub(crate) path: burgundy::Path,
}

impl ClubhouseDeleteProjectProjectPublicId {
  /// See https://clubhouse.io/api/rest/v2/#Delete-Project
  pub fn run(self) -> burgundy::Result<()> {
    self.path
        .execute_as_json::<()>()
  }
}
