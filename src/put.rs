mod labels;
pub use self::labels::ClubhousePutLabel;
mod epics;
pub use self::epics::ClubhousePutEpic;
mod linked_files;
pub use self::linked_files::ClubhousePutLinkedFile;
mod stories;
pub use self::stories::ClubhousePutStory;
mod files;
pub use self::files::ClubhousePutFile;
mod projects;
pub use self::projects::ClubhousePutProject;
mod categories;
pub use self::categories::ClubhousePutCategory;
mod milestones;
pub use self::milestones::ClubhousePutMilestone;

pub struct ClubhousePut {
    pub(crate) path: burgundy::Path,
}

impl ClubhousePut {
    pub fn labels(self) -> self::labels::ClubhousePutLabel {
        self::labels::ClubhousePutLabel {
            path: self.path.push(&"labels"),
        }
    }

    pub fn epics(self) -> self::epics::ClubhousePutEpic {
        self::epics::ClubhousePutEpic {
            path: self.path.push(&"epics"),
        }
    }

    pub fn linked_files(self) -> self::linked_files::ClubhousePutLinkedFile {
        self::linked_files::ClubhousePutLinkedFile {
            path: self.path.push(&"linked-files"),
        }
    }

    pub fn stories(self) -> self::stories::ClubhousePutStory {
        self::stories::ClubhousePutStory {
            path: self.path.push(&"stories"),
        }
    }

    pub fn files(self) -> self::files::ClubhousePutFile {
        self::files::ClubhousePutFile {
            path: self.path.push(&"files"),
        }
    }

    pub fn projects(self) -> self::projects::ClubhousePutProject {
        self::projects::ClubhousePutProject {
            path: self.path.push(&"projects"),
        }
    }

    pub fn categories(self) -> self::categories::ClubhousePutCategory {
        self::categories::ClubhousePutCategory {
            path: self.path.push(&"categories"),
        }
    }

    pub fn milestones(self) -> self::milestones::ClubhousePutMilestone {
        self::milestones::ClubhousePutMilestone {
            path: self.path.push(&"milestones"),
        }
    }
}
