use std::env;
use std::future::Future;

use taskboard_core_lib::{commands::CreateTaskCommand, uuid::Uuid, Project, Status, Task};
use warp::{
    hyper::StatusCode,
    reject,
    reply::{self, with_status},
    Rejection, Reply,
};

use crate::errors::PersistError;
use crate::store::TaskStore;

pub async fn handle_task_create<Fut>(
    store: impl TaskStore,
    claim_task_number: impl FnOnce(Uuid) -> Fut,
    command: CreateTaskCommand,
) -> Result<impl Reply, Rejection>
where
    Fut: Future<Output = Result<usize, anyhow::Error>>,
{
    info!("Create Task: {:?}", command);

    let number = claim_task_number(command.project_id).await.map_err(|e| {
        error!("Unable to claim task number: {:?}", e);
        reject::custom(PersistError {
            reason: String::from("Unable to claim task number"),
        })
    })?;

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
            error!("Unable to persist task {:?}: {:?}", task, e);
            reject::custom(PersistError {
                reason: String::from("Unable to persist to store"),
            })
        })?;

    info!("Task {} created successfully", number);

    Ok(with_status(reply::json(&task), StatusCode::CREATED))
}

pub async fn claim_task_number(project_id: Uuid) -> Result<usize, anyhow::Error> {
    let url = format!(
        "{}/{}/increment-counter",
        env::var("PROJECT_SERVICE_URL")?,
        project_id
    );

    let response = reqwest::Client::new()
        .post(&url)
        .send()
        .await?
        .json::<Project>()
        .await?;

    Ok(response.task_conter)
}
