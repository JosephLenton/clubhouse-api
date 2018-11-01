pub struct ClubhousePostStoryBulk {
    pub(crate) path: burgundy::Path,
}

impl ClubhousePostStoryBulk {
    /// See https://clubhouse.io/api/rest/v2/#Create-Multiple-Stories
    pub fn run(
        self,
        body: crate::types::CreateMultipleStories,
    ) -> crate::Result<Vec<crate::types::StorySlim>> {
        self.path
            .execute_as_json::<crate::types::CreateMultipleStories, Vec<crate::types::StorySlim>>(
                Some(&body),
            )
    }
}
