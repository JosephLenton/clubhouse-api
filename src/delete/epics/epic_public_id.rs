mod comments;
pub use self::comments::ClubhouseDeleteEpicEpicPublicIdComment;

pub struct ClubhouseDeleteEpicEpicPublicId {
  pub(crate) path: burgundy::Path,
}

impl ClubhouseDeleteEpicEpicPublicId {
  pub fn comments(self) -> self::comments::ClubhouseDeleteEpicEpicPublicIdComment {
    self::comments::ClubhouseDeleteEpicEpicPublicIdComment {
      path: self.path.push(&"comments"),
    }
  }

  /// See https://clubhouse.io/api/rest/v2/#Delete-Epic
  pub fn run(self) -> burgundy::Result<()> {
    self.path
        .execute_as_json::<()>()
  }
}
