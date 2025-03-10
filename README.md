# Axum Web API

A modern, high-performance web API built with Rust using the Axum framework and Supabase for PostgreSQL database.

## Features

-   **High Performance**: Built with Rust and Axum for maximum performance and resource efficiency
-   **REST API Architecture**: Clean endpoints following REST best practices
-   **Database Integration**: PostgreSQL support via Supabase
-   **Error Handling**: Comprehensive error system with appropriate status codes and messages
-   **Middleware Support**: Request ID tracking and logging
-   **Authentication**: Simplified auth for protected endpoints
-   **Testing**: Integrated test suite for API endpoints
-   **Environment Configuration**: Easy configuration via environment variables
-   **Cross-Platform**: Runs on Windows, macOS, and Linux

## Tech Stack

-   **Rust**: Core programming language
-   **Axum**: Web framework built on top of Tokio and Hyper
-   **SQLx**: Async SQL toolkit with compile-time query checking
-   **Supabase**: PostgreSQL database with extra features
-   **Tokio**: Async runtime
-   **Serde**: Serialization/deserialization
-   **Tower**: Middleware framework
-   **tracing/tracing-subscriber**: Logging framework
-   **dotenv**: Environment variable management

## Project Structure

```
src/
├── config.rs            # Application configuration management
├── database.rs          # Database connection handling
├── error.rs             # Error types and conversions
├── main.rs              # Application entry point
├── middleware/          # Custom middleware components
│   ├── mod.rs           # Middleware module definition
│   └── request_id.rs    # Request ID tracking middleware
├── models/              # Database models
│   ├── mod.rs           # Models module definition
│   └── user.rs          # User model and database operations
├── routes/              # API routes
│   ├── mod.rs           # Routes module definition
│   ├── db_users.rs      # Database user endpoints
│   └── users.rs         # In-memory user endpoints (demo)
└── tests/               # Test modules
    └── mod.rs           # API endpoint tests
```

## Setup and Installation

### Prerequisites

-   Rust (latest stable version)
-   Cargo (comes with Rust)
-   A Supabase account and project

### Database Setup

1. Create a new Supabase project
2. Set up the users table in the SQL Editor:

```sql
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    role VARCHAR(50) DEFAULT 'user',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
```

### Environment Configuration

Create a `.env` file in the project root:

```dotenv
DATABASE_URL=postgres://[YOUR-SUPABASE-CONNECTION-STRING]
RUST_LOG=debug
PORT=3000
HOST=127.0.0.1
ENV=development
```

Replace `[YOUR-SUPABASE-CONNECTION-STRING]` with your actual connection string from Supabase:

1. Go to your Supabase project dashboard
2. Navigate to Project Settings > Database
3. Find the "Connection string" section and copy the URI format
4. Replace the placeholder with your database password

### Building and Running

```bash
# Clone the repository
git clone [repository-url]
cd axum-web-api

# Build the project
cargo build

# Run the server
cargo run
```

The server will start on http://127.0.0.1:3000 (or the port specified in your .env file).

## API Endpoints

### Health Check

-   `GET /health` - Check if the API is running
-   Response: 200 OK

### Root

-   `GET /` - Welcome message with Request ID
-   Response: Welcome message text

### In-memory Users API (for Demo)

-   `POST /users` - Create a user
    -   Body: `{ "name": "User Name", "email": "user@example.com", "role": "user" }`
    -   Response: 201 Created with user object
-   `GET /users` - List all users
    -   Response: Array of user objects
-   `GET /users/:id` - Get specific user
    -   Response: User object or 404 Not Found
-   `GET /users/:id/admin` - Get admin details for a user
    -   Headers: `Authorization: Bearer admin-token`
    -   Response: Admin user object or 401/403/404

### Database Users API

-   `POST /db/users` - Create a user
    -   Body: `{ "name": "User Name", "email": "user@example.com", "role": "user" }`
    -   Response: 201 Created with user object
-   `GET /db/users` - List all users
    -   Response: Array of user objects
-   `GET /db/users/:id` - Get specific user
    -   Response: User object or 404 Not Found
-   `GET /db/users/:id/admin` - Get admin details for a user
    -   Headers: `Authorization: Bearer admin-token`
    -   Response: Admin user object or 401/403/404

## Error Handling

The API returns appropriate HTTP status codes and error messages:

-   `400 Bad Request` - Invalid input data
-   `401 Unauthorized` - Missing authentication
-   `403 Forbidden` - Insufficient permissions
-   `404 Not Found` - Resource not found
-   `500 Internal Server Error` - Server-side error

All error responses follow the format:

```json
{
	"error": "Error message",
	"code": 400
}
```

## Authentication

For protected endpoints (like admin-only routes), include an Authorization header:

```
Authorization: Bearer admin-token
```

This is a simplified example. In a production environment, you would implement JWT or OAuth2.

## Testing

Run the test suite with:

```bash
cargo test
```

The test suite includes:

-   Health check endpoint
-   Root endpoint
-   User creation
-   User retrieval
-   Error case validation

## Performance Considerations

-   The server uses Tokio for asynchronous processing
-   Database connections are pooled for optimal performance
-   Axum is built on Hyper, one of the fastest HTTP implementations

## Deployment

### Docker (recommended)

Create a `Dockerfile`:

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/axum-web-api /usr/local/bin/
CMD ["axum-web-api"]
```

Build and run:

```bash
docker build -t axum-web-api .
docker run -p 3000:3000 --env-file .env axum-web-api
```

### Railway, Fly.io, or similar platforms

These platforms support Rust applications natively - follow their specific deployment guides.

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Commit your changes: `git commit -am 'Add some feature'`
4. Push to the branch: `git push origin feature-name`
5. Submit a pull request

## License

MIT

## Additional Resources

-   [Axum Documentation](https://docs.rs/axum/latest/axum/)
-   [SQLx Documentation](https://docs.rs/sqlx/latest/sqlx/)
-   [Supabase Documentation](https://supabase.io/docs)
-   [Rust Book](https://doc.rust-lang.org/book/)
