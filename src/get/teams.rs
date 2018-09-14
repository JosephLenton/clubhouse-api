mod team_public_id;
pub use self::team_public_id::ClubhouseGetTeamTeamPublicId;

pub struct ClubhouseGetTeam {
  pub(crate) path: burgundy::Path,
}

impl ClubhouseGetTeam {
  pub fn team_public_id(self, team_public_id : String) -> self::team_public_id::ClubhouseGetTeamTeamPublicId {
    self::team_public_id::ClubhouseGetTeamTeamPublicId {
      path: self.path.push(&team_public_id),
    }
  }

  /// See https://clubhouse.io/api/rest/v2/#List-Teams
  pub fn run(self) -> burgundy::Result<Vec<crate::types::Team>> {
    self.path
        .execute_as_json::<Vec<crate::types::Team>>()
  }
}
