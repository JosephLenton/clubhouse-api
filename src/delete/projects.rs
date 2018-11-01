pub mod project_public_id;
pub use self::project_public_id::ClubhouseDeleteProjectProjectPublicId;

pub struct ClubhouseDeleteProject {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseDeleteProject {
    pub fn project_public_id(
        self,
        project_public_id: u64,
    ) -> self::project_public_id::ClubhouseDeleteProjectProjectPublicId {
        self::project_public_id::ClubhouseDeleteProjectProjectPublicId {
            path: self.path.push(&project_public_id),
        }
    }
}
