use crate::handlers::{
    health::handle_health, task_create::handle_task_create, task_get::handle_task_get,
    task_list::handle_task_list, task_update::handle_task_update,
};
use warp::{Filter, Rejection, Reply};

pub fn health_check_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("healthz").and_then(handle_health)
}

pub fn task_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let get = warp::path!("task" / String)
        .and(warp::get())
        .and_then(handle_task_get);

    let create = warp::path!("task")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_task_create);

    let update = warp::path!("task")
        .and(warp::put())
        .and(warp::body::json())
        .and_then(handle_task_update);

    let get_for_project = warp::path!("project-tasks" / String)
        .and(warp::get())
        .and_then(handle_task_list);

    get.or(create).or(update).or(get_for_project)
}

#[cfg(test)]
mod tests {
    use super::*;
    use taskboard_core_lib::{
        commands::CreateTaskCommand, commands::UpdateTaskCommand, uuid::Uuid, Status, Task,
    };
    use warp::hyper::StatusCode;

    #[tokio::test]
    async fn health_check_should_be_ok() {
        let route = health_check_route();

        let res = warp::test::request()
            .method("GET")
            .path("/healthz")
            .reply(&route)
            .await;

        assert_eq!(StatusCode::OK, res.status());
    }

    #[tokio::test]
    async fn get_task_should_be_ok() {
        let routes = task_routes();

        let res = warp::test::request()
            .method("GET")
            .path("/task/some-dummy-id")
            .reply(&routes)
            .await;

        assert_eq!(StatusCode::OK, res.status());
    }

    #[tokio::test]
    async fn create_task_should_create() {
        let routes = task_routes();

        let res = warp::test::request()
            .method("POST")
            .path("/task")
            .json(&CreateTaskCommand {
                title: "created test-task".into(),
                project_id: Uuid::new_v4(),
                estimate: None,
            })
            .reply(&routes)
            .await;

        assert_eq!(StatusCode::CREATED, res.status());
    }

    #[tokio::test]
    async fn update_task_should_be_ok() {
        let routes = task_routes();

        let res = warp::test::request()
            .method("PUT")
            .path("/task")
            .json(&UpdateTaskCommand {
                project_id: Uuid::new_v4(),
                updated_task: Task {
                    number: 6,
                    title: "updated test-task".into(),
                    status: Status::Doing,
                    remaining_work: Some(5),
                },
            })
            .reply(&routes)
            .await;

        assert_eq!(StatusCode::OK, res.status());
    }

    #[tokio::test]
    async fn get_for_project_should_be_ok() {
        let routes = task_routes();

        let res = warp::test::request()
            .method("GET")
            .path("/project-tasks/some-project-id")
            .reply(&routes)
            .await;

        assert_eq!(StatusCode::OK, res.status());
    }
}
