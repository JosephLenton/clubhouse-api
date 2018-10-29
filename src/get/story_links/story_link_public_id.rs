pub struct ClubhouseGetStoryLinkStoryLinkPublicId {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseGetStoryLinkStoryLinkPublicId {
    /// See https://clubhouse.io/api/rest/v2/#Get-Story-Link
    pub fn run(self) -> burgundy::Result<crate::types::StoryLink> {
        self.path
            .execute_as_json::<(), crate::types::StoryLink>(None)
    }
}
