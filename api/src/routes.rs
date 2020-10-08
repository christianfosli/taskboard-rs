use warp::{Filter, Rejection, Reply};

use crate::{
    handlers::{
        health::handle_health, task_create::handle_task_create, task_get::handle_task_get,
        task_list::handle_task_list, task_update::handle_task_update,
    },
    store::with_store,
    store::TaskStore,
};

pub fn health_check_route<T: TaskStore + Clone + Sync + Send>(
    store: &T,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("healthz")
        .and(with_store(store.clone()))
        .and_then(handle_health)
}

pub fn task_routes<T: TaskStore + Clone + Sync + Send>(
    store: &T,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let get = warp::path!("task" / String)
        .and(warp::get())
        .and_then(handle_task_get);

    let create = warp::path!("task")
        .and(warp::post())
        .and(with_store(store.clone()))
        .and(warp::body::json())
        .and_then(handle_task_create);

    let update = warp::path!("task")
        .and(warp::put())
        .and(warp::body::json())
        .and_then(handle_task_update);

    let get_for_project = warp::path!("project-tasks" / String)
        .and(warp::get())
        .and(with_store(store.clone()))
        .and_then(|id, store| handle_task_list(store, id));

    get.or(create).or(update).or(get_for_project)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{anyhow, Error};
    use async_trait::async_trait;
    use taskboard_core_lib::{
        commands::CreateTaskCommand, commands::UpdateTaskCommand, uuid::Uuid, Status, Task,
    };
    use warp::hyper::StatusCode;

    #[derive(Debug, Clone)]
    struct MockTaskStore {
        success: bool,
    }

    #[async_trait]
    impl TaskStore for MockTaskStore {
        async fn ping(&self) -> Result<(), Error> {
            match self.success {
                true => Ok(()),
                false => Err(anyhow!("MockTaskStoreError: Ping failed")),
            }
        }
        async fn fetch_task(&self, _: &Uuid, number: usize) -> Result<Option<Task>, Error> {
            match self.success {
                true => Ok(Some(Task::new(number, "mock"))),
                false => Err(anyhow!("MockTaskStoreError: Could not fetch")),
            }
        }
        async fn fetch_tasks(&self, _: &Uuid) -> Result<Vec<Task>, Error> {
            match self.success {
                true => Ok(vec![Task::new(1, "mock")]),
                false => Err(anyhow!("MockTaskStoreError: Could not fetch")),
            }
        }
        async fn persist(&self, _: &Uuid, _: &Task) -> Result<(), Error> {
            match self.success {
                true => Ok(()),
                false => Err(anyhow!("MockTaskStoreError: Could not persist")),
            }
        }
    }

    #[tokio::test]
    async fn health_check_given_taskstore_up_should_be_ok() {
        let store = MockTaskStore { success: true };
        let route = health_check_route(&store);

        let res = warp::test::request()
            .method("GET")
            .path("/healthz")
            .reply(&route)
            .await;

        assert!(res.status().is_success());
        assert_eq!("OK", res.body())
    }

    #[tokio::test]
    async fn health_check_given_taskstore_down_should_be_degraded() {
        let store = MockTaskStore { success: false };
        let route = health_check_route(&store);

        let res = warp::test::request()
            .method("GET")
            .path("/healthz")
            .reply(&route)
            .await;

        assert!(res.status().is_success());

        let response = String::from_utf8(res.body().to_vec()).unwrap();
        assert!(response.contains("Degraded"));
    }

    #[tokio::test]
    async fn get_task_should_be_ok() {
        let store = MockTaskStore { success: true };
        let routes = task_routes(&store);

        let res = warp::test::request()
            .method("GET")
            .path("/task/some-dummy-id")
            .reply(&routes)
            .await;

        assert_eq!(StatusCode::OK, res.status());
    }

    #[tokio::test]
    async fn create_task_should_create() {
        let store = MockTaskStore { success: true };
        let routes = task_routes(&store);

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
        let store = MockTaskStore { success: true };
        let routes = task_routes(&store);

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
        let store = MockTaskStore { success: true };
        let routes = task_routes(&store);

        let some_project_id = Uuid::new_v4();

        let res = warp::test::request()
            .method("GET")
            .path(&format!("/project-tasks/{}", some_project_id))
            .reply(&routes)
            .await;

        assert_eq!(StatusCode::OK, res.status());
    }
}
