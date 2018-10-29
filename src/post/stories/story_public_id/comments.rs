pub struct ClubhousePostStoryStoryPublicIdComment {
    pub(crate) path: burgundy::Path,
}

impl ClubhousePostStoryStoryPublicIdComment {
    /// See https://clubhouse.io/api/rest/v2/#Create-Comment
    pub fn run(self, body: crate::types::CreateComment) -> burgundy::Result<crate::types::Comment> {
        self.path
            .execute_as_json::<crate::types::CreateComment, crate::types::Comment>(Some(&body))
    }
}
