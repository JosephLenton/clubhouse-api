mod file_public_id;
pub use self::file_public_id::ClubhouseGetFileFilePublicId;

pub struct ClubhouseGetFile {
  pub(crate) path: burgundy::Path,
}

impl ClubhouseGetFile {
  pub fn file_public_id(self, file_public_id : String) -> self::file_public_id::ClubhouseGetFileFilePublicId {
    self::file_public_id::ClubhouseGetFileFilePublicId {
      path: self.path.push(&file_public_id),
    }
  }

  /// See https://clubhouse.io/api/rest/v2/#List-Files
  pub fn run(self) -> burgundy::Result<Vec<crate::types::File>> {
    self.path
        .execute_as_json::<Vec<crate::types::File>>()
  }
}
