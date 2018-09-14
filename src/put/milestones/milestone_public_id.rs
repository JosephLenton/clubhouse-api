
pub struct ClubhousePutMilestoneMilestonePublicId {
  pub(crate) path: burgundy::Path,
}

impl ClubhousePutMilestoneMilestonePublicId {
  /// See https://clubhouse.io/api/rest/v2/#Update-Milestone
  pub fn run(self) -> burgundy::Result<crate::types::Milestone> {
    self.path
        .execute_as_json::<crate::types::Milestone>()
  }
}
