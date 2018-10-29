pub struct ClubhouseDeleteEpicEpicPublicIdCommentCommentPublicId {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseDeleteEpicEpicPublicIdCommentCommentPublicId {
    /// See https://clubhouse.io/api/rest/v2/#Delete-Epic-Comment
    pub fn run(self) -> burgundy::Result<()> {
        self.path.execute_as_json::<(), ()>(None)
    }
}
