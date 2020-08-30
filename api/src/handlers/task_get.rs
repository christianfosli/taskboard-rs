use std::convert::Infallible;
use taskboard_core_lib::Task;
use warp::Reply;

pub async fn handle_task_get(id: String) -> Result<impl Reply, Infallible> {
    info!("Pretending to retrieve task with id {} from ES", id);
    Ok(warp::reply::json(&Task::new("dummy")))
}
