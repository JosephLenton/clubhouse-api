pub struct ClubhousePostEpicEpicPublicIdCommentCommentPublicId {
    pub(crate) path: burgundy::Path,
}

impl ClubhousePostEpicEpicPublicIdCommentCommentPublicId {
    /// See https://clubhouse.io/api/rest/v2/#Create-Epic-Comment-Comment
    pub fn run(
        self,
        body: crate::types::CreateEpicCommentComment,
    ) -> burgundy::Result<crate::types::ThreadedComment> {
        self.path
        .execute_as_json::<crate::types::CreateEpicCommentComment, crate::types::ThreadedComment>(Some(&body))
    }
}
