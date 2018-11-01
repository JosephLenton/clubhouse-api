pub struct ClubhousePutProjectProjectPublicId {
    pub(crate) path: burgundy::Path,
}

impl ClubhousePutProjectProjectPublicId {
    /// See https://clubhouse.io/api/rest/v2/#Update-Project
    pub fn run(self, body: crate::types::UpdateProject) -> crate::Result<crate::types::Project> {
        self.path
            .execute_as_json::<crate::types::UpdateProject, crate::types::Project>(Some(&body))
    }
}
