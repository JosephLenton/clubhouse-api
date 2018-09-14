
pub struct ClubhousePostStoryLink {
  pub(crate) path: burgundy::Path,
}

impl ClubhousePostStoryLink {
  /// See https://clubhouse.io/api/rest/v2/#Create-Story-Link
  pub fn run(self) -> burgundy::Result<crate::types::StoryLink> {
    self.path
        .execute_as_json::<crate::types::StoryLink>()
  }
}
