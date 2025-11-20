# Project Architecture

## Directory Structure

```
/workspace/
├── Cargo.toml              # Rust project manifest
├── Cargo.lock              # Dependency lock file
├── build.rs                # Build script
├── .env.example           # Environment variables example
├── README.md              # Project documentation
├── ARCHITECTURE.md        # This file
├── config/                # Configuration files
│   ├── base.toml          # Base configuration
│   ├── development.toml   # Development environment config
│   ├── test.toml          # Test environment config
│   ├── production.toml    # Production environment config
│   └── casbin/
│       └── model.conf     # Casbin access control model
├── migrations/            # Database migrations
│   └── 001_casbin_rules_table.sql
├── src/
│   ├── main.rs            # Application entry point
│   ├── config/            # Configuration management
│   │   └── mod.rs         # Configuration structs and loading
│   ├── handlers/          # API route handlers
│   │   ├── mod.rs
│   │   ├── auth.rs        # Authentication endpoints
│   │   ├── casbin.rs      # Casbin policy management
│   │   ├── data_source.rs # Data source management
│   │   ├── health.rs      # Health check endpoints
│   │   └── query.rs       # Query execution endpoints
│   ├── middleware/        # Axum middleware
│   │   ├── mod.rs
│   │   ├── auth.rs        # Authentication middleware
│   │   ├── cors.rs        # CORS middleware
│   │   └── logger.rs      # Request logging middleware
│   ├── services/          # Business logic services
│   │   └── casbin_service.rs # Casbin service with DB persistence
│   ├── datafusion_adapters/ # DataFusion integration
│   │   ├── mod.rs
│   │   ├── data_source.rs # Data source management
│   │   ├── query_engine.rs # Query execution engine
│   │   └── flight_server.rs # Flight SQL server
│   └── utils/             # Utility functions
│       ├── mod.rs
│       └── auth.rs        # Authentication utilities
```

## Core Components

### 1. Configuration System (`src/config/`)
- Environment-specific configuration loading
- Support for dev/test/prod environments
- Configuration via files and environment variables

### 2. Authentication & Authorization (`src/utils/auth.rs`, `src/services/casbin_service.rs`)
- JWT-based authentication with refresh tokens
- OAuth2 flow implementation
- Casbin-based authorization with database persistence
- Role-based access control (RBAC)

### 3. Data Processing Engine (`src/datafusion_adapters/`)
- Apache DataFusion query engine integration
- Support for multiple data sources (CSV, Parquet, JSON, databases, etc.)
- Apache Arrow in-memory format
- Flight SQL protocol implementation
- Apache Iceberg native support

### 4. Web API (`src/handlers/`, `src/middleware/`)
- RESTful API endpoints
- Modular route organization
- Authentication and authorization middleware
- Unified error handling and response format
- CORS support

### 5. Database Integration
- PostgreSQL as primary database
- SQLx for async database operations
- Connection pooling
- Database migrations

## API Endpoints

### Authentication
- `POST /api/auth/login` - User authentication
- `POST /api/auth/refresh` - Token refresh

### Casbin Policy Management
- `GET /api/casbin/policies` - List all policies
- `POST /api/casbin/policies` - Add policy
- `DELETE /api/casbin/policies` - Remove policy
- `GET /api/casbin/rules` - List all rules
- `POST /api/casbin/rules` - Add rule
- `PUT /api/casbin/rules/{id}` - Update rule
- `DELETE /api/casbin/rules/{id}` - Delete rule

### Data Source Management
- `GET /api/data-sources` - List data sources
- `POST /api/data-sources` - Create data source
- `GET /api/data-sources/{id}` - Get specific data source
- `PUT /api/data-sources/{id}` - Update data source
- `DELETE /api/data-sources/{id}` - Delete data source

### Query Execution
- `POST /api/query/execute` - Execute SQL query against registered data sources

## Data Flow

1. **Request Processing**: Incoming HTTP requests are processed by Axum
2. **Authentication**: JWT tokens are validated in middleware
3. **Authorization**: Casbin enforces access control policies
4. **Business Logic**: Handlers process requests using services
5. **Data Access**: Database operations via SQLx/SeaORM
6. **Data Processing**: DataFusion processes queries against various data sources
7. **Response**: Structured responses with unified error handling

## Error Handling

The application uses a unified error handling system:
- `AppError` enum for all possible error types
- Automatic conversion to HTTP responses
- Structured error responses with codes and messages
- Comprehensive logging for debugging

## Security Features

- JWT token-based authentication
- Casbin-based authorization with database persistence
- SQL injection prevention through parameterized queries
- CORS configuration
- Secure token storage and refresh mechanisms

## Performance Features

- Async/await throughout for high concurrency
- Connection pooling for database operations
- Apache Arrow for efficient in-memory data processing
- Apache DataFusion for optimized query execution
- Flight SQL for high-performance data access