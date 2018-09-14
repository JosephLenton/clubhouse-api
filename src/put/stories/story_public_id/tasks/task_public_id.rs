
pub struct ClubhousePutStoryStoryPublicIdTaskTaskPublicId {
  pub(crate) path: burgundy::Path,
}

impl ClubhousePutStoryStoryPublicIdTaskTaskPublicId {
  /// See https://clubhouse.io/api/rest/v2/#Update-Task
  pub fn run(self) -> burgundy::Result<crate::types::Task> {
    self.path
        .execute_as_json::<crate::types::Task>()
  }
}
