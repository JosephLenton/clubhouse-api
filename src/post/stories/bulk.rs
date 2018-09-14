
pub struct ClubhousePostStoryBulk {
  pub(crate) path: burgundy::Path,
}

impl ClubhousePostStoryBulk {
  /// See https://clubhouse.io/api/rest/v2/#Create-Multiple-Stories
  pub fn run(self) -> burgundy::Result<Vec<crate::types::StorySlim>> {
    self.path
        .execute_as_json::<Vec<crate::types::StorySlim>>()
  }
}
