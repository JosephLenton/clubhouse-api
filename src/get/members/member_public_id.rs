pub struct ClubhouseGetMemberMemberPublicId {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseGetMemberMemberPublicId {
    /// See https://clubhouse.io/api/rest/v2/#Get-Member
    pub fn run(self) -> crate::Result<crate::types::Member> {
        self.path.execute_as_json::<(), crate::types::Member>(None)
    }
}
