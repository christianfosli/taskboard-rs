use reqwest::StatusCode;
use taskboard_core_lib::ErrorMessage;

/// Maps exceptions to a status code
/// Also fix for CORS on rejected requests (warp issue #518)
pub async fn handle_rejection(
    err: warp::Rejection,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    tracing::error!("{:?}", err);

    let status_code = if err.is_not_found() {
        StatusCode::NOT_FOUND
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    };

    let body = warp::reply::json(&ErrorMessage {
        message: format!("{:?}", err),
        status_code: status_code.as_u16(),
    });

    Ok(warp::reply::with_status(body, status_code))
}
