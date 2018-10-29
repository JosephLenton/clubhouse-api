mod task_public_id;
pub use self::task_public_id::ClubhouseDeleteStoryStoryPublicIdTaskTaskPublicId;

pub struct ClubhouseDeleteStoryStoryPublicIdTask {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseDeleteStoryStoryPublicIdTask {
    pub fn task_public_id(
        self,
        task_public_id: String,
    ) -> self::task_public_id::ClubhouseDeleteStoryStoryPublicIdTaskTaskPublicId {
        self::task_public_id::ClubhouseDeleteStoryStoryPublicIdTaskTaskPublicId {
            path: self.path.push(&task_public_id),
        }
    }
}
