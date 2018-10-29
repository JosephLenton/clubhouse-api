pub struct ClubhousePutEpicEpicPublicIdCommentCommentPublicId {
    pub(crate) path: burgundy::Path,
}

impl ClubhousePutEpicEpicPublicIdCommentCommentPublicId {
    /// See https://clubhouse.io/api/rest/v2/#Update-Epic-Comment
    pub fn run(
        self,
        body: crate::types::UpdateEpicComment,
    ) -> burgundy::Result<crate::types::ThreadedComment> {
        self.path
            .execute_as_json::<crate::types::UpdateEpicComment, crate::types::ThreadedComment>(
                Some(&body),
            )
    }
}
