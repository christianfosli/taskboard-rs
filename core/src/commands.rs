#[deny(missing_docs)]
use crate::task::Task;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Command for adding a new task to an existing project
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateTaskCommand {
    /// Id for the project to add the task to
    pub project_id: Uuid,
    /// Title of the task to create
    pub title: String,
    /// The number of hours the task is estimated to take
    pub estimate: Option<u8>,
}

/// Command for updating an existing task
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateTaskCommand {
    /// Id for the project that the task is a part of
    pub project_id: Uuid,
    /// The updated task
    pub updated_task: Task,
}
