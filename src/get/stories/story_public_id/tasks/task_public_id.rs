
pub struct ClubhouseGetStoryStoryPublicIdTaskTaskPublicId {
  pub(crate) path: burgundy::Path,
}

impl ClubhouseGetStoryStoryPublicIdTaskTaskPublicId {
  /// See https://clubhouse.io/api/rest/v2/#Get-Task
  pub fn run(self) -> burgundy::Result<crate::types::Task> {
    self.path
        .execute_as_json::<crate::types::Task>()
  }
}
