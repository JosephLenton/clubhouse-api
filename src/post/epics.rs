pub mod epic_public_id;
pub use self::epic_public_id::ClubhousePostEpicEpicPublicId;

pub struct ClubhousePostEpic {
    pub(crate) path: burgundy::Path,
}

impl ClubhousePostEpic {
    pub fn epic_public_id(
        self,
        epic_public_id: u64,
    ) -> self::epic_public_id::ClubhousePostEpicEpicPublicId {
        self::epic_public_id::ClubhousePostEpicEpicPublicId {
            path: self.path.push(&epic_public_id),
        }
    }

    /// See https://clubhouse.io/api/rest/v2/#Create-Epic
    pub fn run(self, body: crate::types::CreateEpic) -> crate::Result<crate::types::Epic> {
        self.path
            .execute_as_json::<crate::types::CreateEpic, crate::types::Epic>(Some(&body))
    }
}
