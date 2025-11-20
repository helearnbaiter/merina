# Modern Data Processing Platform

A production-ready data processing platform built with Rust, featuring:

- **Web Framework**: Axum for high-performance, type-safe APIs
- **Authentication**: JWT-based authentication with OAuth2 flow
- **Authorization**: Casbin-based permission system with database persistence
- **Database**: PostgreSQL with SQLx and SeaORM for async database operations
- **Data Processing**: Apache DataFusion query engine supporting multiple data sources
- **Data Formats**: Arrow in-memory format, Parquet, CSV, JSON
- **Flight SQL**: Apache Arrow Flight protocol for high-performance data access
- **ADBC**: Arrow Database Connectivity for database abstraction
- **Iceberg**: Native Apache Iceberg support
- **Config Management**: Environment-specific configuration for dev/test/prod
- **Error Handling**: Comprehensive error handling with unified response format
- **Logging**: Structured logging with tracing

## Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Frontend      │────│    Axum API      │────│  PostgreSQL     │
│   (Superset,    │    │                  │    │                 │
│   etc.)         │    │  ┌─────────────┐ │    │  ┌─────────────┐ │
└─────────────────┘    │  │ Auth Layer  │ │    │  │   Casbin    │ │
                       │  │ (JWT/Casbin)│ │    │  │   Policy    │ │
                       │  └─────────────┘ │    │  │   Storage   │ │
                       │                  │    │  └─────────────┘ │
                       │  ┌─────────────┐ │    │                 │
                       │  │ DataFusion  │ │    │  ┌─────────────┐ │
                       │  │   Engine    │ │────│──│  DataFusion │ │
                       │  └─────────────┘ │    │  │   Adapter   │ │
                       │                  │    │  └─────────────┘ │
                       │  ┌─────────────┐ │    └─────────────────┘
                       │  │ Flight SQL  │ │
                       │  │   Server    │ │
                       │  └─────────────┘ │
                       └──────────────────┘
```

## Features

### Authentication & Authorization
- JWT-based authentication with refresh tokens
- OAuth2 integration
- Casbin-based authorization with database persistence
- Role-based access control (RBAC)
- Policy management via REST API

### Data Processing
- Apache DataFusion query engine
- Support for multiple data sources:
  - Memory tables
  - CSV files
  - PostgreSQL, MySQL, SQLite databases
  - Parquet files
  - JSON files
  - Apache Arrow format
  - Apache Iceberg tables
  - Remote data sources
- SQL query execution
- Distributed query capabilities
- Custom function support

### Flight SQL
- Apache Arrow Flight protocol implementation
- High-performance data access
- Compatible with Apache Superset
- Prepared statement support

### Configuration
- Environment-specific configurations (dev/test/prod)
- Database connection pooling
- JWT token management
- DataFusion settings
- Flight server configuration

## Getting Started

### Prerequisites
- Rust 1.70+
- PostgreSQL 12+
- Docker (for optional containerization)

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd modern-data-platform
```

2. Install dependencies:
```bash
cargo build
```

3. Set up environment variables:
```bash
cp .env.example .env
# Edit .env with your configuration
```

4. Run database migrations:
```bash
cargo run
```

### Environment Configuration

The application supports three environments:
- `development` - For local development
- `test` - For testing
- `production` - For production deployment

Configuration files are located in the `config/` directory.

### API Endpoints

#### Authentication
- `POST /api/auth/login` - User login
- `POST /api/auth/refresh` - Token refresh

#### Casbin Policy Management
- `GET /api/casbin/policies` - Get all policies
- `POST /api/casbin/policies` - Add policy
- `DELETE /api/casbin/policies` - Remove policy
- `GET /api/casbin/rules` - Get all rules
- `POST /api/casbin/rules` - Add rule
- `PUT /api/casbin/rules/{id}` - Update rule
- `DELETE /api/casbin/rules/{id}` - Delete rule

#### Data Sources
- `GET /api/data-sources` - List data sources
- `POST /api/data-sources` - Create data source
- `GET /api/data-sources/{id}` - Get data source
- `PUT /api/data-sources/{id}` - Update data source
- `DELETE /api/data-sources/{id}` - Delete data source

#### Query Execution
- `POST /api/query/execute` - Execute SQL query

### Flight SQL Server

The Flight SQL server runs on port 50051 by default when enabled in the configuration. It's compatible with Apache Superset and other Flight SQL clients.

## Environment Variables

```bash
# Database
DATABASE_URL=postgresql://user:password@localhost/dbname

# JWT
JWT_SECRET=your-super-secret-jwt-key

# Application
APP_ENV=development
APP_HOST=0.0.0.0
APP_PORT=8080
```

## Running the Application

### Development
```bash
cargo run
```

### Production
```bash
APP_ENV=production cargo run --release
```

## Testing

Run all tests:
```bash
cargo test
```

Run tests with logging:
```bash
RUST_LOG=debug cargo test
```

## Deployment

The application can be deployed as a standalone binary or containerized with Docker.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.