mod comment_public_id;
pub use self::comment_public_id::ClubhousePutStoryStoryPublicIdCommentCommentPublicId;

pub struct ClubhousePutStoryStoryPublicIdComment {
  pub(crate) path: burgundy::Path,
}

impl ClubhousePutStoryStoryPublicIdComment {
  pub fn comment_public_id(self, comment_public_id : String) -> self::comment_public_id::ClubhousePutStoryStoryPublicIdCommentCommentPublicId {
    self::comment_public_id::ClubhousePutStoryStoryPublicIdCommentCommentPublicId {
      path: self.path.push(&comment_public_id),
    }
  }
}
