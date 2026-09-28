use executors::profile::ExecutorConfig;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use super::{execution_process::ExecutionProcess, workspace::Workspace};

#[derive(Debug, Deserialize, Serialize)]
pub struct ContainerQuery {
    #[serde(rename = "ref")]
    pub container_ref: String,
}

#[derive(Debug, Serialize, Deserialize, TS)]
pub struct WorkspaceRepoInput {
    pub repo_id: Uuid,
    pub target_branch: String,
}

#[derive(Debug, Serialize, Deserialize, TS)]
pub struct CreateWorkspaceApiRequest {
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
pub struct LinkedIssueInfo {
    pub remote_project_id: Uuid,
    pub issue_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, TS)]
pub struct WorkspaceFileOverlay {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, TS)]
pub struct WorkspaceManagedBlockOverlay {
    pub path: String,
    pub marker: String,
    pub content: String,
}

#[derive(Debug, Default, Serialize, Deserialize, TS)]
pub struct WorkspaceOverlay {
    #[serde(default)]
    pub files: Vec<WorkspaceFileOverlay>,
    #[serde(default)]
    pub managed_blocks: Vec<WorkspaceManagedBlockOverlay>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
pub struct CreateAndStartWorkspaceRequest {
    pub workspace_id: Option<Uuid>,
    pub name: Option<String>,
    pub repos: Vec<WorkspaceRepoInput>,
    pub linked_issue: Option<LinkedIssueInfo>,
    pub executor_config: ExecutorConfig,
    pub prompt: String,
    pub attachment_ids: Option<Vec<Uuid>>,
    pub workspace_overlay: Option<WorkspaceOverlay>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
pub struct CreateAndStartWorkspaceResponse {
    pub workspace: Workspace,
    pub execution_process: ExecutionProcess,
}

#[derive(Debug, Serialize, Deserialize, TS)]
pub struct UpdateWorkspace {
    pub archived: Option<bool>,
    pub pinned: Option<bool>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
pub struct UpdateSession {
    pub name: Option<String>,
}
