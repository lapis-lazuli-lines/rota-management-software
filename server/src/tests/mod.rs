#[cfg(test)]
mod api_tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        Router,
    };
    use serde_json::{json, Value};
    use tower::ServiceExt;

    use crate::routes::app_routes;

    // Helper function to create a test app router
    fn app() -> Router {
        app_routes()
    }

    #[tokio::test]
    async fn test_health_check() {
        // Arrange
        let app = app();

        // Create a request to /health
        let request = Request::builder()
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        // Act
        let response = app.oneshot(request).await.unwrap();

        // Assert
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_root() {
        // Arrange
        let app = app();

        // Create a request to /
        let request = Request::builder()
            .uri("/")
            .body(Body::empty())
            .unwrap();

        // Act
        let response = app.oneshot(request).await.unwrap();

        // Assert
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_create_user() {
        // Arrange
        let app = app();

        // Create a request to create a user
        let request = Request::builder()
            .uri("/users")
            .method("POST")
            .header("Content-Type", "application/json")
            .body(Body::from(json!({
                "name": "Test User",
                "email": "test@example.com"
            }).to_string()))
            .unwrap();

        // Act
        let response = app.oneshot(request).await.unwrap();

        // Assert
        assert_eq!(response.status(), StatusCode::CREATED);

        // Check the response body
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body["name"], "Test User");
        assert_eq!(body["email"], "test@example.com");
        assert!(body["id"].is_number());
    }

    #[tokio::test]
    async fn test_get_nonexistent_user() {
        // Arrange
        let app = app();

        // Create a request to get a user that doesn't exist
        let request = Request::builder()
            .uri("/users/999")
            .body(Body::empty())
            .unwrap();

        // Act
        let response = app.oneshot(request).await.unwrap();

        // Assert - should be not found
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_and_get_user() {
        // Arrange
        let app = app();

        // Create a user first
        let create_request = Request::builder()
            .uri("/users")
            .method("POST")
            .header("Content-Type", "application/json")
            .body(Body::from(json!({
                "name": "Jane Doe",
                "email": "jane@example.com"
            }).to_string()))
            .unwrap();

        let create_response = app.clone().oneshot(create_request).await.unwrap();
        assert_eq!(create_response.status(), StatusCode::CREATED);
        
        let body = hyper::body::to_bytes(create_response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        let user_id = body["id"].as_i64().unwrap();

        // Now get the user
        let get_request = Request::builder()
            .uri(&format!("/users/{}", user_id))
            .body(Body::empty())
            .unwrap();

        // Act
        let get_response = app.oneshot(get_request).await.unwrap();

        // Assert
        assert_eq!(get_response.status(), StatusCode::OK);
        
        let body = hyper::body::to_bytes(get_response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        
        assert_eq!(body["id"], user_id);
        assert_eq!(body["name"], "Jane Doe");
        assert_eq!(body["email"], "jane@example.com");
    }

    #[tokio::test]
    async fn test_invalid_email_validation() {
        // Arrange
        let app = app();

        // Create a request with an invalid email
        let request = Request::builder()
            .uri("/users")
            .method("POST")
            .header("Content-Type", "application/json")
            .body(Body::from(json!({
                "name": "Invalid User",
                "email": "not-an-email"
            }).to_string()))
            .unwrap();

        // Act
        let response = app.oneshot(request).await.unwrap();

        // Assert - should be bad request
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        
        // Check error message contains info about invalid email
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        
        assert!(body["error"].as_str().unwrap().contains("email"));
    }
}