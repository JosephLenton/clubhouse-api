pub mod label_public_id;
pub use self::label_public_id::ClubhousePutLabelLabelPublicId;

pub struct ClubhousePutLabel {
    pub(crate) path: burgundy::Path,
}

impl ClubhousePutLabel {
    pub fn label_public_id(
        self,
        label_public_id: u64,
    ) -> self::label_public_id::ClubhousePutLabelLabelPublicId {
        self::label_public_id::ClubhousePutLabelLabelPublicId {
            path: self.path.push(&label_public_id),
        }
    }
}
