mod project_public_id;
pub use self::project_public_id::ClubhouseGetProjectProjectPublicId;

pub struct ClubhouseGetProject {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseGetProject {
    pub fn project_public_id(
        self,
        project_public_id: String,
    ) -> self::project_public_id::ClubhouseGetProjectProjectPublicId {
        self::project_public_id::ClubhouseGetProjectProjectPublicId {
            path: self.path.push(&project_public_id),
        }
    }

    /// See https://clubhouse.io/api/rest/v2/#List-Projects
    pub fn run(self) -> burgundy::Result<Vec<crate::types::Project>> {
        self.path
            .execute_as_json::<(), Vec<crate::types::Project>>(None)
    }
}
