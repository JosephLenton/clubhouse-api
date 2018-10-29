mod categories;
pub use self::categories::ClubhouseGetCategory;
mod members;
pub use self::members::ClubhouseGetMember;
mod repositories;
pub use self::repositories::ClubhouseGetRepository;
mod workflows;
pub use self::workflows::ClubhouseGetWorkflow;
mod search;
pub use self::search::ClubhouseGetSearch;
mod teams;
pub use self::teams::ClubhouseGetTeam;
mod epic_workflow;
pub use self::epic_workflow::ClubhouseGetEpicWorkflow;
mod milestones;
pub use self::milestones::ClubhouseGetMilestone;
mod files;
pub use self::files::ClubhouseGetFile;
mod linked_files;
pub use self::linked_files::ClubhouseGetLinkedFile;
mod stories;
pub use self::stories::ClubhouseGetStory;
mod projects;
pub use self::projects::ClubhouseGetProject;
mod story_links;
pub use self::story_links::ClubhouseGetStoryLink;
mod labels;
pub use self::labels::ClubhouseGetLabel;
mod epics;
pub use self::epics::ClubhouseGetEpic;

pub struct ClubhouseGet {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseGet {
    pub fn categories(self) -> self::categories::ClubhouseGetCategory {
        self::categories::ClubhouseGetCategory {
            path: self.path.push(&"categories"),
        }
    }

    pub fn members(self) -> self::members::ClubhouseGetMember {
        self::members::ClubhouseGetMember {
            path: self.path.push(&"members"),
        }
    }

    pub fn repositories(self) -> self::repositories::ClubhouseGetRepository {
        self::repositories::ClubhouseGetRepository {
            path: self.path.push(&"repositories"),
        }
    }

    pub fn workflows(self) -> self::workflows::ClubhouseGetWorkflow {
        self::workflows::ClubhouseGetWorkflow {
            path: self.path.push(&"workflows"),
        }
    }

    pub fn search(self) -> self::search::ClubhouseGetSearch {
        self::search::ClubhouseGetSearch {
            path: self.path.push(&"search"),
        }
    }

    pub fn teams(self) -> self::teams::ClubhouseGetTeam {
        self::teams::ClubhouseGetTeam {
            path: self.path.push(&"teams"),
        }
    }

    pub fn epic_workflow(self) -> self::epic_workflow::ClubhouseGetEpicWorkflow {
        self::epic_workflow::ClubhouseGetEpicWorkflow {
            path: self.path.push(&"epic-workflow"),
        }
    }

    pub fn milestones(self) -> self::milestones::ClubhouseGetMilestone {
        self::milestones::ClubhouseGetMilestone {
            path: self.path.push(&"milestones"),
        }
    }

    pub fn files(self) -> self::files::ClubhouseGetFile {
        self::files::ClubhouseGetFile {
            path: self.path.push(&"files"),
        }
    }

    pub fn linked_files(self) -> self::linked_files::ClubhouseGetLinkedFile {
        self::linked_files::ClubhouseGetLinkedFile {
            path: self.path.push(&"linked-files"),
        }
    }

    pub fn stories(self) -> self::stories::ClubhouseGetStory {
        self::stories::ClubhouseGetStory {
            path: self.path.push(&"stories"),
        }
    }

    pub fn projects(self) -> self::projects::ClubhouseGetProject {
        self::projects::ClubhouseGetProject {
            path: self.path.push(&"projects"),
        }
    }

    pub fn story_links(self) -> self::story_links::ClubhouseGetStoryLink {
        self::story_links::ClubhouseGetStoryLink {
            path: self.path.push(&"story-links"),
        }
    }

    pub fn labels(self) -> self::labels::ClubhouseGetLabel {
        self::labels::ClubhouseGetLabel {
            path: self.path.push(&"labels"),
        }
    }

    pub fn epics(self) -> self::epics::ClubhouseGetEpic {
        self::epics::ClubhouseGetEpic {
            path: self.path.push(&"epics"),
        }
    }
}
