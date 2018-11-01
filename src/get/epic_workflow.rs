pub struct ClubhouseGetEpicWorkflow {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseGetEpicWorkflow {
    /// See https://clubhouse.io/api/rest/v2/#Get-Epic-Workflow
    pub fn run(self) -> crate::Result<crate::types::EpicWorkflow> {
        self.path
            .execute_as_json::<(), crate::types::EpicWorkflow>(None)
    }
}
