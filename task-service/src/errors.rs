use warp::reject::Reject;

#[derive(Clone, Debug)]
pub struct PersistError {
    reason: String,
}

#[derive(Clone, Debug)]
pub struct FetchError {
    reason: String,
}

#[derive(Clone, Debug)]
pub struct ValidationError {
    reason: String,
}

impl Reject for PersistError {}
impl Reject for FetchError {}
impl Reject for ValidationError {}
