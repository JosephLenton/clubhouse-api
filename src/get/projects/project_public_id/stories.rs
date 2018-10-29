pub struct ClubhouseGetProjectProjectPublicIdStory {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseGetProjectProjectPublicIdStory {
    /// See https://clubhouse.io/api/rest/v2/#List-Stories
    pub fn run(self) -> burgundy::Result<Vec<crate::types::StorySlim>> {
        self.path
            .execute_as_json::<(), Vec<crate::types::StorySlim>>(None)
    }
}
