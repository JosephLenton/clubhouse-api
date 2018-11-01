pub mod project_public_id;
pub use self::project_public_id::ClubhousePutProjectProjectPublicId;

pub struct ClubhousePutProject {
    pub(crate) path: burgundy::Path,
}

impl ClubhousePutProject {
    pub fn project_public_id(
        self,
        project_public_id: u64,
    ) -> self::project_public_id::ClubhousePutProjectProjectPublicId {
        self::project_public_id::ClubhousePutProjectProjectPublicId {
            path: self.path.push(&project_public_id),
        }
    }
}
