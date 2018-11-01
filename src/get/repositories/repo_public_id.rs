pub struct ClubhouseGetRepositoryRepoPublicId {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseGetRepositoryRepoPublicId {
    /// See https://clubhouse.io/api/rest/v2/#Get-Repository
    pub fn run(self) -> crate::Result<crate::types::Repository> {
        self.path
            .execute_as_json::<(), crate::types::Repository>(None)
    }
}
