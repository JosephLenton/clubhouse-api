mod comment_public_id;
pub use self::comment_public_id::ClubhouseDeleteStoryStoryPublicIdCommentCommentPublicId;

pub struct ClubhouseDeleteStoryStoryPublicIdComment {
  pub(crate) path: burgundy::Path,
}

impl ClubhouseDeleteStoryStoryPublicIdComment {
  pub fn comment_public_id(self, comment_public_id : String) -> self::comment_public_id::ClubhouseDeleteStoryStoryPublicIdCommentCommentPublicId {
    self::comment_public_id::ClubhouseDeleteStoryStoryPublicIdCommentCommentPublicId {
      path: self.path.push(&comment_public_id),
    }
  }
}
