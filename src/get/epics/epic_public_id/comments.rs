mod comment_public_id;
pub use self::comment_public_id::ClubhouseGetEpicEpicPublicIdCommentCommentPublicId;

pub struct ClubhouseGetEpicEpicPublicIdComment {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseGetEpicEpicPublicIdComment {
    pub fn comment_public_id(
        self,
        comment_public_id: String,
    ) -> self::comment_public_id::ClubhouseGetEpicEpicPublicIdCommentCommentPublicId {
        self::comment_public_id::ClubhouseGetEpicEpicPublicIdCommentCommentPublicId {
            path: self.path.push(&comment_public_id),
        }
    }

    /// See https://clubhouse.io/api/rest/v2/#List-Epic-Comments
    pub fn run(self) -> burgundy::Result<Vec<crate::types::ThreadedComment>> {
        self.path
            .execute_as_json::<(), Vec<crate::types::ThreadedComment>>(None)
    }
}
