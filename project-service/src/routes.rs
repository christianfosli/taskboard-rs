use warp::{Filter, Rejection, Reply};

use crate::{
    handlers::create_project::handle_create_project, handlers::get_project::handle_get_project,
    handlers::health::handle_health, store::with_store, store::ProjectStore,
};

pub fn health_check_route<T: ProjectStore + Clone + Sync + Send>(
    store: &T,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("healthz")
        .and(with_store(store.clone()))
        .and_then(handle_health)
}

pub fn project_routes<T: ProjectStore + Clone + Sync + Send>(
    store: &T,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let get = warp::path!(String)
        .and(warp::get())
        .and(with_store(store.clone()))
        .and_then(|id, store| handle_get_project(store, id));

    let create = warp::post()
        .and(with_store(store.clone()))
        .and(warp::body::json())
        .and_then(handle_create_project);

    get.or(create)
}
