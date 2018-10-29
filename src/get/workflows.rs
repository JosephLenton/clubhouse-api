pub struct ClubhouseGetWorkflow {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseGetWorkflow {
    /// See https://clubhouse.io/api/rest/v2/#List-Workflows
    pub fn run(self) -> burgundy::Result<Vec<crate::types::Workflow>> {
        self.path
            .execute_as_json::<(), Vec<crate::types::Workflow>>(None)
    }
}
