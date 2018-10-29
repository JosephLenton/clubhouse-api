mod linked_file_public_id;
pub use self::linked_file_public_id::ClubhouseDeleteLinkedFileLinkedFilePublicId;

pub struct ClubhouseDeleteLinkedFile {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseDeleteLinkedFile {
    pub fn linked_file_public_id(
        self,
        linked_file_public_id: String,
    ) -> self::linked_file_public_id::ClubhouseDeleteLinkedFileLinkedFilePublicId {
        self::linked_file_public_id::ClubhouseDeleteLinkedFileLinkedFilePublicId {
            path: self.path.push(&linked_file_public_id),
        }
    }
}
