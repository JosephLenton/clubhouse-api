pub struct ClubhouseDeleteCategoryCategoryPublicId {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseDeleteCategoryCategoryPublicId {
    /// See https://clubhouse.io/api/rest/v2/#Delete-Category
    pub fn run(self) -> burgundy::Result<()> {
        self.path.execute_as_json::<(), ()>(None)
    }
}
