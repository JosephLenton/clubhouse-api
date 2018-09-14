
pub struct ClubhouseDeleteStoryBulk {
  pub(crate) path: burgundy::Path,
}

impl ClubhouseDeleteStoryBulk {
  /// See https://clubhouse.io/api/rest/v2/#Delete-Multiple-Stories
  pub fn run(self) -> burgundy::Result<()> {
    self.path
        .execute_as_json::<()>()
  }
}
