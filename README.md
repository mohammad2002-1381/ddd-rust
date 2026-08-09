# Clean Architecture Rust API

A RESTful API built with Rust following Clean Architecture principles, powered by the Axum web framework.

## 🏗️ Architecture

This project implements Clean Architecture, organizing code into distinct layers with clear separation of concerns:

```
┌─────────────────────────────────────────┐
│           API Layer (Axum)              │
│  Controllers, Routes, Middlewares        │
└─────────────────┬───────────────────────┘
                  │ depends on
┌─────────────────▼───────────────────────┐
│        Application Layer                │
│  Business Logic, Commands, Queries,     │
│  DTOs, MediatR                          │
└─────────────────┬───────────────────────┘
                  │ depends on
┌─────────────────▼───────────────────────┐
│      Infrastructure Layer               │
│  Repositories, Database Context         │
└─────────────────┬───────────────────────┘
                  │ depends on
┌─────────────────▼───────────────────────┐
│        Domain Layer                     │
│  Entities, Models, Domain Models       │
└─────────────────────────────────────────┘
```

### Layer Responsibilities & Dependencies

The layers follow a strict dependency flow: **API → Application → Infrastructure → Domain**

- **API Layer** (`crates/api`): Handles HTTP requests/responses, routing, and middleware. Depends on Application layer.
- **Application Layer** (`crates/application`): Contains business logic, use cases, commands, queries, and DTOs following the MediatR pattern. Depends on Infrastructure layer.
- **Infrastructure Layer** (`crates/infrastructure`): Database access, repositories, and external service integrations. Depends on Domain layer.
- **Domain Layer** (`crates/domain`): Core business entities and domain models. No dependencies on other layers (innermost layer).

### Key Design Patterns

- **MediatR Pattern**: Decouples request handling from controllers
- **Repository Pattern**: Abstracts data access logic
- **Dependency Injection**: Loose coupling between layers
- **CQRS**: Separation of commands (writes) and queries (reads)

## 🚀 Technology Stack

