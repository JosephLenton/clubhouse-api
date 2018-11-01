pub struct ClubhousePutStoryStoryPublicIdCommentCommentPublicId {
    pub(crate) path: burgundy::Path,
}

impl ClubhousePutStoryStoryPublicIdCommentCommentPublicId {
    /// See https://clubhouse.io/api/rest/v2/#Update-Comment
    pub fn run(self, body: crate::types::UpdateComment) -> crate::Result<crate::types::Comment> {
        self.path
            .execute_as_json::<crate::types::UpdateComment, crate::types::Comment>(Some(&body))
    }
}
