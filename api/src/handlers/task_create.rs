use std::convert::Infallible;
use taskboard_core_lib::commands::CreateTaskCommand;
use warp::{hyper::StatusCode, Reply};

pub async fn handle_task_create(command: CreateTaskCommand) -> Result<impl Reply, Infallible> {
    info!("Pretending to handle {:?}", command);
    Ok(StatusCode::CREATED)
}
