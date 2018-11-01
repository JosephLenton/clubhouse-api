pub struct ClubhousePostProject {
    pub(crate) path: burgundy::Path,
}

impl ClubhousePostProject {
    /// See https://clubhouse.io/api/rest/v2/#Create-Project
    pub fn run(self, body: crate::types::CreateProject) -> crate::Result<crate::types::Project> {
        self.path
            .execute_as_json::<crate::types::CreateProject, crate::types::Project>(Some(&body))
    }
}
