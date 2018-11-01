pub mod member_public_id;
pub use self::member_public_id::ClubhouseGetMemberMemberPublicId;

pub struct ClubhouseGetMember {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseGetMember {
    pub fn member_public_id(
        self,
        member_public_id: uuid::Uuid,
    ) -> self::member_public_id::ClubhouseGetMemberMemberPublicId {
        self::member_public_id::ClubhouseGetMemberMemberPublicId {
            path: self.path.push(&member_public_id),
        }
    }

    /// See https://clubhouse.io/api/rest/v2/#List-Members
    pub fn run(self) -> crate::Result<Vec<crate::types::Member>> {
        self.path
            .execute_as_json::<(), Vec<crate::types::Member>>(None)
    }
}
