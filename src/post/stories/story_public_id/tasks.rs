
pub struct ClubhousePostStoryStoryPublicIdTask {
  pub(crate) path: burgundy::Path,
}

impl ClubhousePostStoryStoryPublicIdTask {
  /// See https://clubhouse.io/api/rest/v2/#Create-Task
  pub fn run(self) -> burgundy::Result<crate::types::Task> {
    self.path
        .execute_as_json::<crate::types::Task>()
  }
}
