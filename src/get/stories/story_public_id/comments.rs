mod comment_public_id;
pub use self::comment_public_id::ClubhouseGetStoryStoryPublicIdCommentCommentPublicId;

pub struct ClubhouseGetStoryStoryPublicIdComment {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseGetStoryStoryPublicIdComment {
    pub fn comment_public_id(
        self,
        comment_public_id: String,
    ) -> self::comment_public_id::ClubhouseGetStoryStoryPublicIdCommentCommentPublicId {
        self::comment_public_id::ClubhouseGetStoryStoryPublicIdCommentCommentPublicId {
            path: self.path.push(&comment_public_id),
        }
    }
}
