pub struct ClubhousePutStoryStoryPublicIdTaskTaskPublicId {
    pub(crate) path: burgundy::Path,
}

impl ClubhousePutStoryStoryPublicIdTaskTaskPublicId {
    /// See https://clubhouse.io/api/rest/v2/#Update-Task
    pub fn run(self, body: crate::types::UpdateTask) -> burgundy::Result<crate::types::Task> {
        self.path
            .execute_as_json::<crate::types::UpdateTask, crate::types::Task>(Some(&body))
    }
}
