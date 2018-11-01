pub struct ClubhousePutFileFilePublicId {
    pub(crate) path: burgundy::Path,
}

impl ClubhousePutFileFilePublicId {
    /// See https://clubhouse.io/api/rest/v2/#Update-File
    pub fn run(self, body: crate::types::UpdateFile) -> crate::Result<crate::types::File> {
        self.path
            .execute_as_json::<crate::types::UpdateFile, crate::types::File>(Some(&body))
    }
}
