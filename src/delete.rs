mod milestones;
pub use self::milestones::ClubhouseDeleteMilestone;
mod categories;
pub use self::categories::ClubhouseDeleteCategory;
mod stories;
pub use self::stories::ClubhouseDeleteStory;
mod linked_files;
pub use self::linked_files::ClubhouseDeleteLinkedFile;
mod story_links;
pub use self::story_links::ClubhouseDeleteStoryLink;
mod files;
pub use self::files::ClubhouseDeleteFile;
mod epics;
pub use self::epics::ClubhouseDeleteEpic;
mod projects;
pub use self::projects::ClubhouseDeleteProject;
mod labels;
pub use self::labels::ClubhouseDeleteLabel;

pub struct ClubhouseDelete {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseDelete {
    pub fn milestones(self) -> self::milestones::ClubhouseDeleteMilestone {
        self::milestones::ClubhouseDeleteMilestone {
            path: self.path.push(&"milestones"),
        }
    }

    pub fn categories(self) -> self::categories::ClubhouseDeleteCategory {
        self::categories::ClubhouseDeleteCategory {
            path: self.path.push(&"categories"),
        }
    }

    pub fn stories(self) -> self::stories::ClubhouseDeleteStory {
        self::stories::ClubhouseDeleteStory {
            path: self.path.push(&"stories"),
        }
    }

    pub fn linked_files(self) -> self::linked_files::ClubhouseDeleteLinkedFile {
        self::linked_files::ClubhouseDeleteLinkedFile {
            path: self.path.push(&"linked-files"),
        }
    }

    pub fn story_links(self) -> self::story_links::ClubhouseDeleteStoryLink {
        self::story_links::ClubhouseDeleteStoryLink {
            path: self.path.push(&"story-links"),
        }
    }

    pub fn files(self) -> self::files::ClubhouseDeleteFile {
        self::files::ClubhouseDeleteFile {
            path: self.path.push(&"files"),
        }
    }

    pub fn epics(self) -> self::epics::ClubhouseDeleteEpic {
        self::epics::ClubhouseDeleteEpic {
            path: self.path.push(&"epics"),
        }
    }

    pub fn projects(self) -> self::projects::ClubhouseDeleteProject {
        self::projects::ClubhouseDeleteProject {
            path: self.path.push(&"projects"),
        }
    }

    pub fn labels(self) -> self::labels::ClubhouseDeleteLabel {
        self::labels::ClubhouseDeleteLabel {
            path: self.path.push(&"labels"),
        }
    }
}
