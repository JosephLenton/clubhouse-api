mod comment_public_id;
pub use self::comment_public_id::ClubhousePostEpicEpicPublicIdCommentCommentPublicId;

pub struct ClubhousePostEpicEpicPublicIdComment {
  pub(crate) path: burgundy::Path,
}

impl ClubhousePostEpicEpicPublicIdComment {
  pub fn comment_public_id(self, comment_public_id : String) -> self::comment_public_id::ClubhousePostEpicEpicPublicIdCommentCommentPublicId {
    self::comment_public_id::ClubhousePostEpicEpicPublicIdCommentCommentPublicId {
      path: self.path.push(&comment_public_id),
    }
  }

  /// See https://clubhouse.io/api/rest/v2/#Create-Epic-Comment
  pub fn run(self) -> burgundy::Result<crate::types::ThreadedComment> {
    self.path
        .execute_as_json::<crate::types::ThreadedComment>()
  }
}
