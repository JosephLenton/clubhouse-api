pub struct ClubhousePostMilestone {
    pub(crate) path: burgundy::Path,
}

impl ClubhousePostMilestone {
    /// See https://clubhouse.io/api/rest/v2/#Create-Milestone
    pub fn run(
        self,
        body: crate::types::CreateMilestone,
    ) -> crate::Result<crate::types::Milestone> {
        self.path
            .execute_as_json::<crate::types::CreateMilestone, crate::types::Milestone>(Some(&body))
    }
}
