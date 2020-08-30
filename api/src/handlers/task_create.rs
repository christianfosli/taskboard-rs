use std::convert::Infallible;
use taskboard_core_lib::Task;
use warp::{hyper::StatusCode, Reply};

pub async fn handle_task_create(task: Task) -> Result<impl Reply, Infallible> {
    info!("Pretending that task {:?} was created", task);
    Ok(StatusCode::CREATED)
}
