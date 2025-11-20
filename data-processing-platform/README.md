# Modern Data Processing Platform

A comprehensive data processing platform built with Rust, featuring:

- Axum web framework for high-performance APIs
- Casbin for authorization with database storage
- OAuth2 with JWT for authentication
- PostgreSQL database with connection pooling
- DataFusion for analytics and query processing
- Flight SQL for distributed queries
- ADBC for database connectivity
- Support for multiple data sources (CSV, PostgreSQL, MySQL, etc.)

## Features

### Authentication & Authorization
- JWT-based authentication
- OAuth2 flow
- Casbin-based authorization with database persistence
- Role-based access control

### Data Processing
- DataFusion query engine for analytics
- Support for multiple data sources (CSV, JSON, databases)
- Flight SQL for distributed processing
- Arrow as in-memory format
- ADBC for database abstraction

### API Endpoints
- `/auth` - Authentication endpoints
- `/users` - User management
- `/permissions` - Authorization management
- `/datafusion` - Data processing endpoints

## Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   API Layer     │    │   Service       │    │   Data Layer    │
│   (Axum)        │───▶│   Layer         │───▶│   (PostgreSQL   │
└─────────────────┘    └─────────────────┘    │   DataFusion)   │
                                              └─────────────────┘
```

## Setup

1. Install Rust and Cargo
2. Set up PostgreSQL database
3. Configure environment variables
4. Run migrations
5. Start the server

## Environment Variables

- `RUN_MODE` - Environment mode (development, test, production)
- `DATABASE_URL` - PostgreSQL connection string
- `JWT_SECRET` - Secret key for JWT tokens
- `CASBIN_MODEL_PATH` - Path to Casbin model file

## Running the Application

```bash
# Install dependencies
cargo build

# Run migrations
cargo run

# Or with specific environment
RUN_MODE=production cargo run
```

## Project Structure

```
src/
├── config/           # Configuration management
├── database/         # Database connection and models
├── errors/           # Error handling
├── handlers/         # API request handlers
├── middleware/       # Request middleware
├── models/           # Data models
├── routes/           # API route definitions
├── services/         # Business logic
├── utils/            # Utility functions
├── casbin/           # Authorization logic
├── datafusion/       # Data processing logic
├── flight/           # Flight SQL implementation
└── adbc/             # ADBC implementation
```

## Development

This project follows Rust best practices and includes:
- Comprehensive error handling
- Type safety
- Async/await patterns
- Proper logging
- Configuration management
- Security considerations