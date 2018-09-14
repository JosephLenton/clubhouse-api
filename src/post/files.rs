
pub struct ClubhousePostFile {
  pub(crate) path: burgundy::Path,
}

impl ClubhousePostFile {
  /// See https://clubhouse.io/api/rest/v2/#Upload-Files
  pub fn run(self) -> burgundy::Result<Vec<crate::types::File>> {
    self.path
        .execute_as_json::<Vec<crate::types::File>>()
  }
}
