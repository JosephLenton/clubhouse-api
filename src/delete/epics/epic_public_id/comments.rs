mod comment_public_id;
pub use self::comment_public_id::ClubhouseDeleteEpicEpicPublicIdCommentCommentPublicId;

pub struct ClubhouseDeleteEpicEpicPublicIdComment {
  pub(crate) path: burgundy::Path,
}

impl ClubhouseDeleteEpicEpicPublicIdComment {
  pub fn comment_public_id(self, comment_public_id : String) -> self::comment_public_id::ClubhouseDeleteEpicEpicPublicIdCommentCommentPublicId {
    self::comment_public_id::ClubhouseDeleteEpicEpicPublicIdCommentCommentPublicId {
      path: self.path.push(&comment_public_id),
    }
  }
}
