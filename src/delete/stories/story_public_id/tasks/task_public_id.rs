pub struct ClubhouseDeleteStoryStoryPublicIdTaskTaskPublicId {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseDeleteStoryStoryPublicIdTaskTaskPublicId {
    /// See https://clubhouse.io/api/rest/v2/#Delete-Task
    pub fn run(self) -> crate::Result<()> {
        self.path.execute_as_json::<(), ()>(None)
    }
}
