mod milestone_public_id;
pub use self::milestone_public_id::ClubhousePutMilestoneMilestonePublicId;

pub struct ClubhousePutMilestone {
    pub(crate) path: burgundy::Path,
}

impl ClubhousePutMilestone {
    pub fn milestone_public_id(
        self,
        milestone_public_id: String,
    ) -> self::milestone_public_id::ClubhousePutMilestoneMilestonePublicId {
        self::milestone_public_id::ClubhousePutMilestoneMilestonePublicId {
            path: self.path.push(&milestone_public_id),
        }
    }
}