- **Framework**: [Axum](https://github.com/tokio-rs/axum) 0.8.8 - A fast, ergonomic web framework
- **Runtime**: [Tokio](https://tokio.rs/) - Async runtime for Rust
- **ORM**: [Sea-ORM](https://www.sea-orm.com/) 1.1.19 - Async ORM for Rust
- **Database**: PostgreSQL
- **Serialization**: [Serde](https://serde.rs/)
- **Authentication**: JWT (jsonwebtoken) with Argon2 password hashing
- **Language**: Rust (Edition 2024)

## 📡 API Endpoints

### Base URL
```
http://localhost:3000/api/v1
```

### Products

> **Note:** All product endpoints require authentication. Include a valid JWT token in the `Authorization` header: `Authorization: Bearer <token>`

#### Create Product
```http
POST /api/v1/product
Authorization: Bearer <token>
Content-Type: application/json

{
  "name": "Product Name",
  "price": 9999
}
```

**Response:** `201 Created`
```json
{
  "id": 1,
  "name": "Product Name",
  "price": 9999,
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z"
}
```

#### Get Product by ID
```http
GET /api/v1/product/{id}
Authorization: Bearer <token>
```

**Response:** `202 Accepted`
```json
{
  "id": 1,
  "name": "Product Name",
  "price": 9999,
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z"
}
```

#### Get Product List
```http
GET /api/v1/product?page=1&page_size=10
Authorization: Bearer <token>
```

**Response:** `202 Accepted`
```json
[
  {
    "id": 1,
    "name": "Product Name",
    "price": 9999,
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  }
]
```

#### Update Product
```http
PATCH /api/v1/product
Authorization: Bearer <token>
Content-Type: application/json

{
  "id": 1,
  "name": "Updated Product Name",
  "price": 12999,
  "is_active": true
}
```

**Response:** `201 Created`
```json
{
  "id": 1,
  "name": "Updated Product Name",
  "price": 12999,
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:01Z"
}
```

#### Delete Product
```http
DELETE /api/v1/product
Authorization: Bearer <token>
Content-Type: application/json

{
  "id": 1
}
```

**Response:** `201 Created`
```json
1
```

### Users

#### Register User
```http
POST /api/v1/user/register
Content-Type: application/json

{
  "first_name": "John",
  "last_name": "Doe",
  "email": "user@example.com",
  "password": "securepassword",
  "is_active": true
}
```

**Response:** `201 Created`
```json
{
  "user": {
    "id": 1,
    "first_name": "John",
    "last_name": "Doe",
    "email": "user@example.com",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  },
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refresh_token": "refresh-token-string"
}
```

#### Login User
```http
POST /api/v1/user/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "securepassword"
}
```

**Response:** `201 Created`
```json
{
  "user": {
    "id": 1,
    "first_name": "John",
    "last_name": "Doe",
    "email": "user@example.com",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  },
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refresh_token": "refresh-token-string"
}
```

> **Note:** Use the `token` from the response in the `Authorization` header for authenticated requests: `Authorization: Bearer <token>`

## 🛠️ Setup Guide

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [PostgreSQL](https://www.postgresql.org/download/) (12 or higher)
- [Docker](https://www.docker.com/get-started) and [Docker Compose](https://docs.docker.com/compose/install/) (optional, for containerized setup)

### Setup Without Docker

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd clean-architecture-rust
   ```

2. **Set up PostgreSQL database**
   ```bash
   # Create a new database
   createdb clean_architecture_db
   
   # Or using psql
   psql -U postgres
   CREATE DATABASE clean_architecture_db;
   ```

3. **Configure environment variables**
   ```bash
   # Create a .env file in the root directory
   cp .env.example .env  # If you have an example file
   # Or create it manually
   ```
   
   Add the following to `.env`:
   ```env
   DATABASE_URL=postgresql://username:password@localhost:5432/clean_architecture_db
   JWT_SECRET=your-secret-key-here-minimum-32-characters
   ```

4. **Run database migrations** (if you have migrations)
   ```bash
   # Using sea-orm-cli (install with: cargo install sea-orm-cli)
   sea-orm-cli migrate up
   
   # Or if using SQLx migrations
   # cargo install sqlx-cli
   # sqlx migrate run
   ```

5. **Build the project**
   ```bash
   cargo build --release
   ```

6. **Run the application**
   ```bash
   cargo run --release -p api
   ```

   The server will start on `http://localhost:3000`

### Setup With Docker

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd clean-architecture-rust
   ```

2. **Create `.env` file**
   ```env
   DATABASE_URL=postgresql://postgres:postgres@postgres:5432/clean_architecture_db
   JWT_SECRET=your-secret-key-here-minimum-32-characters
   ```

3. **Start services with Docker Compose**
   ```bash
   docker-compose up --build
   ```

   This will:
   - Build the Rust application
   - Start the API service on port 3000
   - Set up PostgreSQL (if configured in docker-compose.yaml)

4. **Access the API**
   ```
   http://localhost:3000
   ```

### Development

For development with hot-reload, you can use:

```bash
# Install cargo-watch if not already installed
cargo install cargo-watch

# Run with auto-reload
cargo watch -x 'run -p api'
```

## 📁 Project Structure

```
clean-architecture-rust/
├── Cargo.toml                 # Workspace configuration
├── docker-compose.yaml        # Docker Compose configuration
├── crates/
│   ├── api/                   # API Layer
│   │   ├── src/
│   │   │   ├── main.rs        # Application entry point
│   │   │   ├── controllers/   # Route handlers
│   │   │   ├── middlewares/   # Custom middlewares
│   │   │   └── state.rs       # Application state
│   │   └── Dockerfile         # Docker image definition
│   ├── application/           # Application Layer
│   │   └── src/
│   │       ├── features/      # Feature modules (CQRS)
│   │       │   ├── products/  # Product feature
│   │       │   └── users/     # User feature
│   │       └── common/        # Shared application logic
│   ├── domain/                # Domain Layer
│   │   └── src/
│   │       ├── models/        # Domain entities
│   │       └── base_entity.rs # Base entity traits
│   └── infrastructure/        # Infrastructure Layer
│       └── src/
│           ├── repositories/  # Data access implementations
│           └── db_context.rs  # Database connection pool
└── target/                    # Build artifacts
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p api
cargo test -p application
```

## 📝 Environment Variables

| Variable | Description | Required |
|----------|-------------|----------|
| `DATABASE_URL` | PostgreSQL connection string | Yes |
| `JWT_SECRET` | Secret key for JWT token signing | Yes (if using authentication) |

Example `.env` file:
```env
DATABASE_URL=postgresql://username:password@localhost:5432/database_name
JWT_SECRET=your-secret-key-here
```

## 🔧 Building for Production

```bash
# Build optimized release binary
cargo build --release -p api

# The binary will be located at:
# target/release/api
```

## 📚 Additional Resources

- [Axum Documentation](https://docs.rs/axum/)
- [Tokio Documentation](https://docs.rs/tokio/)
- [Sea-ORM Documentation](https://www.sea-orm.com/)
- [Rust Book](https://doc.rust-lang.org/book/)

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

MIT

---

Built with ❤️ using Rust and Axum

