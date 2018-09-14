
pub struct ClubhousePutStoryBulk {
  pub(crate) path: burgundy::Path,
}

impl ClubhousePutStoryBulk {
  /// See https://clubhouse.io/api/rest/v2/#Update-Multiple-Stories
  pub fn run(self) -> burgundy::Result<Vec<crate::types::StorySlim>> {
    self.path
        .execute_as_json::<Vec<crate::types::StorySlim>>()
  }
}
