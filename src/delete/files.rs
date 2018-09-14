mod file_public_id;
pub use self::file_public_id::ClubhouseDeleteFileFilePublicId;

pub struct ClubhouseDeleteFile {
  pub(crate) path: burgundy::Path,
}

impl ClubhouseDeleteFile {
  pub fn file_public_id(self, file_public_id : String) -> self::file_public_id::ClubhouseDeleteFileFilePublicId {
    self::file_public_id::ClubhouseDeleteFileFilePublicId {
      path: self.path.push(&file_public_id),
    }
  }
}
