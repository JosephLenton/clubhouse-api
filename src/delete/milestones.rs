mod milestone_public_id;
pub use self::milestone_public_id::ClubhouseDeleteMilestoneMilestonePublicId;

pub struct ClubhouseDeleteMilestone {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseDeleteMilestone {
    pub fn milestone_public_id(
        self,
        milestone_public_id: String,
    ) -> self::milestone_public_id::ClubhouseDeleteMilestoneMilestonePublicId {
        self::milestone_public_id::ClubhouseDeleteMilestoneMilestonePublicId {
            path: self.path.push(&milestone_public_id),
        }
    }
}
