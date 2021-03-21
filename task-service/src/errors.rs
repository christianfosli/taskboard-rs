use std::convert::Infallible;

use reqwest::StatusCode;
use warp::{body::BodyDeserializeError, reject::Reject, Reply};

#[derive(Clone, Debug)]
pub struct PersistError {
    pub reason: String,
}

#[derive(Clone, Debug)]
pub struct FetchError {
    pub reason: String,
}

#[derive(Clone, Debug)]
pub struct DeleteError {
    pub reason: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ValidationError {
    pub reason: String,
}

impl Reject for PersistError {}
impl Reject for FetchError {}
impl Reject for DeleteError {}
impl Reject for ValidationError {}

/// Maps exceptions to a status code
/// also used to make CORS behave as it should for rejected requests
/// see https://github.com/seanmonstar/warp/issues/518
pub async fn handle_rejection(err: warp::Rejection) -> Result<impl Reply, Infallible> {
    tracing::error!("{:?}", err);

    let status_code = if err.is_not_found() {
        StatusCode::NOT_FOUND
    } else if let Some(_) = err.find::<BodyDeserializeError>() {
        StatusCode::BAD_REQUEST
    } else if let Some(_) = err.find::<ValidationError>() {
        StatusCode::UNPROCESSABLE_ENTITY
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    };

    Ok(warp::reply::with_status(format!("{:?}", err), status_code))
}
