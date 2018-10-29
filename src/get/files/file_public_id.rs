pub struct ClubhouseGetFileFilePublicId {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseGetFileFilePublicId {
    /// See https://clubhouse.io/api/rest/v2/#Get-File
    pub fn run(self) -> burgundy::Result<crate::types::File> {
        self.path.execute_as_json::<(), crate::types::File>(None)
    }
}
