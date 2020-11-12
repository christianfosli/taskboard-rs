use warp::reject::Reject;

#[derive(Clone, Debug)]
pub struct PersistError {
    pub reason: String,
}

#[derive(Clone, Debug)]
pub struct FetchError {
    pub reason: String,
}

#[derive(Clone, Debug)]
pub struct ValidationError {
    pub reason: String,
}

impl Reject for PersistError {}
impl Reject for FetchError {}
impl Reject for ValidationError {}
