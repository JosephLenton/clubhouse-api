
pub struct ClubhouseDeleteMilestoneMilestonePublicId {
  pub(crate) path: burgundy::Path,
}

impl ClubhouseDeleteMilestoneMilestonePublicId {
  /// See https://clubhouse.io/api/rest/v2/#Delete-Milestone
  pub fn run(self) -> burgundy::Result<()> {
    self.path
        .execute_as_json::<()>()
  }
}
