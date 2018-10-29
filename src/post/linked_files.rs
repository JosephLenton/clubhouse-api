pub struct ClubhousePostLinkedFile {
    pub(crate) path: burgundy::Path,
}

impl ClubhousePostLinkedFile {
    /// See https://clubhouse.io/api/rest/v2/#Create-Linked-File
    pub fn run(
        self,
        body: crate::types::CreateLinkedFile,
    ) -> burgundy::Result<crate::types::LinkedFile> {
        self.path
            .execute_as_json::<crate::types::CreateLinkedFile, crate::types::LinkedFile>(Some(
                &body,
            ))
    }
}
