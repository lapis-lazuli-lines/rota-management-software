use axum::{
    body::Body,
    http::{Request, Response, HeaderValue},
    middleware::Next,
};
use uuid::Uuid;

pub async fn request_id_middleware(
    mut request: Request<Body>,
    next: Next,
) -> Response<Body> {
    // Generate a UUID for the request
    let request_id = Uuid::new_v4().to_string();
    
    // Add it as an extension to the request
    request.extensions_mut().insert(RequestId(request_id.clone()));
    
    // Log the request with its ID
    tracing::info!(request_id = %request_id, "Incoming request");
    
    // Pass the request to the next middleware or handler
    let mut response = next.run(request).await;
    
    // Add the request ID to the response headers
    if let Ok(header_value) = HeaderValue::from_str(&request_id) {
        response.headers_mut().insert("X-Request-ID", header_value);
    }
    
    response
}

// A newtype to hold the request ID
#[derive(Clone, Debug)]
pub struct RequestId(pub String);