mod task_public_id;
pub use self::task_public_id::ClubhouseGetStoryStoryPublicIdTaskTaskPublicId;

pub struct ClubhouseGetStoryStoryPublicIdTask {
  pub(crate) path: burgundy::Path,
}

impl ClubhouseGetStoryStoryPublicIdTask {
  pub fn task_public_id(self, task_public_id : String) -> self::task_public_id::ClubhouseGetStoryStoryPublicIdTaskTaskPublicId {
    self::task_public_id::ClubhouseGetStoryStoryPublicIdTaskTaskPublicId {
      path: self.path.push(&task_public_id),
    }
  }
}
