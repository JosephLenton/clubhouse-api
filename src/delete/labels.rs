mod label_public_id;
pub use self::label_public_id::ClubhouseDeleteLabelLabelPublicId;

pub struct ClubhouseDeleteLabel {
  pub(crate) path: burgundy::Path,
}

impl ClubhouseDeleteLabel {
  pub fn label_public_id(self, label_public_id : String) -> self::label_public_id::ClubhouseDeleteLabelLabelPublicId {
    self::label_public_id::ClubhouseDeleteLabelLabelPublicId {
      path: self.path.push(&label_public_id),
    }
  }
}
