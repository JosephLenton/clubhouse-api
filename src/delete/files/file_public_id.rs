pub struct ClubhouseDeleteFileFilePublicId {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseDeleteFileFilePublicId {
    /// See https://clubhouse.io/api/rest/v2/#Delete-File
    pub fn run(self) -> burgundy::Result<()> {
        self.path.execute_as_json::<(), ()>(None)
    }
}
