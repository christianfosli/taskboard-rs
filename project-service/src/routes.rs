use warp::{Filter, Rejection, Reply};

use crate::{
    handlers::create_project::handle_create_project,
    handlers::delete_project::handle_delete_project, handlers::get_project::handle_get_project,
    handlers::health::handle_health, handlers::increment_counter::handle_increment_counter,
    handlers::search_project::handle_search_project, services::task_service::with_task_service,
    services::task_service::ITaskService, store::with_store, store::ProjectStore,
};

pub fn health_check_route<T: ProjectStore + Clone + Sync + Send>(
    store: &T,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("healthz")
        .and(with_store(store.clone()))
        .and_then(handle_health)
}

pub fn project_routes<TStore, TTaskService>(
    store: &TStore,
    task_service: &TTaskService,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    TStore: ProjectStore + Clone + Sync + Send,
    TTaskService: ITaskService + Clone + Sync + Send,
{
    let get = warp::path!(String)
        .and(warp::get())
        .and(with_store(store.clone()))
        .and_then(|id, store| handle_get_project(store, id));

    let search = warp::path!("search" / String)
        .and(warp::get())
        .and(with_store(store.clone()))
        .and_then(|searchquery, store| handle_search_project(store, searchquery));

    let create = warp::post()
        .and(with_store(store.clone()))
        .and(warp::body::json())
        .and_then(handle_create_project);

    let delete = warp::path!(String)
        .and(warp::delete())
        .and(with_store(store.clone()))
        .and(with_task_service(task_service.clone()))
        .and_then(|id, store, task_service| handle_delete_project(store, task_service, id));

    let increment_counter = warp::path!(String / "increment-counter")
        .and(warp::post())
        .and(with_store(store.clone()))
        .and_then(|id, store| handle_increment_counter(store, id));

    get.or(search).or(create).or(delete).or(increment_counter)
}
