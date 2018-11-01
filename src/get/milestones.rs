pub mod milestone_public_id;
pub use self::milestone_public_id::ClubhouseGetMilestoneMilestonePublicId;

pub struct ClubhouseGetMilestone {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseGetMilestone {
    pub fn milestone_public_id(
        self,
        milestone_public_id: u64,
    ) -> self::milestone_public_id::ClubhouseGetMilestoneMilestonePublicId {
        self::milestone_public_id::ClubhouseGetMilestoneMilestonePublicId {
            path: self.path.push(&milestone_public_id),
        }
    }

    /// See https://clubhouse.io/api/rest/v2/#List-Milestones
    pub fn run(self) -> crate::Result<Vec<crate::types::Milestone>> {
        self.path
            .execute_as_json::<(), Vec<crate::types::Milestone>>(None)
    }
}
