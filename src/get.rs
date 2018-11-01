pub mod story_links;
pub use self::story_links::ClubhouseGetStoryLink;
pub mod epics;
pub use self::epics::ClubhouseGetEpic;
pub mod milestones;
pub use self::milestones::ClubhouseGetMilestone;
pub mod labels;
pub use self::labels::ClubhouseGetLabel;
pub mod search;
pub use self::search::ClubhouseGetSearch;
pub mod teams;
pub use self::teams::ClubhouseGetTeam;
pub mod linked_files;
pub use self::linked_files::ClubhouseGetLinkedFile;
pub mod projects;
pub use self::projects::ClubhouseGetProject;
pub mod categories;
pub use self::categories::ClubhouseGetCategory;
pub mod members;
pub use self::members::ClubhouseGetMember;
pub mod stories;
pub use self::stories::ClubhouseGetStory;
pub mod repositories;
pub use self::repositories::ClubhouseGetRepository;
pub mod files;
pub use self::files::ClubhouseGetFile;
pub mod workflows;
pub use self::workflows::ClubhouseGetWorkflow;
pub mod epic_workflow;
pub use self::epic_workflow::ClubhouseGetEpicWorkflow;

pub struct ClubhouseGet {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseGet {
    pub fn story_links(self) -> self::story_links::ClubhouseGetStoryLink {
        self::story_links::ClubhouseGetStoryLink {
            path: self.path.push(&"story-links"),
        }
    }

    pub fn epics(self) -> self::epics::ClubhouseGetEpic {
        self::epics::ClubhouseGetEpic {
            path: self.path.push(&"epics"),
        }
    }

    pub fn milestones(self) -> self::milestones::ClubhouseGetMilestone {
        self::milestones::ClubhouseGetMilestone {
            path: self.path.push(&"milestones"),
        }
    }

    pub fn labels(self) -> self::labels::ClubhouseGetLabel {
        self::labels::ClubhouseGetLabel {
            path: self.path.push(&"labels"),
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

    pub fn linked_files(self) -> self::linked_files::ClubhouseGetLinkedFile {
        self::linked_files::ClubhouseGetLinkedFile {
            path: self.path.push(&"linked-files"),
        }
    }

    pub fn projects(self) -> self::projects::ClubhouseGetProject {
        self::projects::ClubhouseGetProject {
            path: self.path.push(&"projects"),
        }
    }

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

    pub fn stories(self) -> self::stories::ClubhouseGetStory {
        self::stories::ClubhouseGetStory {
            path: self.path.push(&"stories"),
        }
    }

    pub fn repositories(self) -> self::repositories::ClubhouseGetRepository {
        self::repositories::ClubhouseGetRepository {
            path: self.path.push(&"repositories"),
        }
    }

    pub fn files(self) -> self::files::ClubhouseGetFile {
        self::files::ClubhouseGetFile {
            path: self.path.push(&"files"),
        }
    }

    pub fn workflows(self) -> self::workflows::ClubhouseGetWorkflow {
        self::workflows::ClubhouseGetWorkflow {
            path: self.path.push(&"workflows"),
        }
    }

    pub fn epic_workflow(self) -> self::epic_workflow::ClubhouseGetEpicWorkflow {
        self::epic_workflow::ClubhouseGetEpicWorkflow {
            path: self.path.push(&"epic-workflow"),
        }
    }
}
