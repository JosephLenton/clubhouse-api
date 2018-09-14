mod linked_file_public_id;
pub use self::linked_file_public_id::ClubhouseGetLinkedFileLinkedFilePublicId;

pub struct ClubhouseGetLinkedFile {
  pub(crate) path: burgundy::Path,
}

impl ClubhouseGetLinkedFile {
  pub fn linked_file_public_id(self, linked_file_public_id : String) -> self::linked_file_public_id::ClubhouseGetLinkedFileLinkedFilePublicId {
    self::linked_file_public_id::ClubhouseGetLinkedFileLinkedFilePublicId {
      path: self.path.push(&linked_file_public_id),
    }
  }

  /// See https://clubhouse.io/api/rest/v2/#List-Linked-Files
  pub fn run(self) -> burgundy::Result<Vec<crate::types::LinkedFile>> {
    self.path
        .execute_as_json::<Vec<crate::types::LinkedFile>>()
  }
}
