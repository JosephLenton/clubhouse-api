pub struct ClubhouseDeleteStoryLinkStoryLinkPublicId {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseDeleteStoryLinkStoryLinkPublicId {
    /// See https://clubhouse.io/api/rest/v2/#Delete-Story-Link
    pub fn run(self) -> crate::Result<()> {
        self.path.execute_as_json::<(), ()>(None)
    }
}
