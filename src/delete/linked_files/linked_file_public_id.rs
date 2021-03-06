pub struct ClubhouseDeleteLinkedFileLinkedFilePublicId {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseDeleteLinkedFileLinkedFilePublicId {
    /// See https://clubhouse.io/api/rest/v2/#Delete-Linked-File
    pub fn run(self) -> crate::Result<()> {
        self.path.execute_as_json::<(), ()>(None)
    }
}
