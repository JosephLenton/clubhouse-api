mod comments;
pub use self::comments::ClubhousePutEpicEpicPublicIdComment;

pub struct ClubhousePutEpicEpicPublicId {
    pub(crate) path: burgundy::Path,
}

impl ClubhousePutEpicEpicPublicId {
    pub fn comments(self) -> self::comments::ClubhousePutEpicEpicPublicIdComment {
        self::comments::ClubhousePutEpicEpicPublicIdComment {
            path: self.path.push(&"comments"),
        }
    }

    /// See https://clubhouse.io/api/rest/v2/#Update-Epic
    pub fn run(self, body: crate::types::UpdateEpic) -> burgundy::Result<crate::types::Epic> {
        self.path
            .execute_as_json::<crate::types::UpdateEpic, crate::types::Epic>(Some(&body))
    }
}
