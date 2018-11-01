pub struct ClubhouseDeleteStoryStoryPublicIdCommentCommentPublicId {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseDeleteStoryStoryPublicIdCommentCommentPublicId {
    /// See https://clubhouse.io/api/rest/v2/#Delete-Comment
    pub fn run(self) -> crate::Result<()> {
        self.path.execute_as_json::<(), ()>(None)
    }
}
