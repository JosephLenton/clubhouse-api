pub struct ClubhouseGetTeamTeamPublicId {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseGetTeamTeamPublicId {
    /// See https://clubhouse.io/api/rest/v2/#Get-Team
    pub fn run(self) -> crate::Result<crate::types::Team> {
        self.path.execute_as_json::<(), crate::types::Team>(None)
    }
}
