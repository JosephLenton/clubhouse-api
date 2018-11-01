pub struct ClubhouseGetCategoryCategoryPublicId {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseGetCategoryCategoryPublicId {
    /// See https://clubhouse.io/api/rest/v2/#Get-Category
    pub fn run(self) -> crate::Result<crate::types::Category> {
        self.path
            .execute_as_json::<(), crate::types::Category>(None)
    }
}
