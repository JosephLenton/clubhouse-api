
use uuid::Uuid;
use chrono::DateTime;
use chrono::Utc;


#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Branch {
  pub created_at : Option<DateTime<Utc>>,
  pub deleted : bool,
  pub entity_type : String,
  pub id : Option<u64>,
  pub merged_branch_ids : Vec<u64>,
  pub name : String,
  pub persistent : bool,
  pub pull_requests : Vec<PullRequest>,
  pub repository_id : Option<u64>,
  pub updated_at : Option<DateTime<Utc>>,
  pub url : String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Category {
  pub archived : bool,
  pub color : Option<String>,
  pub created_at : DateTime<Utc>,
  pub entity_type : String,
  pub external_id : Option<String>,
  pub id : u64,
  pub name : String,
  #[serde(rename = "type")]
  pub category_type : String,
  pub updated_at : DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Comment {
  pub author_id : Option<Uuid>,
  pub created_at : DateTime<Utc>,
  pub entity_type : String,
  pub external_id : Option<String>,
  pub id : u64,
  pub mention_ids : Vec<Uuid>,
  pub position : u64,
  pub story_id : u64,
  pub text : String,
  pub updated_at : Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Commit {
  pub author_email : String,
  pub author_id : Option<Uuid>,
  pub author_identity : Identity,
  pub created_at : DateTime<Utc>,
  pub entity_type : String,
  pub hash : String,
  pub id : Option<u64>,
  pub merged_branch_ids : Vec<u64>,
  pub message : String,
  pub repository_id : Option<u64>,
  pub timestamp : DateTime<Utc>,
  pub updated_at : Option<DateTime<Utc>>,
  pub url : String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateCategoryParams {
  pub color : String,
  pub external_id : String,
  pub name : String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateCommentParams {
  pub author_id : Uuid,
  pub created_at : DateTime<Utc>,
  pub external_id : String,
  pub text : String,
  pub updated_at : DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateLabelParams {
  pub color : String,
  pub external_id : String,
  pub name : String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateStoryLinkParams {
  pub object_id : u64,
  pub subject_id : u64,
  pub verb : StoryLinkType,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateStoryParams {
  pub comments : Vec<CreateCommentParams>,
  pub completed_at_override : DateTime<Utc>,
  pub created_at : DateTime<Utc>,
  pub deadline : Option<DateTime<Utc>>,
  pub description : String,
  pub epic_id : Option<u64>,
  pub estimate : Option<u64>,
  pub external_id : String,
  pub file_ids : Vec<u64>,
  pub follower_ids : Vec<Uuid>,
  pub labels : Vec<CreateLabelParams>,
  pub linked_file_ids : Vec<u64>,
  pub name : String,
  pub owner_ids : Vec<Uuid>,
  pub project_id : u64,
  pub requested_by_id : Uuid,
  pub started_at_override : DateTime<Utc>,
  pub story_links : Vec<CreateStoryLinkParams>,
  pub story_type : StoryType,
  pub tasks : Vec<CreateTaskParams>,
  pub updated_at : DateTime<Utc>,
  pub workflow_state_id : u64,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateTaskParams {
  pub complete : bool,
  pub created_at : DateTime<Utc>,
  pub description : String,
  pub external_id : String,
  pub owner_ids : Vec<Uuid>,
  pub updated_at : DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Epic {
  pub archived : bool,
  pub comments : Vec<ThreadedComment>,
  pub completed : bool,
  pub completed_at : Option<DateTime<Utc>>,
  pub completed_at_override : Option<DateTime<Utc>>,
  pub created_at : Option<DateTime<Utc>>,
  pub deadline : Option<DateTime<Utc>>,
  pub description : String,
  pub entity_type : String,
  pub epic_state_id : u64,
  pub external_id : Option<String>,
  pub follower_ids : Vec<Uuid>,
  pub id : u64,
  pub labels : Vec<Label>,
  pub mention_ids : Vec<Uuid>,
  pub milestone_id : Option<u64>,
  pub name : String,
  pub owner_ids : Vec<Uuid>,
  pub position : u64,
  pub project_ids : Vec<u64>,
  pub requested_by_id : Uuid,
  pub started : bool,
  pub started_at : Option<DateTime<Utc>>,
  pub started_at_override : Option<DateTime<Utc>>,
  pub stats : EpicStats,
  pub updated_at : Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct EpicState {
  pub color : String,
  pub created_at : DateTime<Utc>,
  pub description : String,
  pub entity_type : String,
  pub id : u64,
  pub name : String,
  pub position : u64,
  #[serde(rename = "type")]
  pub epic_state_type : String,
  pub updated_at : DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct EpicStats {
  pub last_story_update : Option<DateTime<Utc>>,
  pub num_points : u64,
  pub num_points_done : u64,
  pub num_points_started : u64,
  pub num_points_unstarted : u64,
  pub num_stories_done : u64,
  pub num_stories_started : u64,
  pub num_stories_unestimated : u64,
  pub num_stories_unstarted : u64,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct EpicWorkflow {
  pub created_at : DateTime<Utc>,
  pub default_epic_state_id : u64,
  pub entity_type : String,
  pub epic_states : Vec<EpicState>,
  pub id : u64,
  pub updated_at : DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct File {
  pub content_type : String,
  pub created_at : DateTime<Utc>,
  pub description : Option<String>,
  pub entity_type : String,
  pub external_id : Option<String>,
  pub filename : String,
  pub id : u64,
  pub mention_ids : Vec<Uuid>,
  pub name : String,
  pub size : u64,
  pub story_ids : Vec<u64>,
  pub thumbnail_url : Option<String>,
  pub updated_at : Option<DateTime<Utc>>,
  pub uploader_id : Uuid,
  pub url : Option<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Icon {
  pub created_at : DateTime<Utc>,
  pub entity_type : String,
  pub id : Uuid,
  pub updated_at : DateTime<Utc>,
  pub url : String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Identity {
  pub entity_type : String,
  pub name : Option<String>,
  #[serde(rename = "type")]
  pub identity_type : Option<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Label {
  pub archived : bool,
  pub color : Option<String>,
  pub created_at : Option<DateTime<Utc>>,
  pub entity_type : String,
  pub external_id : Option<String>,
  pub id : u64,
  pub name : String,
  pub stats : LabelStats,
  pub updated_at : Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct LabelStats {
  pub num_epics : u64,
  pub num_points_completed : u64,
  pub num_points_in_progress : u64,
  pub num_points_total : u64,
  pub num_stories_completed : u64,
  pub num_stories_in_progress : u64,
  pub num_stories_total : u64,
  pub num_stories_unestimated : u64,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct LinkedFile {
  pub content_type : Option<String>,
  pub created_at : DateTime<Utc>,
  pub description : Option<String>,
  pub entity_type : String,
  pub id : u64,
  pub mention_ids : Vec<Uuid>,
  pub name : String,
  pub size : Option<u64>,
  pub story_ids : Vec<u64>,
  pub thumbnail_url : Option<String>,
  #[serde(rename = "type")]
  pub linked_file_type : String,
  pub updated_at : DateTime<Utc>,
  pub uploader_id : Uuid,
  pub url : String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Member {
  pub created_at : Option<DateTime<Utc>>,
  pub disabled : bool,
  pub entity_type : String,
  pub id : Uuid,
  pub profile : Profile,
  pub role : String,
  pub updated_at : Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Milestone {
  pub categories : Vec<Category>,
  pub completed : bool,
  pub completed_at : Option<DateTime<Utc>>,
  pub completed_at_override : Option<DateTime<Utc>>,
  pub created_at : DateTime<Utc>,
  pub description : String,
  pub entity_type : String,
  pub id : u64,
  pub name : String,
  pub position : u64,
  pub started : bool,
  pub started_at : Option<DateTime<Utc>>,
  pub started_at_override : Option<DateTime<Utc>>,
  pub state : String,
  pub updated_at : DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Profile {
  pub deactivated : bool,
  pub display_icon : Option<Icon>,
  pub email_address : Option<String>,
  pub entity_type : String,
  pub gravatar_hash : Option<String>,
  pub id : Uuid,
  pub mention_name : String,
  pub name : String,
  pub two_factor_auth_activated : bool,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Project {
  pub abbreviation : Option<String>,
  pub archived : bool,
  pub color : Option<String>,
  pub created_at : Option<DateTime<Utc>>,
  pub days_to_thermometer : u64,
  pub description : Option<String>,
  pub entity_type : String,
  pub external_id : Option<String>,
  pub follower_ids : Vec<Uuid>,
  pub id : u64,
  pub iteration_length : u64,
  pub name : String,
  pub show_thermometer : bool,
  pub start_time : DateTime<Utc>,
  pub stats : ProjectStats,
  pub team_id : u64,
  pub updated_at : Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct ProjectStats {
  pub num_points : u64,
  pub num_stories : u64,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct PullRequest {
  pub branch_id : u64,
  pub closed : bool,
  pub created_at : DateTime<Utc>,
  pub entity_type : String,
  pub id : u64,
  pub num_added : u64,
  pub num_commits : u64,
  pub num_modified : u64,
  pub num_removed : u64,
  pub number : u64,
  pub target_branch_id : u64,
  pub title : String,
  pub updated_at : DateTime<Utc>,
  pub url : String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Repository {
  pub created_at : Option<DateTime<Utc>>,
  pub entity_type : String,
  pub external_id : Option<String>,
  pub full_name : Option<String>,
  pub id : Option<u64>,
  pub name : Option<String>,
  #[serde(rename = "type")]
  pub repository_type : String,
  pub updated_at : Option<DateTime<Utc>>,
  pub url : Option<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct SearchResults {
  pub data : Vec<StorySearch>,
  pub next : Option<String>,
  pub total : u64,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Story {
  pub app_url : String,
  pub archived : bool,
  pub blocked : bool,
  pub blocker : bool,
  pub branches : Vec<Branch>,
  pub comments : Vec<Comment>,
  pub commits : Vec<Commit>,
  pub completed : bool,
  pub completed_at : Option<DateTime<Utc>>,
  pub completed_at_override : Option<DateTime<Utc>>,
  pub created_at : DateTime<Utc>,
  pub deadline : Option<DateTime<Utc>>,
  pub description : String,
  pub entity_type : String,
  pub epic_id : Option<u64>,
  pub estimate : Option<u64>,
  pub external_id : Option<String>,
  pub files : Vec<File>,
  pub follower_ids : Vec<Uuid>,
  pub id : u64,
  pub labels : Vec<Label>,
  pub linked_files : Vec<LinkedFile>,
  pub mention_ids : Vec<Uuid>,
  pub moved_at : Option<DateTime<Utc>>,
  pub name : String,
  pub owner_ids : Vec<Uuid>,
  pub position : u64,
  pub project_id : u64,
  pub requested_by_id : Uuid,
  pub started : bool,
  pub started_at : Option<DateTime<Utc>>,
  pub started_at_override : Option<DateTime<Utc>>,
  pub story_links : Vec<TypedStoryLink>,
  pub story_type : String,
  pub tasks : Vec<Task>,
  pub updated_at : Option<DateTime<Utc>>,
  pub workflow_state_id : u64,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct StoryLink {
  pub created_at : DateTime<Utc>,
  pub entity_type : String,
  pub id : u64,
  pub object_id : u64,
  pub subject_id : u64,
  pub updated_at : DateTime<Utc>,
  pub verb : String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct StorySearch {
  pub app_url : String,
  pub archived : bool,
  pub blocked : bool,
  pub blocker : bool,
  pub completed : bool,
  pub completed_at : Option<DateTime<Utc>>,
  pub completed_at_override : Option<DateTime<Utc>>,
  pub created_at : DateTime<Utc>,
  pub deadline : Option<DateTime<Utc>>,
  pub description : String,
  pub entity_type : String,
  pub epic_id : Option<u64>,
  pub estimate : Option<u64>,
  pub external_id : Option<String>,
  pub follower_ids : Vec<Uuid>,
  pub id : u64,
  pub labels : Vec<Label>,
  pub mention_ids : Vec<Uuid>,
  pub moved_at : Option<DateTime<Utc>>,
  pub name : String,
  pub owner_ids : Vec<Uuid>,
  pub position : u64,
  pub project_id : u64,
  pub requested_by_id : Uuid,
  pub started : bool,
  pub started_at : Option<DateTime<Utc>>,
  pub started_at_override : Option<DateTime<Utc>>,
  pub story_links : Vec<TypedStoryLink>,
  pub story_type : String,
  pub updated_at : Option<DateTime<Utc>>,
  pub workflow_state_id : u64,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct StorySlim {
  pub app_url : String,
  pub archived : bool,
  pub blocked : bool,
  pub blocker : bool,
  pub comment_ids : Vec<u64>,
  pub completed : bool,
  pub completed_at : Option<DateTime<Utc>>,
  pub completed_at_override : Option<DateTime<Utc>>,
  pub created_at : DateTime<Utc>,
  pub deadline : Option<DateTime<Utc>>,
  pub entity_type : String,
  pub epic_id : Option<u64>,
  pub estimate : Option<u64>,
  pub external_id : Option<String>,
  pub file_ids : Vec<u64>,
  pub follower_ids : Vec<Uuid>,
  pub id : u64,
  pub labels : Vec<Label>,
  pub linked_file_ids : Vec<u64>,
  pub mention_ids : Vec<Uuid>,
  pub moved_at : Option<DateTime<Utc>>,
  pub name : String,
  pub owner_ids : Vec<Uuid>,
  pub position : u64,
  pub project_id : u64,
  pub requested_by_id : Uuid,
  pub started : bool,
  pub started_at : Option<DateTime<Utc>>,
  pub started_at_override : Option<DateTime<Utc>>,
  pub story_links : Vec<TypedStoryLink>,
  pub story_type : String,
  pub task_ids : Vec<u64>,
  pub updated_at : Option<DateTime<Utc>>,
  pub workflow_state_id : u64,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Task {
  pub complete : bool,
  pub completed_at : Option<DateTime<Utc>>,
  pub created_at : DateTime<Utc>,
  pub description : String,
  pub entity_type : String,
  pub external_id : Option<String>,
  pub id : u64,
  pub mention_ids : Vec<Uuid>,
  pub owner_ids : Vec<Uuid>,
  pub position : u64,
  pub story_id : u64,
  pub updated_at : Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Team {
  pub created_at : DateTime<Utc>,
  pub description : String,
  pub entity_type : String,
  pub id : u64,
  pub name : String,
  pub position : u64,
  pub project_ids : Vec<u64>,
  pub updated_at : DateTime<Utc>,
  pub workflow : Workflow,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct ThreadedComment {
  pub author_id : Uuid,
  pub comments : Vec<ThreadedComment>,
  pub created_at : DateTime<Utc>,
  pub deleted : bool,
  pub entity_type : String,
  pub external_id : Option<String>,
  pub id : u64,
  pub mention_ids : Vec<Uuid>,
  pub text : String,
  pub updated_at : DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct TypedStoryLink {
  pub created_at : DateTime<Utc>,
  pub entity_type : String,
  pub id : u64,
  pub object_id : u64,
  pub subject_id : u64,
  #[serde(rename = "type")]
  pub typed_story_link_type : String,
  pub updated_at : DateTime<Utc>,
  pub verb : String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Workflow {
  pub created_at : DateTime<Utc>,
  pub default_state_id : u64,
  pub description : String,
  pub entity_type : String,
  pub id : u64,
  pub name : String,
  pub states : Vec<WorkflowState>,
  pub team_id : u64,
  pub updated_at : DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct WorkflowState {
  pub color : String,
  pub created_at : DateTime<Utc>,
  pub description : String,
  pub entity_type : String,
  pub id : u64,
  pub name : String,
  pub num_stories : u64,
  pub position : u64,
  #[serde(rename = "type")]
  pub workflow_state_type : String,
  pub updated_at : DateTime<Utc>,
  pub verb : Option<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq)]
pub enum StoryType {
    #[serde(rename = "bug")]
    Bug,
    #[serde(rename = "chore")]
    Chore,
    #[serde(rename = "feature")]
    Feature,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq)]
pub enum StoryLinkType {
    #[serde(rename = "blocks")]
    Blocks,
    #[serde(rename = "duplicates")]
    Duplicates,
    #[serde(rename = "relates to")]
    RelatesTo,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq)]
pub enum CategoryType {
    #[serde(rename = "milestone")]
    Milestone,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq)]
pub enum FileIntegrationType {
    #[serde(rename = "box")]
    Box,
    #[serde(rename = "dropbox")]
    Dropbox,
    #[serde(rename = "google")]
    Google,
    #[serde(rename = "onedrive")]
    OneDrive,
    #[serde(rename = "url")]
    Url,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq)]
pub enum StateType {
    #[serde(rename = "done")]
    Done,
    #[serde(rename = "in progress")]
    InProgress,
    #[serde(rename = "to do")]
    ToDo,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateCategory {
  pub color : Option<String>,
  pub external_id : Option<String>,
  pub name : String,
  pub category_type : CategoryType,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct UpdateCategory {
  pub archived : Option<bool>,
  pub color : Option<String>,
  pub name : Option<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateEpic {
  pub completed_at_override : Option<DateTime<Utc>>,
  pub created_at : Option<DateTime<Utc>>,
  pub deadline : Option<DateTime<Utc>>,
  pub description : Option<String>,
  pub epic_state_id : Option<u64>,
  pub external_id : Option<String>,
  pub follower_ids : Option<Vec<Uuid>>,
  pub labels : Option<Vec<CreateLabelParams>>,
  pub milestone_id : Option<u64>,
  pub name : String,
  pub owner_ids : Option<Vec<Uuid>>,
  pub requested_by_id : Option<Uuid>,
  pub started_at_override : Option<DateTime<Utc>>,
  pub updated_at : Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct UpdateEpic {
  pub after_id : Option<u64>,
  pub archived : Option<bool>,
  pub before_id : Option<u64>,
  pub completed_at_override : Option<DateTime<Utc>>,
  pub deadline : Option<DateTime<Utc>>,
  pub description : Option<String>,
  pub epic_state_id : Option<u64>,
  pub follower_ids : Option<Vec<Uuid>>,
  pub labels : Option<Vec<CreateLabelParams>>,
  pub milestone_id : Option<u64>,
  pub name : Option<String>,
  pub owner_ids : Option<Vec<Uuid>>,
  pub requested_by_id : Option<Uuid>,
  pub started_at_override : Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateEpicComment {
  pub author_id : Option<Uuid>,
  pub created_at : Option<DateTime<Utc>>,
  pub external_id : Option<String>,
  pub text : String,
  pub updated_at : Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateEpicCommentComment {
  pub external_id : Option<String>,
  pub text : String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct UpdateEpicComment {
  pub text : String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct UpdateFile {
  pub created_at : Option<DateTime<Utc>>,
  pub description : Option<String>,
  pub external_id : Option<String>,
  pub name : Option<String>,
  pub updated_at : Option<DateTime<Utc>>,
  pub uploader_id : Option<Uuid>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateLabel {
  pub color : Option<String>,
  pub external_id : Option<String>,
  pub name : String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct UpdateLabel {
  pub archived : Option<bool>,
  pub color : Option<String>,
  pub name : Option<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateLinkedFile {
  pub content_type : Option<String>,
  pub description : Option<String>,
  pub name : String,
  pub size : Option<u64>,
  pub story_id : Option<u64>,
  pub thumbnail_url : Option<String>,
  pub file_integration_type : FileIntegrationType,
  pub uploader_id : Option<Uuid>,
  pub url : String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct UpdateLinkedFile {
  pub description : Option<String>,
  pub name : Option<String>,
  pub size : Option<String>,
  pub thumbnail_url : Option<String>,
  pub file_integration_type : Option<FileIntegrationType>,
  pub uploader_id : Option<Uuid>,
  pub url : Option<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateMilestone {
  pub categories : Option<Vec<CreateCategoryParams>>,
  pub completed_at_override : Option<DateTime<Utc>>,
  pub description : Option<String>,
  pub name : String,
  pub started_at_override : Option<DateTime<Utc>>,
  pub state : Option<StateType>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct UpdateMilestone {
  pub after_id : Option<u64>,
  pub before_id : Option<u64>,
  pub categories : Option<Vec<CreateCategoryParams>>,
  pub completed_at_override : Option<DateTime<Utc>>,
  pub description : Option<String>,
  pub name : Option<String>,
  pub started_at_override : Option<DateTime<Utc>>,
  pub state : Option<StateType>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateProject {
  pub abbreviation : Option<String>,
  pub color : Option<String>,
  pub created_at : Option<DateTime<Utc>>,
  pub description : Option<String>,
  pub external_id : Option<String>,
  pub follower_ids : Option<Vec<Uuid>>,
  pub iteration_length : Option<u64>,
  pub name : String,
  pub team_id : Option<u64>,
  pub updated_at : Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct UpdateProject {
  pub abbreviation : Option<String>,
  pub archived : Option<bool>,
  pub color : Option<String>,
  pub days_to_thermometer : Option<u64>,
  pub description : Option<String>,
  pub follower_ids : Option<Vec<Uuid>>,
  pub name : Option<String>,
  pub show_thermometer : Option<bool>,
  pub team_id : Option<u64>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct SearchStories {
  pub page_size : Option<u64>,
  pub query : String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateStory {
  pub comments : Option<Vec<CreateCommentParams>>,
  pub completed_at_override : Option<DateTime<Utc>>,
  pub created_at : Option<DateTime<Utc>>,
  pub deadline : Option<DateTime<Utc>>,
  pub description : Option<String>,
  pub epic_id : Option<u64>,
  pub estimate : Option<u64>,
  pub external_id : Option<String>,
  pub file_ids : Option<Vec<u64>>,
  pub follower_ids : Option<Vec<Uuid>>,
  pub labels : Option<Vec<CreateLabelParams>>,
  pub linked_file_ids : Option<Vec<u64>>,
  pub name : String,
  pub owner_ids : Option<Vec<Uuid>>,
  pub project_id : u64,
  pub requested_by_id : Option<Uuid>,
  pub started_at_override : Option<DateTime<Utc>>,
  pub story_links : Option<Vec<CreateStoryLinkParams>>,
  pub story_type : Option<StoryType>,
  pub tasks : Option<Vec<CreateTaskParams>>,
  pub updated_at : Option<DateTime<Utc>>,
  pub workflow_state_id : Option<u64>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateMultipleStories {
  pub stories : Vec<CreateStoryParams>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct UpdateMultipleStories {
  pub after_id : Option<u64>,
  pub archived : Option<bool>,
  pub deadline : Option<DateTime<Utc>>,
  pub epic_id : Option<u64>,
  pub estimate : Option<u64>,
  pub follower_ids_add : Option<Vec<Uuid>>,
  pub follower_ids_remove : Option<Vec<Uuid>>,
  pub labels_add : Option<Vec<CreateLabelParams>>,
  pub labels_remove : Option<Vec<CreateLabelParams>>,
  pub owner_ids_add : Option<Vec<Uuid>>,
  pub owner_ids_remove : Option<Vec<Uuid>>,
  pub project_id : Option<u64>,
  pub requested_by_id : Option<Uuid>,
  pub story_ids : Vec<u64>,
  pub story_type : Option<StoryType>,
  pub workflow_state_id : Option<u64>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct DeleteMultipleStories {
  pub story_ids : Vec<u64>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct UpdateStory {
  pub after_id : Option<u64>,
  pub archived : Option<bool>,
  pub before_id : Option<u64>,
  pub branch_ids : Option<Vec<u64>>,
  pub commit_ids : Option<Vec<u64>>,
  pub completed_at_override : Option<DateTime<Utc>>,
  pub deadline : Option<DateTime<Utc>>,
  pub description : Option<String>,
  pub epic_id : Option<u64>,
  pub estimate : Option<u64>,
  pub file_ids : Option<Vec<u64>>,
  pub follower_ids : Option<Vec<Uuid>>,
  pub labels : Option<Vec<CreateLabelParams>>,
  pub linked_file_ids : Option<Vec<u64>>,
  pub name : Option<String>,
  pub owner_ids : Option<Vec<Uuid>>,
  pub project_id : Option<u64>,
  pub requested_by_id : Option<Uuid>,
  pub started_at_override : Option<DateTime<Utc>>,
  pub story_type : Option<StoryType>,
  pub workflow_state_id : Option<u64>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateComment {
  pub author_id : Option<Uuid>,
  pub created_at : Option<DateTime<Utc>>,
  pub external_id : Option<String>,
  pub text : String,
  pub updated_at : Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct UpdateComment {
  pub text : String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateTask {
  pub complete : Option<bool>,
  pub created_at : Option<DateTime<Utc>>,
  pub description : String,
  pub external_id : Option<String>,
  pub owner_ids : Option<Vec<Uuid>>,
  pub updated_at : Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct UpdateTask {
  pub after_id : Option<u64>,
  pub before_id : Option<u64>,
  pub complete : Option<bool>,
  pub description : Option<String>,
  pub owner_ids : Option<Vec<Uuid>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateStoryLink {
  pub object_id : u64,
  pub subject_id : u64,
  pub verb : StoryLinkType,
}
