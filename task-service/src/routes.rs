use warp::{Filter, Rejection, Reply};

use crate::{
    handlers::health::{handle_liveness, handle_readiness},
    handlers::task_completed::handle_task_completed,
    handlers::task_create::handle_task_create,
    handlers::task_delete::handle_task_delete,
    handlers::task_list::handle_task_list,
    handlers::task_started::handle_task_started,
    handlers::task_update::handle_task_update,
    services::project_service::{with_project_service, IProjectService},
    store::with_store,
    store::TaskStore,
};

pub fn health_routes<TStore>(
    store: &TStore,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    TStore: TaskStore + Clone + Sync + Send,
{
    let liveness = warp::path!("livez").map(handle_liveness);

    let readiness = warp::path!("readyz")
        .and(with_store(store.clone()))
        .and_then(handle_readiness);

    liveness.or(readiness)
}

pub fn task_routes<TStore, TProjectService>(
    store: &TStore,
    project_service: &TProjectService,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    TStore: TaskStore + Clone + Sync + Send,
    TProjectService: IProjectService + Clone + Sync + Send,
{
    let get = warp::path!("project-tasks" / String)
        .and(warp::get())
        .and(with_store(store.clone()))
        .and(with_project_service(project_service.clone()))
        .and_then(|id, store, project_service| handle_task_list(store, project_service, id));

    let create = warp::path!("task" / "create")
        .and(warp::post())
        .and(with_store(store.clone()))
        .and(with_project_service(project_service.clone()))
        .and(warp::body::json())
        .and_then(handle_task_create);

    let update = warp::path!("task" / "update")
        .and(warp::put())
        .and(with_store(store.clone()))
        .and(warp::body::json())
        .and_then(handle_task_update);

    let delete = warp::path!("project-tasks" / String)
        .and(warp::delete())
        .and(with_store(store.clone()))
        .and_then(|id, store| handle_task_delete(store, id));

    let start = warp::path!("task" / "start")
        .and(warp::post())
        .and(with_store(store.clone()))
        .and(warp::body::json())
        .and_then(handle_task_started);

    let complete = warp::path!("task" / "complete")
        .and(warp::post())
        .and(with_store(store.clone()))
        .and(warp::body::json())
        .and_then(handle_task_completed);

    get.or(create).or(update).or(delete).or(start).or(complete)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{anyhow, Error};
    use async_trait::async_trait;
    use taskboard_core_lib::{
        commands::CreateTaskCommand,
        commands::{CompleteTaskCommand, StartTaskCommand, UpdateTaskCommand},
        uuid::Uuid,
        Project, Status, Task,
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
        async fn fetch_tasks(&self, _: &Uuid) -> Result<Vec<Task>, Error> {
            match self.success {
                true => Ok(vec![Task::new(1, "mock")]),
                false => Err(anyhow!("MockTaskStoreError: Could not fetch")),
            }
        }
        async fn get(&self, _: &Uuid, _: usize) -> Result<Option<Task>, Error> {
            match self.success {
                true => Ok(Some(Task::new(1, "mock"))),
                false => Err(anyhow!("MockTaskStoreError: Could not get")),
            }
        }
        async fn persist(&self, _: &Uuid, _: &Task) -> Result<(), Error> {
            match self.success {
                true => Ok(()),
                false => Err(anyhow!("MockTaskStoreError: Could not persist")),
            }
        }
        async fn delete(&self, _: &Uuid) -> Result<(), Error> {
            match self.success {
                true => Ok(()),
                false => Err(anyhow!("MockTaskStoreError: Could not delete")),
            }
        }
    }

    #[derive(Debug, Clone)]
    struct MockProjectService {}

    #[async_trait]
    impl IProjectService for MockProjectService {
        async fn get_project(&self, _: &Uuid) -> Result<taskboard_core_lib::Project, Error> {
            Ok(Project::new("Mock project"))
        }

        async fn claim_task_number(&self, _: &Uuid) -> Result<usize, Error> {
            Ok(3)
        }
    }

    #[tokio::test]
    async fn liveness_should_be_ok() {
        let store = MockTaskStore { success: true };
        let route = health_routes(&store);

        let res = warp::test::request()
            .method("GET")
            .path("/livez")
            .reply(&route)
            .await;

        assert!(res.status().is_success());
    }

    #[tokio::test]
    async fn readiness_given_store_up_should_be_ok() {
        let store = MockTaskStore { success: true };
        let route = health_routes(&store);

        let res = warp::test::request()
            .method("GET")
            .path("/readyz")
            .reply(&route)
            .await;

        assert!(res.status().is_success());

        let response = String::from_utf8(res.body().to_vec()).unwrap();
        assert!(response.contains("OK"));
    }

    #[tokio::test]
    async fn readiness_given_store_down_should_not_be_ok() {
        let store = MockTaskStore { success: false };
        let route = health_routes(&store);

        let res = warp::test::request()
            .method("GET")
            .path("/readyz")
            .reply(&route)
            .await;

        assert!(!res.status().is_success());
    }

    #[tokio::test]
    async fn create_task_should_create() {
        let store = MockTaskStore { success: true };
        let project_service = MockProjectService {};
        let routes = task_routes(&store, &project_service);

        let res = warp::test::request()
            .method("POST")
            .path("/task/create")
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
        let project_service = MockProjectService {};
        let routes = task_routes(&store, &project_service);

        let res = warp::test::request()
            .method("PUT")
            .path("/task/update")
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
    async fn delete_project_tasks_should_be_ok() {
        let store = MockTaskStore { success: true };
        let project_service = MockProjectService {};
        let routes = task_routes(&store, &project_service);

        let res = warp::test::request()
            .method("DELETE")
            .path(&format!("/project-tasks/{}", Uuid::new_v4()))
            .reply(&routes)
            .await;

        assert_eq!(StatusCode::OK, res.status());
    }

    #[tokio::test]
    async fn start_task_should_be_ok() {
        let store = MockTaskStore { success: true };
        let project_service = MockProjectService {};
        let routes = task_routes(&store, &project_service);

        let res = warp::test::request()
            .method("POST")
            .path("/task/start")
            .json(&StartTaskCommand {
                project_id: Uuid::new_v4(),
                task_number: 2,
            })
            .reply(&routes)
            .await;

        assert_eq!(StatusCode::OK, res.status());
    }

    #[tokio::test]
    async fn complete_task_should_be_ok() {
        let store = MockTaskStore { success: true };
        let project_service = MockProjectService {};
        let routes = task_routes(&store, &project_service);

        let res = warp::test::request()
            .method("POST")
            .path("/task/complete")
            .json(&CompleteTaskCommand {
                project_id: Uuid::new_v4(),
                task_number: 2,
            })
            .reply(&routes)
            .await;

        assert_eq!(StatusCode::OK, res.status());
    }

    #[tokio::test]
    async fn get_for_project_should_be_ok() {
        let store = MockTaskStore { success: true };
        let project_service = MockProjectService {};
        let routes = task_routes(&store, &project_service);

        let some_project_id = Uuid::new_v4();

        let res = warp::test::request()
            .method("GET")
            .path(&format!("/project-tasks/{}", some_project_id))
            .reply(&routes)
            .await;

        assert_eq!(StatusCode::OK, res.status());
    }
}
