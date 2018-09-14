mod label_public_id;
pub use self::label_public_id::ClubhouseGetLabelLabelPublicId;

pub struct ClubhouseGetLabel {
  pub(crate) path: burgundy::Path,
}

impl ClubhouseGetLabel {
  pub fn label_public_id(self, label_public_id : String) -> self::label_public_id::ClubhouseGetLabelLabelPublicId {
    self::label_public_id::ClubhouseGetLabelLabelPublicId {
      path: self.path.push(&label_public_id),
    }
  }

  /// See https://clubhouse.io/api/rest/v2/#List-Labels
  pub fn run(self) -> burgundy::Result<Vec<crate::types::Label>> {
    self.path
        .execute_as_json::<Vec<crate::types::Label>>()
  }
}
