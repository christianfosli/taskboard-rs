use taskboard_core_lib::commands::UpdateTaskCommand;
use warp::{hyper::StatusCode, Rejection, Reply};

pub async fn handle_task_update(command: UpdateTaskCommand) -> Result<impl Reply, Rejection> {
    info!("Pretending handle {:?}", command);
    Ok(StatusCode::OK)
}
