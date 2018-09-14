mod epic_public_id;
pub use self::epic_public_id::ClubhouseGetEpicEpicPublicId;

pub struct ClubhouseGetEpic {
  pub(crate) path: burgundy::Path,
}

impl ClubhouseGetEpic {
  pub fn epic_public_id(self, epic_public_id : String) -> self::epic_public_id::ClubhouseGetEpicEpicPublicId {
    self::epic_public_id::ClubhouseGetEpicEpicPublicId {
      path: self.path.push(&epic_public_id),
    }
  }

  /// See https://clubhouse.io/api/rest/v2/#List-Epics
  pub fn run(self) -> burgundy::Result<Vec<crate::types::Epic>> {
    self.path
        .execute_as_json::<Vec<crate::types::Epic>>()
  }
}
