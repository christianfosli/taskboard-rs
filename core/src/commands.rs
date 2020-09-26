use crate::task::Task;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Command for adding a new task to an existing project
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateTaskCommand {
    /// Id for the project to add the task to
    project_id: Uuid,
    /// Title of the task to create
    title: String,
    /// The number of hours the task is estimated to take
    estimate: Option<u8>,
}

/// Command for updating an existing task
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateTaskCommand {
    /// Id for the project that the task is a part of
    project_id: String,
    /// The updated task
    updated_task: Task,
}
