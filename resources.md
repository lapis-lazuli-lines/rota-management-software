# Learning Resources for Axum Web API Development

This document provides comprehensive learning resources to understand all the concepts used in this Axum-based web API project. Whether you're new to Rust or experienced but new to web development with Axum, these resources will help you master the technologies and patterns used.

## Table of Contents

-   [Rust Fundamentals](#rust-fundamentals)
-   [Asynchronous Programming](#asynchronous-programming)
-   [Web Development with Axum](#web-development-with-axum)
-   [Database Access with SQLx](#database-access-with-sqlx)
-   [REST API Design](#rest-api-design)
-   [Authentication & Security](#authentication--security)
-   [Testing in Rust](#testing-in-rust)
-   [Error Handling](#error-handling)
-   [Example Projects](#example-projects)
-   [Books](#books)
-   [Video Courses](#video-courses)

## Rust Fundamentals

### Official Resources

-   [The Rust Programming Language Book](https://doc.rust-lang.org/book/) - The official and comprehensive guide to Rust
-   [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn Rust through annotated examples
-   [Rustlings](https://github.com/rust-lang/rustlings) - Small exercises to get you used to reading and writing Rust code

### Additional Resources

-   [Rust for Rustaceans](https://nostarch.com/rust-rustaceans) - Intermediate Rust programming
-   [Tour of Rust](https://tourofrust.com/) - Interactive Rust learning journey
-   [Exercism Rust Track](https://exercism.org/tracks/rust) - Hands-on exercises with mentorship

## Asynchronous Programming

### Understanding Async in Rust

-   [Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/) - The official async book
-   [Tokio Tutorial](https://tokio.rs/tokio/tutorial) - Learn the Tokio runtime that powers Axum
-   [Async/Await in Rust](https://blog.rust-lang.org/2019/11/07/Async-await-stable.html) - The stabilization announcement with explanations

### Advanced Async Concepts

-   [Tokio Internals](https://tokio.rs/tokio/tutorial/async) - How Tokio works under the hood
-   [Asynchronous Programming in Rust](https://www.manning.com/books/rust-in-action) - Chapter in Rust in Action book
-   [Future Trait Documentation](https://doc.rust-lang.org/std/future/trait.Future.html) - Understanding the core of async Rust

## Web Development with Axum

### Getting Started

-   [Axum GitHub Repository](https://github.com/tokio-rs/axum) - Official repository with examples
-   [Axum Documentation](https://docs.rs/axum/latest/axum/) - API documentation
-   [Tower Documentation](https://docs.rs/tower/latest/tower/) - Understanding the middleware system

### Tutorials

-   [Building a REST API with Axum](https://docs.rs/axum/latest/axum/#example) - Official example
-   [Zero To Production In Rust](https://www.zero2prod.com/) - Building a production-ready API (uses Actix, but concepts apply)
-   [Let's Build a REST API with Axum](https://dev.to/tower_rs/building-a-rest-api-with-axum-4cjf) - Step-by-step tutorial

### Advanced Topics

-   [Axum Extract Documentation](https://docs.rs/axum/latest/axum/extract/index.html) - Deep dive into extractors
-   [Tower Middleware Concepts](https://docs.rs/tower/latest/tower/trait.Service.html) - Understanding the Service trait
-   [Routing in Axum](https://docs.rs/axum/latest/axum/routing/index.html) - Advanced routing techniques

## Database Access with SQLx

### Fundamentals

-   [SQLx GitHub Repository](https://github.com/launchbadge/sqlx) - Official repository with examples
-   [SQLx Documentation](https://docs.rs/sqlx/latest/sqlx/) - Comprehensive API documentation
-   [SQLx Guide](https://github.com/launchbadge/sqlx/blob/master/README.md) - Getting started guide

### Working with Databases

-   [SQLx Migrations](https://docs.rs/sqlx/latest/sqlx/migrate/index.html) - Managing database schema changes
-   [Type-safe SQL with SQLx](https://github.com/launchbadge/sqlx#compile-time-verification) - Compile-time SQL verification
-   [Supabase Documentation](https://supabase.com/docs) - For Supabase-specific features

### Advanced SQLx

-   [Transaction Management with SQLx](https://docs.rs/sqlx/latest/sqlx/trait.TransactionManager.html) - Working with transactions
-   [SQLx with Multiple Databases](https://docs.rs/sqlx/latest/sqlx/#feature-flags) - Using SQLx with different databases
-   [Connection Pooling](https://docs.rs/sqlx/latest/sqlx/pool/struct.Pool.html) - Advanced pool configuration

## REST API Design

### Core Concepts

-   [REST API Tutorial](https://restfulapi.net/) - Comprehensive guide to REST principles
-   [RESTful Web Services](https://www.oreilly.com/library/view/restful-web-services/9780596809140/) - Classic book on REST API design
-   [HTTP Status Codes](https://httpstatuses.com/) - Reference for HTTP status codes and their meaning

### Best Practices

-   [API Design Guide by Google](https://cloud.google.com/apis/design) - Google's approach to API design
-   [JSON:API Specification](https://jsonapi.org/) - A standard for designing APIs in JSON
-   [REST API Design Rulebook](https://www.oreilly.com/library/view/rest-api-design/9781449317904/) - Consistent design rules

## Authentication & Security

### Web Security Concepts

-   [OWASP Top Ten](https://owasp.org/www-project-top-ten/) - Common web vulnerabilities
-   [Authentication vs. Authorization](https://auth0.com/docs/get-started/identity-fundamentals/authentication-and-authorization) - Understanding the difference
-   [Web Security Academy](https://portswigger.net/web-security) - Free, in-depth web security training

### Implementing Auth in Rust

-   [JWT Authentication in Rust](https://blog.logrocket.com/jwt-authentication-in-rust/) - Using JSON Web Tokens
-   [argonautica](https://docs.rs/argonautica/latest/argonautica/) - Secure password hashing
-   [Rust Crypto Libraries](https://github.com/RustCrypto) - Cryptographic algorithms in Rust

## Testing in Rust

### Fundamentals

-   [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html) - Official documentation on testing
-   [Test Organization](https://doc.rust-lang.org/book/ch11-03-test-organization.html) - How to structure tests
-   [rust-analyzer test explorer](https://rust-analyzer.github.io/) - IDE integration for tests

### Testing Web APIs

-   [Axum Testing Examples](https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs) - Official examples
-   [Tower Testing Documentation](https://docs.rs/tower/latest/tower/util/index.html) - Testing middleware
-   [MockDB for SQLx](https://docs.rs/sqlx-mock/latest/sqlx_mock/) - Mocking database connections

### Advanced Testing

-   [Property-based Testing with proptest](https://altsysrq.github.io/proptest-book/intro.html) - Beyond example-based tests
-   [Fuzz Testing in Rust](https://rust-fuzz.github.io/book/) - Finding bugs with random inputs
-   [Test Coverage in Rust](https://github.com/xd009642/tarpaulin) - Measuring code coverage

## Error Handling

### Rust Error Handling

-   [Rust Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html) - Official guide
-   [Error Handling in Rust](https://blog.burntsushi.net/rust-error-handling/) - In-depth blog post
-   [anyhow and thiserror](https://github.com/dtolnay/anyhow) - Popular error handling libraries

### API Error Patterns

-   [Problem Details for HTTP APIs](https://datatracker.ietf.org/doc/html/rfc7807) - Standard for error responses
-   [API Error Response Design](https://apisyouwonthate.com/blog/making-the-most-of-http-errors/) - Best practices
-   [Axum Error Handling](https://docs.rs/axum/latest/axum/error_handling/index.html) - Axum-specific patterns

## Example Projects

### Axum Examples

-   [Axum Examples Directory](https://github.com/tokio-rs/axum/tree/main/examples) - Official examples
-   [Axum CRUD API](https://github.com/launchbadge/realworld-axum-sqlx) - RealWorld implementation
-   [Axum REST API](https://github.com/codemountains/axum-rest-api) - Full featured example

### Full Stack Rust

-   [Shuttle](https://www.shuttle.rs/examples) - Rust examples for full applications
-   [Awesome Rust Web](https://github.com/flosse/rust-web-framework-comparison) - Collection of Rust web projects
-   [Rust Web Development](https://github.com/Rust-Web-Development/code) - Code for the Rust Web Development book

## Books

### Rust Books

-   [Programming Rust, 2nd Edition](https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/) - Comprehensive Rust book
-   [Rust in Action](https://www.manning.com/books/rust-in-action) - Practical systems programming
-   [Hands-on Rust](https://pragprog.com/titles/hwrust/hands-on-rust/) - Learning by building games

### Web Development Books

-   [Rust Web Development](https://www.manning.com/books/rust-web-development) - Building web applications with Rust
-   [Zero To Production In Rust](https://www.zero2prod.com/) - Building production-ready applications
-   [Black Hat Rust](https://kerkour.com/black-hat-rust) - Security-focused Rust development

## Video Courses

### Rust Fundamentals

-   [Rust Programming: The Complete Developer's Guide](https://www.udemy.com/course/rust-programming-the-complete-developers-guide/) - Udemy course
-   [Rust Fundamentals](https://www.pluralsight.com/courses/rust-fundamentals) - Pluralsight course
-   [Rust 101](https://www.youtube.com/playlist?list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8) - YouTube series

### Web Development

-   [Building Web APIs with Rust](https://www.youtube.com/watch?v=7aQ61XGOHbg) - Conference talk
-   [Tokio and Axum Tutorial](https://www.youtube.com/watch?v=XZtlD_m59sM) - Building a REST API
-   [Let's Get Rusty](https://www.youtube.com/c/LetsGetRusty) - YouTube channel with Rust web development videos

---

## How to Use These Resources

If you're new to Rust:

1. Start with "The Rust Programming Language Book"
2. Complete Rustlings exercises
3. Build small command-line applications

If you're new to Axum:

1. Read the Axum documentation and examples
2. Study the Tower middleware system
3. Build a simple CRUD API

If you're experienced with both:

1. Focus on advanced topics like authentication, testing, and performance
2. Study the example projects for best practices
3. Contribute to open-source Rust web projects

Remember that the best way to learn is by doing. Start with small projects and gradually build up to more complex applications.
