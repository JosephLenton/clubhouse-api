pub struct ClubhousePutMilestoneMilestonePublicId {
    pub(crate) path: burgundy::Path,
}

impl ClubhousePutMilestoneMilestonePublicId {
    /// See https://clubhouse.io/api/rest/v2/#Update-Milestone
    pub fn run(
        self,
        body: crate::types::UpdateMilestone,
    ) -> crate::Result<crate::types::Milestone> {
        self.path
            .execute_as_json::<crate::types::UpdateMilestone, crate::types::Milestone>(Some(&body))
    }
}
