use taskboard_core_lib::{commands::CreateTaskCommand, Status, Task};
use warp::{
    hyper::StatusCode,
    reject::{self, Reject},
    reply::{self, with_status},
    Rejection, Reply,
};

use crate::store::TaskStore;

pub async fn handle_task_create(
    store: impl TaskStore,
    command: CreateTaskCommand,
) -> Result<impl Reply, Rejection> {
    info!("Create Task: {:?}", command);

    let number = 2; // TODO: get a unique number for this project

    let task = Task {
        number,
        title: command.title,
        status: Status::Todo,
        remaining_work: command.estimate,
    };

    store
        .persist(&command.project_id, &task)
        .await
        .map_err(|e| {
            error!("Unable to create task {:?}\nErr: {:?}", task, e);
            reject::custom(TaskPersistError {})
        })?;

    info!("Task {} created successfully", number);

    Ok(with_status(reply::json(&task), StatusCode::CREATED))
}

#[derive(Clone, Debug)]
struct TaskPersistError {}
impl Reject for TaskPersistError {}
