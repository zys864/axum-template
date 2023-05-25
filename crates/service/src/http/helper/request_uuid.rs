use axum::{http::Request};

use tower_http::request_id::{MakeRequestId, RequestId};

#[derive(Clone)]
pub struct RequestUuid;

impl MakeRequestId for RequestUuid {
    fn make_request_id<B>(&mut self, _request: &Request<B>) -> Option<RequestId> {


        Some(RequestId::new(
            uuid::Uuid::new_v4().to_string().parse().unwrap(),
        ))
    }
}
