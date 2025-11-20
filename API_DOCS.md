# API Documentation

## Authentication Endpoints

### POST /api/auth/login
**Description**: Authenticate user and return JWT tokens

**Request Body**:
```json
{
  "username": "string",
  "password": "string"
}
```

**Response**:
```json
{
  "access_token": "string",
  "refresh_token": "string",
  "token_type": "Bearer",
  "expires_in": 3600
}
```

### POST /api/auth/refresh
**Description**: Refresh expired access token using refresh token

**Request Body**:
```json
{
  "refresh_token": "string"
}
```

**Response**:
```json
{
  "access_token": "string",
  "refresh_token": "string",
  "token_type": "Bearer",
  "expires_in": 3600
}
```

## Casbin Policy Management

### GET /api/casbin/policies
**Description**: Retrieve all authorization policies

**Response**:
```json
[
  ["user1", "data1", "read"],
  ["user2", "data2", "write"]
]
```

### POST /api/casbin/policies
**Description**: Add a new authorization policy

**Request Body**:
```json
{
  "sub": "user_id",
  "obj": "resource_id",
  "act": "action"
}
```

**Response**:
```json
{
  "success": true,
  "message": "Policy added successfully"
}
```

### DELETE /api/casbin/policies
**Description**: Remove an authorization policy

**Request Body**:
```json
{
  "sub": "user_id",
  "obj": "resource_id",
  "act": "action"
}
```

**Response**:
```json
{
  "success": true,
  "message": "Policy removed successfully"
}
```

## Casbin Rule Management

### GET /api/casbin/rules
**Description**: Retrieve all Casbin rules from database

**Response**:
```json
[
  {
    "id": 1,
    "ptype": "p",
    "v0": "user1",
    "v1": "data1",
    "v2": "read",
    "v3": null,
    "v4": null,
    "v5": null
  }
]
```

### POST /api/casbin/rules
**Description**: Add a new Casbin rule to database

**Request Body**:
```json
{
  "ptype": "p",
  "v0": "user1",
  "v1": "data1",
  "v2": "read",
  "v3": null,
  "v4": null,
  "v5": null
}
```

**Response**:
```json
{
  "id": 1,
  "ptype": "p",
  "v0": "user1",
  "v1": "data1",
  "v2": "read",
  "v3": null,
  "v4": null,
  "v5": null
}
```

### PUT /api/casbin/rules/{id}
**Description**: Update an existing Casbin rule

**Request Body**:
```json
{
  "ptype": "p",
  "v0": "user1",
  "v1": "data1",
  "v2": "write",
  "v3": null,
  "v4": null,
  "v5": null
}
```

**Response**:
```json
{
  "id": 1,
  "ptype": "p",
  "v0": "user1",
  "v1": "data1",
  "v2": "write",
  "v3": null,
  "v4": null,
  "v5": null
}
```

### DELETE /api/casbin/rules/{id}
**Description**: Delete a Casbin rule

**Response**:
```json
{
  "success": true,
  "deleted_count": 1,
  "message": "Rule deleted successfully"
}
```

## Data Source Management

### GET /api/data-sources
**Description**: List all registered data sources

**Response**:
```json
[
  {
    "id": "uuid-string",
    "name": "My Data Source",
    "type": "PostgreSQL",
    "connection_string": "postgresql://...",
    "options": {},
    "schema": null,
    "created_at": "2023-01-01T00:00:00",
    "updated_at": "2023-01-01T00:00:00"
  }
]
```

### POST /api/data-sources
**Description**: Create a new data source

**Request Body**:
```json
{
  "name": "My Data Source",
  "type": "PostgreSQL",
  "connection_string": "postgresql://user:pass@host:port/db",
  "options": {
    "ssl": "true"
  }
}
```

**Response**:
```json
{
  "id": "uuid-string",
  "name": "My Data Source",
  "type": "PostgreSQL",
  "connection_string": "postgresql://user:pass@host:port/db",
  "options": {
    "ssl": "true"
  },
  "schema": null,
  "created_at": "2023-01-01T00:00:00",
  "updated_at": "2023-01-01T00:00:00"
}
```

### GET /api/data-sources/{id}
**Description**: Get a specific data source by ID

**Response**:
```json
{
  "id": "uuid-string",
  "name": "My Data Source",
  "type": "PostgreSQL",
  "connection_string": "postgresql://user:pass@host:port/db",
  "options": {},
  "schema": null,
  "created_at": "2023-01-01T00:00:00",
  "updated_at": "2023-01-01T00:00:00"
}
```

### PUT /api/data-sources/{id}
**Description**: Update an existing data source

**Request Body**:
```json
{
  "name": "Updated Data Source",
  "type": "PostgreSQL",
  "connection_string": "postgresql://newuser:newpass@host:port/db",
  "options": {
    "ssl": "true"
  }
}
```

**Response**:
```json
{
  "id": "uuid-string",
  "name": "Updated Data Source",
  "type": "PostgreSQL",
  "connection_string": "postgresql://newuser:newpass@host:port/db",
  "options": {
    "ssl": "true"
  },
  "schema": null,
  "created_at": "2023-01-01T00:00:00",
  "updated_at": "2023-01-02T00:00:00"
}
```

### DELETE /api/data-sources/{id}
**Description**: Delete a data source

**Response**:
```json
{
  "success": true,
  "message": "Data source deleted successfully"
}
```

## Query Execution

### POST /api/query/execute
**Description**: Execute a SQL query against registered data sources

**Request Body**:
```json
{
  "sql": "SELECT * FROM my_table WHERE id = $1"
}
```

**Response**:
```json
{
  "schema": "{\"fields\":[...]}",
  "rows": [
    {"id": 1, "name": "example"},
    {"id": 2, "name": "test"}
  ],
  "execution_time_ms": 15,
  "row_count": 2
}
```

## Health Check

### GET /health
**Description**: Check application health status

**Response**:
```json
{
  "status": "healthy",
  "timestamp": 1678886400
}
```

## Error Response Format

All error responses follow this format:

```json
{
  "error": "Error reason",
  "message": "Human-readable error message",
  "status_code": 400
}
```

## Authentication

Most endpoints require a valid JWT token in the Authorization header:

```
Authorization: Bearer <jwt-token>
```

Public endpoints (like `/health` and `/api/auth/login`) do not require authentication.