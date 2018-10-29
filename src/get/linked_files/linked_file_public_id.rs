pub struct ClubhouseGetLinkedFileLinkedFilePublicId {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseGetLinkedFileLinkedFilePublicId {
    /// See https://clubhouse.io/api/rest/v2/#Get-Linked-File
    pub fn run(self) -> burgundy::Result<crate::types::LinkedFile> {
        self.path
            .execute_as_json::<(), crate::types::LinkedFile>(None)
    }
}
