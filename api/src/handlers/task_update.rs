use taskboard_core_lib::Task;
use warp::{hyper::StatusCode, Rejection, Reply};

pub async fn handle_task_update(updated_task: Task) -> Result<impl Reply, Rejection> {
    info!("Pretending to update {:?}", updated_task);
    Ok(StatusCode::OK)
}
