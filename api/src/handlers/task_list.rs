use std::convert::Infallible;
use taskboard_core_lib::Task;
use warp::Reply;

pub async fn handle_task_list(project_id: String) -> Result<impl Reply, Infallible> {
    info!(
        "Pretending to retrive tasks associated to project {}",
        project_id
    );
    let tasks = vec![
        Task::new("dummy1"),
        Task::new("dummy2"),
        Task::new("dummy3"),
    ];
    Ok(warp::reply::json(&tasks))
}
