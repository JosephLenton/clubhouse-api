pub mod projects;
pub use self::projects::ClubhousePostProject;
pub mod epics;
pub use self::epics::ClubhousePostEpic;
pub mod categories;
pub use self::categories::ClubhousePostCategory;
pub mod labels;
pub use self::labels::ClubhousePostLabel;
pub mod milestones;
pub use self::milestones::ClubhousePostMilestone;
pub mod story_links;
pub use self::story_links::ClubhousePostStoryLink;
pub mod stories;
pub use self::stories::ClubhousePostStory;
pub mod files;
pub use self::files::ClubhousePostFile;
pub mod linked_files;
pub use self::linked_files::ClubhousePostLinkedFile;

pub struct ClubhousePost {
    pub(crate) path: burgundy::Path,
}

impl ClubhousePost {
    pub fn projects(self) -> self::projects::ClubhousePostProject {
        self::projects::ClubhousePostProject {
            path: self.path.push(&"projects"),
        }
    }

    pub fn epics(self) -> self::epics::ClubhousePostEpic {
        self::epics::ClubhousePostEpic {
            path: self.path.push(&"epics"),
        }
    }

    pub fn categories(self) -> self::categories::ClubhousePostCategory {
        self::categories::ClubhousePostCategory {
            path: self.path.push(&"categories"),
        }
    }

    pub fn labels(self) -> self::labels::ClubhousePostLabel {
        self::labels::ClubhousePostLabel {
            path: self.path.push(&"labels"),
        }
    }

    pub fn milestones(self) -> self::milestones::ClubhousePostMilestone {
        self::milestones::ClubhousePostMilestone {
            path: self.path.push(&"milestones"),
        }
    }

    pub fn story_links(self) -> self::story_links::ClubhousePostStoryLink {
        self::story_links::ClubhousePostStoryLink {
            path: self.path.push(&"story-links"),
        }
    }

    pub fn stories(self) -> self::stories::ClubhousePostStory {
        self::stories::ClubhousePostStory {
            path: self.path.push(&"stories"),
        }
    }

    pub fn files(self) -> self::files::ClubhousePostFile {
        self::files::ClubhousePostFile {
            path: self.path.push(&"files"),
        }
    }

    pub fn linked_files(self) -> self::linked_files::ClubhousePostLinkedFile {
        self::linked_files::ClubhousePostLinkedFile {
            path: self.path.push(&"linked-files"),
        }
    }
}
