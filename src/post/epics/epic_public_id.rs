mod comments;
pub use self::comments::ClubhousePostEpicEpicPublicIdComment;

pub struct ClubhousePostEpicEpicPublicId {
    pub(crate) path: burgundy::Path,
}

impl ClubhousePostEpicEpicPublicId {
    pub fn comments(self) -> self::comments::ClubhousePostEpicEpicPublicIdComment {
        self::comments::ClubhousePostEpicEpicPublicIdComment {
            path: self.path.push(&"comments"),
        }
    }
}
