pub mod story_link_public_id;
pub use self::story_link_public_id::ClubhouseGetStoryLinkStoryLinkPublicId;

pub struct ClubhouseGetStoryLink {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseGetStoryLink {
    pub fn story_link_public_id(
        self,
        story_link_public_id: u64,
    ) -> self::story_link_public_id::ClubhouseGetStoryLinkStoryLinkPublicId {
        self::story_link_public_id::ClubhouseGetStoryLinkStoryLinkPublicId {
            path: self.path.push(&story_link_public_id),
        }
    }
}
