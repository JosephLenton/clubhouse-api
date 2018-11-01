pub struct ClubhouseGetMilestoneMilestonePublicId {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseGetMilestoneMilestonePublicId {
    /// See https://clubhouse.io/api/rest/v2/#Get-Milestone
    pub fn run(self) -> crate::Result<crate::types::Milestone> {
        self.path
            .execute_as_json::<(), crate::types::Milestone>(None)
    }
}
