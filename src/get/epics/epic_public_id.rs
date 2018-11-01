pub mod comments;
pub use self::comments::ClubhouseGetEpicEpicPublicIdComment;

pub struct ClubhouseGetEpicEpicPublicId {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseGetEpicEpicPublicId {
    pub fn comments(self) -> self::comments::ClubhouseGetEpicEpicPublicIdComment {
        self::comments::ClubhouseGetEpicEpicPublicIdComment {
            path: self.path.push(&"comments"),
        }
    }

    /// See https://clubhouse.io/api/rest/v2/#Get-Epic
    pub fn run(self) -> crate::Result<crate::types::Epic> {
        self.path.execute_as_json::<(), crate::types::Epic>(None)
    }
}
