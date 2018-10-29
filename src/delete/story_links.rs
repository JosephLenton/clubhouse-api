mod story_link_public_id;
pub use self::story_link_public_id::ClubhouseDeleteStoryLinkStoryLinkPublicId;

pub struct ClubhouseDeleteStoryLink {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseDeleteStoryLink {
    pub fn story_link_public_id(
        self,
        story_link_public_id: String,
    ) -> self::story_link_public_id::ClubhouseDeleteStoryLinkStoryLinkPublicId {
        self::story_link_public_id::ClubhouseDeleteStoryLinkStoryLinkPublicId {
            path: self.path.push(&story_link_public_id),
        }
    }
}
