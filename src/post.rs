mod files;
pub use self::files::ClubhousePostFile;
mod epics;
pub use self::epics::ClubhousePostEpic;
mod stories;
pub use self::stories::ClubhousePostStory;
mod linked_files;
pub use self::linked_files::ClubhousePostLinkedFile;
mod milestones;
pub use self::milestones::ClubhousePostMilestone;
mod labels;
pub use self::labels::ClubhousePostLabel;
mod projects;
pub use self::projects::ClubhousePostProject;
mod categories;
pub use self::categories::ClubhousePostCategory;
mod story_links;
pub use self::story_links::ClubhousePostStoryLink;

pub struct ClubhousePost {
    pub(crate) path: burgundy::Path,
}

impl ClubhousePost {
    pub fn files(self) -> self::files::ClubhousePostFile {
        self::files::ClubhousePostFile {
            path: self.path.push(&"files"),
        }
    }

    pub fn epics(self) -> self::epics::ClubhousePostEpic {
        self::epics::ClubhousePostEpic {
            path: self.path.push(&"epics"),
        }
    }

    pub fn stories(self) -> self::stories::ClubhousePostStory {
        self::stories::ClubhousePostStory {
            path: self.path.push(&"stories"),
        }
    }

    pub fn linked_files(self) -> self::linked_files::ClubhousePostLinkedFile {
        self::linked_files::ClubhousePostLinkedFile {
            path: self.path.push(&"linked-files"),
        }
    }

    pub fn milestones(self) -> self::milestones::ClubhousePostMilestone {
        self::milestones::ClubhousePostMilestone {
            path: self.path.push(&"milestones"),
        }
    }

    pub fn labels(self) -> self::labels::ClubhousePostLabel {
        self::labels::ClubhousePostLabel {
            path: self.path.push(&"labels"),
        }
    }

    pub fn projects(self) -> self::projects::ClubhousePostProject {
        self::projects::ClubhousePostProject {
            path: self.path.push(&"projects"),
        }
    }

    pub fn categories(self) -> self::categories::ClubhousePostCategory {
        self::categories::ClubhousePostCategory {
            path: self.path.push(&"categories"),
        }
    }

    pub fn story_links(self) -> self::story_links::ClubhousePostStoryLink {
        self::story_links::ClubhousePostStoryLink {
            path: self.path.push(&"story-links"),
        }
    }
}
