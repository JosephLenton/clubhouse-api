pub mod task_public_id;
pub use self::task_public_id::ClubhousePutStoryStoryPublicIdTaskTaskPublicId;

pub struct ClubhousePutStoryStoryPublicIdTask {
    pub(crate) path: burgundy::Path,
}

impl ClubhousePutStoryStoryPublicIdTask {
    pub fn task_public_id(
        self,
        task_public_id: u64,
    ) -> self::task_public_id::ClubhousePutStoryStoryPublicIdTaskTaskPublicId {
        self::task_public_id::ClubhousePutStoryStoryPublicIdTaskTaskPublicId {
            path: self.path.push(&task_public_id),
        }
    }
}
