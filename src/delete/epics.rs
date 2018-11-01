pub mod epic_public_id;
pub use self::epic_public_id::ClubhouseDeleteEpicEpicPublicId;

pub struct ClubhouseDeleteEpic {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseDeleteEpic {
    pub fn epic_public_id(
        self,
        epic_public_id: u64,
    ) -> self::epic_public_id::ClubhouseDeleteEpicEpicPublicId {
        self::epic_public_id::ClubhouseDeleteEpicEpicPublicId {
            path: self.path.push(&epic_public_id),
        }
    }
}
