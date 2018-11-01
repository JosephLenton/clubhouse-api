pub mod linked_file_public_id;
pub use self::linked_file_public_id::ClubhousePutLinkedFileLinkedFilePublicId;

pub struct ClubhousePutLinkedFile {
    pub(crate) path: burgundy::Path,
}

impl ClubhousePutLinkedFile {
    pub fn linked_file_public_id(
        self,
        linked_file_public_id: u64,
    ) -> self::linked_file_public_id::ClubhousePutLinkedFileLinkedFilePublicId {
        self::linked_file_public_id::ClubhousePutLinkedFileLinkedFilePublicId {
            path: self.path.push(&linked_file_public_id),
        }
    }
}
