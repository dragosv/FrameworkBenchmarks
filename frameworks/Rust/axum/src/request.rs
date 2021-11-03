use std::sync::atomic::{AtomicI32, Ordering};
use axum::extract::{Extension, FromRequest, RequestParts};
use axum::http;
use axum::http::{HeaderValue, StatusCode};

/// A global atomic counter for generating IDs.
static ID_COUNTER: AtomicI32 = AtomicI32::new(1);

/// A type that represents a request's ID.
#[derive(Hash, Eq, PartialEq, Serialize, Deserialize, Debug, Clone, Copy)]
pub struct RequestId(pub i32);

#[async_trait]
impl<B> FromRequest<B> for RequestId
    where B: Send,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let request_id = RequestId::new();

        return Ok(request_id);
    }
}

impl RequestId {
    fn new() -> Self {
        Self(ID_COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}