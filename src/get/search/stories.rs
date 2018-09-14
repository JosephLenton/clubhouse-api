
pub struct ClubhouseGetSearchStory {
  pub(crate) path: burgundy::Path,
}

impl ClubhouseGetSearchStory {
  /// See https://clubhouse.io/api/rest/v2/#Search-Stories
  pub fn run(self) -> burgundy::Result<crate::types::SearchResults> {
    self.path
        .execute_as_json::<crate::types::SearchResults>()
  }
}
