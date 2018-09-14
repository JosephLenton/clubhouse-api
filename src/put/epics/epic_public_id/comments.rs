mod comment_public_id;
pub use self::comment_public_id::ClubhousePutEpicEpicPublicIdCommentCommentPublicId;

pub struct ClubhousePutEpicEpicPublicIdComment {
  pub(crate) path: burgundy::Path,
}

impl ClubhousePutEpicEpicPublicIdComment {
  pub fn comment_public_id(self, comment_public_id : String) -> self::comment_public_id::ClubhousePutEpicEpicPublicIdCommentCommentPublicId {
    self::comment_public_id::ClubhousePutEpicEpicPublicIdCommentCommentPublicId {
      path: self.path.push(&comment_public_id),
    }
  }
}
