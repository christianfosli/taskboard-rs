use warp::{cors::Builder, hyper::Method};

pub fn cors() -> Builder {
    warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["Content-Type"])
        .allow_methods(&[Method::GET, Method::POST, Method::PUT, Method::DELETE])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::routes;
    use warp::{hyper::StatusCode, Filter};

    #[tokio::test]
    async fn preflight_request_from_localhost_should_be_ok() {
        let route_with_cors = routes::health_check_route().with(cors());

        let res = warp::test::request()
            .method("OPTIONS")
            .header("origin", "http://localhost")
            .header("access-control-request-method", "POST")
            .header("access-control-request-headers", "content-type")
            .path("/healthz")
            .reply(&route_with_cors)
            .await;

        assert_eq!(StatusCode::OK, res.status());
        assert_eq!(
            "http://localhost",
            res.headers()["access-control-allow-origin"]
        );
        assert_eq!(
            "content-type",
            res.headers()["access-control-allow-headers"],
        )
    }
}
