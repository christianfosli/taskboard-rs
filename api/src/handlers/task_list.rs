use std::convert::Infallible;
use taskboard_core_lib::{ProjectTasks, Task};
use warp::Reply;

pub async fn handle_task_list(project_id: String) -> Result<impl Reply, Infallible> {
    let project_name = String::from("Dummy Project");
    info!(
        "Pretending to retrive tasks associated to project {} {}",
        project_id, project_name
    );
    let tasks = ProjectTasks {
        project_name,
        tasks: vec![
            Task::new(1, "dummy1"),
            Task::new(2, "dummy2"),
            Task {
                remaining_work: Some(5),
                ..Task::new(3, "dummy3")
            },
        ],
    };
    Ok(warp::reply::json(&tasks))
}
