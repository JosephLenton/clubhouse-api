pub struct ClubhousePutLabelLabelPublicId {
    pub(crate) path: burgundy::Path,
}

impl ClubhousePutLabelLabelPublicId {
    /// See https://clubhouse.io/api/rest/v2/#Update-Label
    pub fn run(self, body: crate::types::UpdateLabel) -> burgundy::Result<crate::types::Label> {
        self.path
            .execute_as_json::<crate::types::UpdateLabel, crate::types::Label>(Some(&body))
    }
}
