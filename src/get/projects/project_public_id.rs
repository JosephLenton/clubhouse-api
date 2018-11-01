pub mod stories;
pub use self::stories::ClubhouseGetProjectProjectPublicIdStory;

pub struct ClubhouseGetProjectProjectPublicId {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseGetProjectProjectPublicId {
    pub fn stories(self) -> self::stories::ClubhouseGetProjectProjectPublicIdStory {
        self::stories::ClubhouseGetProjectProjectPublicIdStory {
            path: self.path.push(&"stories"),
        }
    }

    /// See https://clubhouse.io/api/rest/v2/#Get-Project
    pub fn run(self) -> crate::Result<crate::types::Project> {
        self.path.execute_as_json::<(), crate::types::Project>(None)
    }
}
