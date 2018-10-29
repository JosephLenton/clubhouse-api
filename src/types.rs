use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Branch {
    created_at: Option<DateTime<Utc>>,
    deleted: bool,
    entity_type: String,
    id: Option<u32>,
    merged_branch_ids: Vec<u32>,
    name: String,
    persistent: bool,
    pull_requests: Vec<PullRequest>,
    repository_id: Option<u32>,
    updated_at: Option<DateTime<Utc>>,
    url: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Category {
    archived: bool,
    color: Option<String>,
    created_at: DateTime<Utc>,
    entity_type: String,
    external_id: Option<String>,
    id: u32,
    name: String,
    #[serde(rename = "type")]
    category_type: String,
    updated_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Comment {
    author_id: Option<Uuid>,
    created_at: DateTime<Utc>,
    entity_type: String,
    external_id: Option<String>,
    id: u32,
    mention_ids: Vec<Uuid>,
    position: u32,
    story_id: u32,
    text: String,
    updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Commit {
    author_email: String,
    author_id: Option<Uuid>,
    author_identity: Identity,
    created_at: DateTime<Utc>,
    entity_type: String,
    hash: String,
    id: Option<u32>,
    merged_branch_ids: Vec<u32>,
    message: String,
    repository_id: Option<u32>,
    timestamp: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
    url: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateCategoryParams {
    color: String,
    external_id: String,
    name: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateCommentParams {
    author_id: Uuid,
    created_at: DateTime<Utc>,
    external_id: String,
    text: String,
    updated_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateLabelParams {
    color: String,
    external_id: String,
    name: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateStoryLinkParams {
    object_id: u32,
    subject_id: u32,
    verb: StoryLinkType,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateStoryParams {
    comments: Vec<CreateCommentParams>,
    completed_at_override: DateTime<Utc>,
    created_at: DateTime<Utc>,
    deadline: Option<DateTime<Utc>>,
    description: String,
    epic_id: Option<u32>,
    estimate: Option<u32>,
    external_id: String,
    file_ids: Vec<u32>,
    follower_ids: Vec<Uuid>,
    labels: Vec<CreateLabelParams>,
    linked_file_ids: Vec<u32>,
    name: String,
    owner_ids: Vec<Uuid>,
    project_id: u32,
    requested_by_id: Uuid,
    started_at_override: DateTime<Utc>,
    story_links: Vec<CreateStoryLinkParams>,
    story_type: StoryType,
    tasks: Vec<CreateTaskParams>,
    updated_at: DateTime<Utc>,
    workflow_state_id: u32,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateTaskParams {
    complete: bool,
    created_at: DateTime<Utc>,
    description: String,
    external_id: String,
    owner_ids: Vec<Uuid>,
    updated_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Epic {
    archived: bool,
    comments: Vec<ThreadedComment>,
    completed: bool,
    completed_at: Option<DateTime<Utc>>,
    completed_at_override: Option<DateTime<Utc>>,
    created_at: Option<DateTime<Utc>>,
    deadline: Option<DateTime<Utc>>,
    description: String,
    entity_type: String,
    epic_state_id: u32,
    external_id: Option<String>,
    follower_ids: Vec<Uuid>,
    id: u32,
    labels: Vec<Label>,
    mention_ids: Vec<Uuid>,
    milestone_id: Option<u32>,
    name: String,
    owner_ids: Vec<Uuid>,
    position: u32,
    project_ids: Vec<u32>,
    requested_by_id: Uuid,
    started: bool,
    started_at: Option<DateTime<Utc>>,
    started_at_override: Option<DateTime<Utc>>,
    stats: EpicStats,
    updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct EpicState {
    color: String,
    created_at: DateTime<Utc>,
    description: String,
    entity_type: String,
    id: u32,
    name: String,
    position: u32,
    #[serde(rename = "type")]
    epic_state_type: String,
    updated_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct EpicStats {
    last_story_update: Option<DateTime<Utc>>,
    num_points: u32,
    num_points_done: u32,
    num_points_started: u32,
    num_points_unstarted: u32,
    num_stories_done: u32,
    num_stories_started: u32,
    num_stories_unestimated: u32,
    num_stories_unstarted: u32,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct EpicWorkflow {
    created_at: DateTime<Utc>,
    default_epic_state_id: u32,
    entity_type: String,
    epic_states: Vec<EpicState>,
    id: u32,
    updated_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct File {
    content_type: String,
    created_at: DateTime<Utc>,
    description: Option<String>,
    entity_type: String,
    external_id: Option<String>,
    filename: String,
    id: u32,
    mention_ids: Vec<Uuid>,
    name: String,
    size: u32,
    story_ids: Vec<u32>,
    thumbnail_url: Option<String>,
    updated_at: Option<DateTime<Utc>>,
    uploader_id: Uuid,
    url: Option<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Icon {
    created_at: DateTime<Utc>,
    entity_type: String,
    id: Uuid,
    updated_at: DateTime<Utc>,
    url: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Identity {
    entity_type: String,
    name: Option<String>,
    #[serde(rename = "type")]
    identity_type: Option<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Label {
    archived: bool,
    color: Option<String>,
    created_at: Option<DateTime<Utc>>,
    entity_type: String,
    external_id: Option<String>,
    id: u32,
    name: String,
    stats: LabelStats,
    updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct LabelStats {
    num_epics: u32,
    num_points_completed: u32,
    num_points_in_progress: u32,
    num_points_total: u32,
    num_stories_completed: u32,
    num_stories_in_progress: u32,
    num_stories_total: u32,
    num_stories_unestimated: u32,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct LinkedFile {
    content_type: Option<String>,
    created_at: DateTime<Utc>,
    description: Option<String>,
    entity_type: String,
    id: u32,
    mention_ids: Vec<Uuid>,
    name: String,
    size: Option<u32>,
    story_ids: Vec<u32>,
    thumbnail_url: Option<String>,
    #[serde(rename = "type")]
    linked_file_type: String,
    updated_at: DateTime<Utc>,
    uploader_id: Uuid,
    url: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Member {
    created_at: Option<DateTime<Utc>>,
    disabled: bool,
    entity_type: String,
    id: Uuid,
    profile: Profile,
    role: String,
    updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Milestone {
    categories: Vec<Category>,
    completed: bool,
    completed_at: Option<DateTime<Utc>>,
    completed_at_override: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    description: String,
    entity_type: String,
    id: u32,
    name: String,
    position: u32,
    started: bool,
    started_at: Option<DateTime<Utc>>,
    started_at_override: Option<DateTime<Utc>>,
    state: String,
    updated_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Profile {
    deactivated: bool,
    display_icon: Option<Icon>,
    email_address: Option<String>,
    entity_type: String,
    gravatar_hash: Option<String>,
    id: Uuid,
    mention_name: String,
    name: String,
    two_factor_auth_activated: bool,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Project {
    abbreviation: Option<String>,
    archived: bool,
    color: Option<String>,
    created_at: Option<DateTime<Utc>>,
    days_to_thermometer: u32,
    description: Option<String>,
    entity_type: String,
    external_id: Option<String>,
    follower_ids: Vec<Uuid>,
    id: u32,
    iteration_length: u32,
    name: String,
    show_thermometer: bool,
    start_time: DateTime<Utc>,
    stats: ProjectStats,
    team_id: u32,
    updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct ProjectStats {
    num_points: u32,
    num_stories: u32,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct PullRequest {
    branch_id: u32,
    closed: bool,
    created_at: DateTime<Utc>,
    entity_type: String,
    id: u32,
    num_added: u32,
    num_commits: u32,
    num_modified: u32,
    num_removed: u32,
    number: u32,
    target_branch_id: u32,
    title: String,
    updated_at: DateTime<Utc>,
    url: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Repository {
    created_at: Option<DateTime<Utc>>,
    entity_type: String,
    external_id: Option<String>,
    full_name: Option<String>,
    id: Option<u32>,
    name: Option<String>,
    #[serde(rename = "type")]
    repository_type: String,
    updated_at: Option<DateTime<Utc>>,
    url: Option<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct SearchResults {
    data: Vec<StorySearch>,
    next: Option<String>,
    total: u32,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Story {
    app_url: String,
    archived: bool,
    blocked: bool,
    blocker: bool,
    branches: Vec<Branch>,
    comments: Vec<Comment>,
    commits: Vec<Commit>,
    completed: bool,
    completed_at: Option<DateTime<Utc>>,
    completed_at_override: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    deadline: Option<DateTime<Utc>>,
    description: String,
    entity_type: String,
    epic_id: Option<u32>,
    estimate: Option<u32>,
    external_id: Option<String>,
    files: Vec<File>,
    follower_ids: Vec<Uuid>,
    id: u32,
    labels: Vec<Label>,
    linked_files: Vec<LinkedFile>,
    mention_ids: Vec<Uuid>,
    moved_at: Option<DateTime<Utc>>,
    name: String,
    owner_ids: Vec<Uuid>,
    position: u32,
    project_id: u32,
    requested_by_id: Uuid,
    started: bool,
    started_at: Option<DateTime<Utc>>,
    started_at_override: Option<DateTime<Utc>>,
    story_links: Vec<TypedStoryLink>,
    story_type: String,
    tasks: Vec<Task>,
    updated_at: Option<DateTime<Utc>>,
    workflow_state_id: u32,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct StoryLink {
    created_at: DateTime<Utc>,
    entity_type: String,
    id: u32,
    object_id: u32,
    subject_id: u32,
    updated_at: DateTime<Utc>,
    verb: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct StorySearch {
    app_url: String,
    archived: bool,
    blocked: bool,
    blocker: bool,
    completed: bool,
    completed_at: Option<DateTime<Utc>>,
    completed_at_override: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    deadline: Option<DateTime<Utc>>,
    description: String,
    entity_type: String,
    epic_id: Option<u32>,
    estimate: Option<u32>,
    external_id: Option<String>,
    follower_ids: Vec<Uuid>,
    id: u32,
    labels: Vec<Label>,
    mention_ids: Vec<Uuid>,
    moved_at: Option<DateTime<Utc>>,
    name: String,
    owner_ids: Vec<Uuid>,
    position: u32,
    project_id: u32,
    requested_by_id: Uuid,
    started: bool,
    started_at: Option<DateTime<Utc>>,
    started_at_override: Option<DateTime<Utc>>,
    story_links: Vec<TypedStoryLink>,
    story_type: String,
    updated_at: Option<DateTime<Utc>>,
    workflow_state_id: u32,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct StorySlim {
    app_url: String,
    archived: bool,
    blocked: bool,
    blocker: bool,
    comment_ids: Vec<u32>,
    completed: bool,
    completed_at: Option<DateTime<Utc>>,
    completed_at_override: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    deadline: Option<DateTime<Utc>>,
    entity_type: String,
    epic_id: Option<u32>,
    estimate: Option<u32>,
    external_id: Option<String>,
    file_ids: Vec<u32>,
    follower_ids: Vec<Uuid>,
    id: u32,
    labels: Vec<Label>,
    linked_file_ids: Vec<u32>,
    mention_ids: Vec<Uuid>,
    moved_at: Option<DateTime<Utc>>,
    name: String,
    owner_ids: Vec<Uuid>,
    position: u32,
    project_id: u32,
    requested_by_id: Uuid,
    started: bool,
    started_at: Option<DateTime<Utc>>,
    started_at_override: Option<DateTime<Utc>>,
    story_links: Vec<TypedStoryLink>,
    story_type: String,
    task_ids: Vec<u32>,
    updated_at: Option<DateTime<Utc>>,
    workflow_state_id: u32,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Task {
    complete: bool,
    completed_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    description: String,
    entity_type: String,
    external_id: Option<String>,
    id: u32,
    mention_ids: Vec<Uuid>,
    owner_ids: Vec<Uuid>,
    position: u32,
    story_id: u32,
    updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Team {
    created_at: DateTime<Utc>,
    description: String,
    entity_type: String,
    id: u32,
    name: String,
    position: u32,
    project_ids: Vec<u32>,
    updated_at: DateTime<Utc>,
    workflow: Workflow,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct ThreadedComment {
    author_id: Uuid,
    comments: Vec<ThreadedComment>,
    created_at: DateTime<Utc>,
    deleted: bool,
    entity_type: String,
    external_id: Option<String>,
    id: u32,
    mention_ids: Vec<Uuid>,
    text: String,
    updated_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct TypedStoryLink {
    created_at: DateTime<Utc>,
    entity_type: String,
    id: u32,
    object_id: u32,
    subject_id: u32,
    #[serde(rename = "type")]
    typed_story_link_type: String,
    updated_at: DateTime<Utc>,
    verb: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Workflow {
    created_at: DateTime<Utc>,
    default_state_id: u32,
    description: String,
    entity_type: String,
    id: u32,
    name: String,
    states: Vec<WorkflowState>,
    team_id: u32,
    updated_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct WorkflowState {
    color: String,
    created_at: DateTime<Utc>,
    description: String,
    entity_type: String,
    id: u32,
    name: String,
    num_stories: u32,
    position: u32,
    #[serde(rename = "type")]
    workflow_state_type: String,
    updated_at: DateTime<Utc>,
    verb: Option<String>,
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
    color: Option<String>,
    external_id: Option<String>,
    name: String,
    category_type: CategoryType,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct UpdateCategory {
    archived: Option<bool>,
    color: Option<String>,
    name: Option<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateEpic {
    completed_at_override: Option<DateTime<Utc>>,
    created_at: Option<DateTime<Utc>>,
    deadline: Option<DateTime<Utc>>,
    description: Option<String>,
    epic_state_id: Option<u32>,
    external_id: Option<String>,
    follower_ids: Option<Vec<Uuid>>,
    labels: Option<Vec<CreateLabelParams>>,
    milestone_id: Option<u32>,
    name: String,
    owner_ids: Option<Vec<Uuid>>,
    requested_by_id: Option<Uuid>,
    started_at_override: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct UpdateEpic {
    after_id: Option<u32>,
    archived: Option<bool>,
    before_id: Option<u32>,
    completed_at_override: Option<DateTime<Utc>>,
    deadline: Option<DateTime<Utc>>,
    description: Option<String>,
    epic_state_id: Option<u32>,
    follower_ids: Option<Vec<Uuid>>,
    labels: Option<Vec<CreateLabelParams>>,
    milestone_id: Option<u32>,
    name: Option<String>,
    owner_ids: Option<Vec<Uuid>>,
    requested_by_id: Option<Uuid>,
    started_at_override: Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateEpicComment {
    author_id: Option<Uuid>,
    created_at: Option<DateTime<Utc>>,
    external_id: Option<String>,
    text: String,
    updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateEpicCommentComment {
    external_id: Option<String>,
    text: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct UpdateEpicComment {
    text: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct UpdateFile {
    created_at: Option<DateTime<Utc>>,
    description: Option<String>,
    external_id: Option<String>,
    name: Option<String>,
    updated_at: Option<DateTime<Utc>>,
    uploader_id: Option<Uuid>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateLabel {
    color: Option<String>,
    external_id: Option<String>,
    name: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct UpdateLabel {
    archived: Option<bool>,
    color: Option<String>,
    name: Option<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateLinkedFile {
    content_type: Option<String>,
    description: Option<String>,
    name: String,
    size: Option<u32>,
    story_id: Option<u32>,
    thumbnail_url: Option<String>,
    file_integration_type: FileIntegrationType,
    uploader_id: Option<Uuid>,
    url: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct UpdateLinkedFile {
    description: Option<String>,
    name: Option<String>,
    size: Option<String>,
    thumbnail_url: Option<String>,
    file_integration_type: Option<FileIntegrationType>,
    uploader_id: Option<Uuid>,
    url: Option<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateMilestone {
    categories: Option<Vec<CreateCategoryParams>>,
    completed_at_override: Option<DateTime<Utc>>,
    description: Option<String>,
    name: String,
    started_at_override: Option<DateTime<Utc>>,
    state: Option<StateType>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct UpdateMilestone {
    after_id: Option<u32>,
    before_id: Option<u32>,
    categories: Option<Vec<CreateCategoryParams>>,
    completed_at_override: Option<DateTime<Utc>>,
    description: Option<String>,
    name: Option<String>,
    started_at_override: Option<DateTime<Utc>>,
    state: Option<StateType>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateProject {
    abbreviation: Option<String>,
    color: Option<String>,
    created_at: Option<DateTime<Utc>>,
    description: Option<String>,
    external_id: Option<String>,
    follower_ids: Option<Vec<Uuid>>,
    iteration_length: Option<u32>,
    name: String,
    team_id: Option<u32>,
    updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct UpdateProject {
    abbreviation: Option<String>,
    archived: Option<bool>,
    color: Option<String>,
    days_to_thermometer: Option<u32>,
    description: Option<String>,
    follower_ids: Option<Vec<Uuid>>,
    name: Option<String>,
    show_thermometer: Option<bool>,
    team_id: Option<u32>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct SearchStories {
    page_size: Option<u32>,
    query: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateStory {
    comments: Option<Vec<CreateCommentParams>>,
    completed_at_override: Option<DateTime<Utc>>,
    created_at: Option<DateTime<Utc>>,
    deadline: Option<DateTime<Utc>>,
    description: Option<String>,
    epic_id: Option<u32>,
    estimate: Option<u32>,
    external_id: Option<String>,
    file_ids: Option<Vec<u32>>,
    follower_ids: Option<Vec<Uuid>>,
    labels: Option<Vec<CreateLabelParams>>,
    linked_file_ids: Option<Vec<u32>>,
    name: String,
    owner_ids: Option<Vec<Uuid>>,
    project_id: u32,
    requested_by_id: Option<Uuid>,
    started_at_override: Option<DateTime<Utc>>,
    story_links: Option<Vec<CreateStoryLinkParams>>,
    story_type: Option<StoryType>,
    tasks: Option<Vec<CreateTaskParams>>,
    updated_at: Option<DateTime<Utc>>,
    workflow_state_id: Option<u32>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateMultipleStories {
    stories: Vec<CreateStoryParams>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct UpdateMultipleStories {
    after_id: Option<u32>,
    archived: Option<bool>,
    deadline: Option<DateTime<Utc>>,
    epic_id: Option<u32>,
    estimate: Option<u32>,
    follower_ids_add: Option<Vec<Uuid>>,
    follower_ids_remove: Option<Vec<Uuid>>,
    labels_add: Option<Vec<CreateLabelParams>>,
    labels_remove: Option<Vec<CreateLabelParams>>,
    owner_ids_add: Option<Vec<Uuid>>,
    owner_ids_remove: Option<Vec<Uuid>>,
    project_id: Option<u32>,
    requested_by_id: Option<Uuid>,
    story_ids: Vec<u32>,
    story_type: Option<StoryType>,
    workflow_state_id: Option<u32>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct DeleteMultipleStories {
    story_ids: Vec<u32>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct UpdateStory {
    after_id: Option<u32>,
    archived: Option<bool>,
    before_id: Option<u32>,
    branch_ids: Option<Vec<u32>>,
    commit_ids: Option<Vec<u32>>,
    completed_at_override: Option<DateTime<Utc>>,
    deadline: Option<DateTime<Utc>>,
    description: Option<String>,
    epic_id: Option<u32>,
    estimate: Option<u32>,
    file_ids: Option<Vec<u32>>,
    follower_ids: Option<Vec<Uuid>>,
    labels: Option<Vec<CreateLabelParams>>,
    linked_file_ids: Option<Vec<u32>>,
    name: Option<String>,
    owner_ids: Option<Vec<Uuid>>,
    project_id: Option<u32>,
    requested_by_id: Option<Uuid>,
    started_at_override: Option<DateTime<Utc>>,
    story_type: Option<StoryType>,
    workflow_state_id: Option<u32>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateComment {
    author_id: Option<Uuid>,
    created_at: Option<DateTime<Utc>>,
    external_id: Option<String>,
    text: String,
    updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct UpdateComment {
    text: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateTask {
    complete: Option<bool>,
    created_at: Option<DateTime<Utc>>,
    description: String,
    external_id: Option<String>,
    owner_ids: Option<Vec<Uuid>>,
    updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct UpdateTask {
    after_id: Option<u32>,
    before_id: Option<u32>,
    complete: Option<bool>,
    description: Option<String>,
    owner_ids: Option<Vec<Uuid>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateStoryLink {
    object_id: u32,
    subject_id: u32,
    verb: StoryLinkType,
}
