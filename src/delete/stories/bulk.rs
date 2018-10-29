pub struct ClubhouseDeleteStoryBulk {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseDeleteStoryBulk {
    /// See https://clubhouse.io/api/rest/v2/#Delete-Multiple-Stories
    pub fn run(self, body: crate::types::DeleteMultipleStories) -> burgundy::Result<()> {
        self.path
            .execute_as_json::<crate::types::DeleteMultipleStories, ()>(Some(&body))
    }
}
