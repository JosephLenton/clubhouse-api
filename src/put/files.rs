mod file_public_id;
pub use self::file_public_id::ClubhousePutFileFilePublicId;

pub struct ClubhousePutFile {
  pub(crate) path: burgundy::Path,
}

impl ClubhousePutFile {
  pub fn file_public_id(self, file_public_id : String) -> self::file_public_id::ClubhousePutFileFilePublicId {
    self::file_public_id::ClubhousePutFileFilePublicId {
      path: self.path.push(&file_public_id),
    }
  }
}
