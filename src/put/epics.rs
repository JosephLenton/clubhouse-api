mod epic_public_id;
pub use self::epic_public_id::ClubhousePutEpicEpicPublicId;

pub struct ClubhousePutEpic {
    pub(crate) path: burgundy::Path,
}

impl ClubhousePutEpic {
    pub fn epic_public_id(
        self,
        epic_public_id: String,
    ) -> self::epic_public_id::ClubhousePutEpicEpicPublicId {
        self::epic_public_id::ClubhousePutEpicEpicPublicId {
            path: self.path.push(&epic_public_id),
        }
    }
}
