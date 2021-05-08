use std::convert::Infallible;

use reqwest::StatusCode;
use taskboard_core_lib::ErrorMessage;
use warp::body::BodyDeserializeError;

#[derive(Clone, Debug)]
pub struct ValidationError {
    pub reason: String,
}

/// Maps exceptions to a status code
/// Also fix for CORS on rejected requests (warp issue #518)
pub async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
    tracing::error!(?err);

    let status_code = if err.is_not_found() {
        StatusCode::NOT_FOUND
    } else if err.find::<BodyDeserializeError>().is_some() {
        StatusCode::BAD_REQUEST
    } else if err.find::<ValidationError>().is_some() {
        StatusCode::UNPROCESSABLE_ENTITY
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    };

    let body = warp::reply::json(&ErrorMessage {
        message: format!("{:?}", err),
        status_code: status_code.as_u16(),
    });

    Ok(warp::reply::with_status(body, status_code))
}
