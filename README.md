# Rust + Clean Architecture + DDD Approach

A Rust workspace built with **Clean Architecture**, **Domain-Driven Design (DDD)**, and **CQRS**. Each layer is an isolated crate. `api` is the only binary (composition root + HTTP). Every other crate is a library.

The write side (`domain` + `infrastructure`) and the read side (`read_model` + `query_handler`) are separate stacks. Each stack owns its own database context. Today both pools read `DATABASE_URL`, so they can share one PostgreSQL instance or later point at different databases without changing application code.

---

## Architecture at a glance

```
                    ┌─────────────────────────────────────┐
                    │              api                    │
                    │  presentation · controllers · JWT   │
                    │  middleware · HTTP mapping          │
                    └──────────────┬──────────────────────┘
                                   │
                    ┌──────────────▼──────────────────────┐
                    │           application               │
                    │  features (commands / queries)      │
                    │  external services · use cases      │
                    └──┬───────────────────────────────┬──┘
                       │                               │
         write side    │                               │    read side
                       │                               │
         ┌─────────────▼─────────────┐   ┌─────────────▼─────────────┐
         │      infrastructure       │   │       query_handler       │
         │  write db_context         │   │  read db_context          │
         │  repository impls         │   │  query service impls      │
         │  SeaORM entity configs    │   │  read mappers             │
         │  (owns one DB)            │   │  pagination               │
         │                           │   │  (owns one DB)            │
         └─────────────┬─────────────┘   └─────────────┬─────────────┘
                       │                               │
         ┌─────────────▼─────────────┐   ┌─────────────▼─────────────┐
         │          domain           │   │         read_model        │
         │  aggregates · entities    │   │  DTOs · query requests    │
         │  repository traits        │   │  query service traits     │
         │  VOs · enums              │   │  mapper · pagination      │
         │  (owns one DB contract)   │   │  (owns one DB contract)   │
         └───────────────────────────┘   └───────────────────────────┘
```

Dependency direction is always **inward**. Outer crates may depend on inner crates. Inner crates never depend on outer crates.

```
api ─────────────────────────────────────────────┐
 │                                               │
 ├── application                                 │
 │    ├── infrastructure ──► domain              │
 │    ├── query_handler ───► read_model          │
 │    ├── domain                                 │
 │    └── read_model                             │
 │                                               │
 ├── infrastructure ───────► domain              │
 ├── query_handler ────────► read_model          │
 ├── domain                                      │
 └── read_model                                  │
```

| Crate | Kind | Role |
|---|---|---|
| `api` | binary | Presentation: HTTP, controllers, auth middleware, response/error mapping |
| `application` | lib | Use cases: feature commands/queries and application services |
| `infrastructure` | lib | Write persistence: SeaORM mappings and repository implementations |
| `domain` | lib | Write model: aggregates, entities, value objects, repository traits |
| `query_handler` | lib | Read persistence: SeaORM read mappings and query service implementations |
| `read_model` | lib | Read model: DTOs, query requests, query service traits, mappers |

`migration/` is a SeaORM migrator crate. It is **not** a workspace member and is not part of the runtime layer graph.

---

## Design principles

### Clean Architecture

- **Independence of frameworks.** Domain and read-model contracts do not know Axum or SeaORM repositories. Persistence adapters live in `infrastructure` and `query_handler`.
- **Independence of UI.** Controllers only translate HTTP into commands/queries and map results back to HTTP.
- **Independence of database.** Application code talks to traits (`IUserRepository`, `IAgentQueryService`, …). PostgreSQL is one adapter.
- **Testability.** A command or query handler is a generic function over a trait. Tests can substitute a fake repository or query service.

### DDD

- Aggregates encapsulate identity and invariants (`User`, `Product`, `Agent`, `Tool`, `Role`, `RoleAssignment`).
- Entities carry identity via `BaseEntity<Id>` and `IBaseEntity`.
- Value objects model configuration that has no identity of its own (`ToolConfiguration`, `RestApiConfig`).
- Repository traits live next to the aggregate they persist. Implementations live in infrastructure.

### CQRS

| Side | Contract crate | Adapter crate | Persistence |
|---|---|---|---|
| **Command (write)** | `domain` repository traits | `infrastructure` | write `db_context` |
| **Query (read)** | `read_model` query service traits | `query_handler` | read `db_context` |

Commands load aggregates, mutate them, and persist through repositories. Queries never touch aggregates. They project rows into DTOs through read-side mappers.

---

## Layer contracts

### `api` — presentation

Owns HTTP and nothing else. It does not contain business rules.

| Area | Responsibility |
|---|---|
| `main.rs` | Composition root: init state, application services, write DB, read DB, bind Axum |
| `controllers/` | Route tables and handler functions |
| `middlewares/` | JWT auth, role gates, JSON validation, `ApiResponse`, `ApiError` |
| `state.rs` | Shared Axum state (composition hook; repositories are currently generic type params, not stored in state) |

Controllers inject concrete adapters as **generic arguments** on application handlers:

```rust
create_agent_command::handle::<AgentRepository>(request).await?;
get_agent_by_id_query::handle::<AgentQueryService>(id).await?;
```

That keeps `application` free of `infrastructure` types at the call site of business logic, while `api` remains the place that chooses PostgreSQL.

Current route groups:

| Prefix | Controller | Auth |
|---|---|---|
| `/api/v1/user` | `IUserController` | public register/login; JWT for the rest |
| `/api/v1/product` | `IProductController` | JWT |
| `/api/v1/admin/users` | `IUsersController` | JWT + `UserRoleType::Admin` |

`IAgentController` is implemented (`/admin/agents` style routes, admin-gated) but is not nested into `admin_router()` yet.

### `application` — use cases

Owns the business workflow for an API request. It depends on **traits**, not adapters.

```
application/src
├── features/
│   ├── users/      commands · queries · auth DTOs
│   ├── products/   commands · queries
│   └── agents/     commands · queries
├── services/       JWT, current user, Redis cache, app config
└── common/         CommandError
```

A feature is split the CQRS way:

- **Command** — deserialize request, load/create an aggregate, apply a change, persist, return an id or auth payload.
- **Query** — take a `read_model` query request, call a query-service trait, return a DTO or `PaginatedResult<T>`.

Handlers are generic over the persistence trait:

```rust
pub async fn handle<AgentRepository: IAgentRepository>(
    request: CreateAgentCommand,
) -> Result<Uuid, CommandError>
```

Application services (external / cross-cutting):

| Service | Trait | Default | Purpose |
|---|---|---|---|
| JWT | `IJwtService` | `DefaultJwtService` | Access token, refresh token, API token, claims |
| Current user | `ICurrentUserService` | `CurrentUserService` | Resolve `user_id` from JWT claims |
| Cache | `ICacheService` | `RedisCacheService` | Get / set / remove with optional TTL |
| Config | `IAppConfigService` | `DefaultConfigService` | Public base URL |

JWT `Claims` (`sub`, `role`, `exp`) live in the application JWT service. API middleware decodes the bearer token into those claims and stores them on the request. Controllers extract `CurrentUserRequest`, which wraps `CurrentUserService`.

Services that need a process-wide connection (Redis) register through `ServiceRegister` during `application::add_application()`.

### `domain` — write model

Innermost write-side crate. No dependency on other workspace crates.

```
domain/src
├── aggregate_root/     IAggregateRoot<TId>
├── base_repository/    BaseCommand · BaseQuery · RepositoryError
└── models/
    ├── base_entity.rs
    ├── users/          User, Token, IUserRepository, UserRoleType
    ├── products/       Product, IProductRepository
    ├── agents/         Agent, IAgentRepository
    ├── roles/          Role
    ├── role_assignments/ RoleAssignment
    └── tools/          Tool, ToolParameters, ToolConfiguration, IToolRepository
```

**Identity and lifecycle**

- `BaseEntity<Id>` holds `id`, `created_at`, `updated_at`.
- `IBaseEntity` exposes `base()` and `touch()`.
- `IAggregateRoot<TId>` marks the consistency boundary. Only aggregates are persisted through `BaseCommand` / `BaseQuery`.

**Repository contract**

```text
BaseQuery<T, Id>     find_by_id(id) -> Option<T>
BaseCommand<T, Id>   create / update (touches updated_at) / delete
IXxxRepository       aggregate-specific methods (find_by_email, create_token, …)
```

`RepositoryError` is the only persistence failure type the domain admits: `NotFound`, `Conflict`, `Mapping`, `Database`.

**Current aggregates**

| Aggregate | Identity | Notes |
|---|---|---|
| `User` | UUID v7 | Name, email, Argon2 hash, `UserRoleType` (`Admin` / `User`), active flag |
| `Token` | UUID v7 | Access + refresh pair, expiry, belongs to a user (entity, persisted via user repository) |
| `Product` | UUID | Owned by a user; name, price, active |
| `Agent` | UUID v7 | Name + system prompt |
| `Role` | UUID v7 | Named role (domain model present; write repository not wired yet) |
| `RoleAssignment` | UUID v7 | User ↔ role link (model present; repository not wired yet) |
| `Tool` | UUID v7 | Belongs to `role_id` + `agent_id`; configuration is a value object |

`ToolConfiguration` is a tagged value object. The first variant is `RestApi { method, url, headers, body }`. `ToolParameters` describe named inputs (`string` / `number` / `boolean` / `date`).

### `infrastructure` — write adapters

Depends only on `domain`. Owns **one** write database connection (`OnceLock<DatabaseConnection>`), initialized by `infrastructure::add_infrastructure()`.

```
infrastructure/src
├── db_context.rs              write pool (DATABASE_URL)
├── entity_configurations/     SeaORM models + From domain ↔ ActiveModel
│   ├── user.rs
│   ├── token.rs
│   ├── product.rs
│   ├── agent.rs
│   └── tool.rs
├── repository/pg/             PostgreSQL repository implementations
│   ├── user_repository.rs     → UserRepository
│   ├── product_repository.rs  → ProductRepository
│   └── agent_repository.rs    → AgentRepository
├── queryable.rs               first_async helpers for SeaORM Select
└── executable.rs              delete exec helpers
```

Entity configurations are **write mappings**: they convert a domain aggregate into a SeaORM `ActiveModel` and a stored row back into an aggregate. They are not used for API responses.

Generic create / update / delete / find-by-id are generated with `base_command!` and `base_query!` macros so each repository stays thin. Extra methods (`find_by_email_async`, `find_by_id_user_id`, `create_token`) are implemented by hand.

Public exports:

```rust
pub use repository::pg as repositories;
// repositories::{UserRepository, ProductRepository, AgentRepository}
```

`IToolRepository` exists in domain; there is no PostgreSQL repository for it yet.

### `read_model` — read contracts

Innermost read-side crate. No dependency on `domain` or `infrastructure`. The read model is allowed to look different from the write model (for example `UserDto` has no password hash).

```
read_model/src
├── dto/                    AgentDto, UserDto, ProductDto
├── query_request/          Get*ByIdQuery, Get*PagedListQuery
├── queries/                IAgentQueryService, IUserQueryService, IProductQueryService
├── base_query_service/     IBaseQueryService + QueryError
├── base_query_entity.rs    IBaseQueryEntity (id / timestamps)
├── mapper.rs               IMapper<T>
└── queryable/              PaginationParams, PaginatedResult, project_to
```

| Piece | Role |
|---|---|
| DTO | Serialize-ready read shape. Implements `IBaseQueryEntity`. |
| Query request | Input of a query (path/query-string params). |
| Query service trait | `find_by_id` plus list/filter methods. |
| `IMapper` | Row → DTO projection. |
| `PaginatedResult<T>` | `{ items, total_count, page_number, page_size }` with `project_to()`. |
| `QueryError` | `NotFound`, `Forbidden`, `Mapping`, `Database`. |

Query requests currently carry `page_number`, `page_size`, and optional `disable_paging`.

### `query_handler` — read adapters

Depends only on `read_model`. Owns **one** read database connection, initialized by `query_handler::add_query_handler()`.

```
query_handler/src
├── db_context.rs              read pool (DATABASE_URL)
├── entity_configurations/     SeaORM read models + IMapper → DTO
│   ├── user.rs                (no password_hash column selected)
│   ├── product.rs
│   └── agent.rs
├── query_service/pg/          PostgreSQL query services
│   ├── user_query_service.rs  → UserQueryService
│   ├── product_query_service.rs
│   └── agent_query_service.rs
└── queryable/pagination.rs    IPaginationExtension for SeaORM Select
```

Read-side `entity_configurations` are **mappers, not write configs**. They only `find` / `filter` / page and implement `IMapper<Dto>`. They never insert or update.

`base_query_service!` implements `IBaseQueryService::find_by_id`. List methods use `IPaginationExtension::paged_list`, then `PaginatedResult::project_to()` to map rows into DTOs.

Public exports:

```rust
pub use query_service::pg as query_services;
// query_services::{UserQueryService, ProductQueryService, AgentQueryService}
```

---

## Request flows

### Command (write)

Example: `POST /api/v1/admin/agents` → create agent.

```
HTTP JSON
  → api controller (JsonParam<CreateAgentCommand>)
  → application::create_agent_command::handle::<AgentRepository>
       Agent::new(name, system_prompt)          // domain factory, UUID v7
       IAgentRepository::create(&agent)
  → infrastructure::PGAgentRepository
       Agent → SeaORM ActiveModel
       insert on write db_context
  → Uuid
  → ApiResponse::created
```

Example: `PATCH /api/v1/product` → update product.

```
JWT claims → CurrentUserService.user_id()
  → UpdateProductCommand
  → IProductRepository::find_by_id_user_id(id, user_id)   // ownership check
  → Product::update(...)
  → IProductRepository::update(&mut product)              // touch + persist
```

### Query (read)

Example: `GET /api/v1/product?page_number=&page_size=`

```
Query string → GetProductPagedListQuery
  → application::get_products_list_query::handle::<ProductQueryService>
  → IProductQueryService::paged_list(request, user_id)
  → query_handler::PGProductQueryService
       SeaORM select + user_id filter
       IPaginationExtension::paged_list
       project_to::<ProductDto>()
  → PaginatedResult<ProductDto>
  → ApiResponse::ok
```

Queries never load a domain aggregate. The read mapper can omit write-only fields (password hash, refresh token, …).

### Authentication

```
Authorization: Bearer <jwt>
  → api::middlewares::authorization::auth
       decode with JwtService::get_secret()
       insert Claims into request extensions
  → optional require_roles(State<Vec<UserRoleType>>)
  → CurrentUserRequest extractor
       CurrentUserService { claims } → user_id()
```

Public routes: `POST /api/v1/user/register`, `POST /api/v1/user/login`.  
Admin routes also require `Claims.role == Admin`.

---

## Error mapping

Failures stay typed until the HTTP edge.

```
RepositoryError          QueryError
      │                       │
      ▼                       │
CommandError                  │
      │                       │
      └──────────┬────────────┘
                 ▼
              ApiError
                 ▼
        HTTP status + { "error": "..." }
```

| Domain / app error | HTTP |
|---|---|
| `NotFound` | 404 |
| `Validation` | 400 |
| `Unauthorized` | 401 |
| `Forbidden` | 403 |
| `Conflict` | 409 |
| `ExternalService` | 503 |
| `Internal` / `Database` / `Mapping` | 500 |

`JsonParam<T>` turns malformed JSON into `ApiError::Validation`. Successful handlers wrap payloads in `ApiResponse` (`200`, `201`, or `204`).

---

## Composition root

`api` is the only crate that knows every layer. Startup in `crates/api/src/main.rs`:

1. `AppState::new()` — shared Axum state.
2. `application::add_application()` — load `.env`, register Redis cache.
3. `infrastructure::add_infrastructure()` — open the write PostgreSQL pool.
4. `query_handler::add_query_handler()` — open the read PostgreSQL pool.
5. `inject_routers(state)` — nest `/api/v1/{admin,user,product}`.
6. CORS + listen on `0.0.0.0:5000`.

Both DB contexts currently use `DATABASE_URL`. They are separate `OnceLock` pools so the read stack can later switch to a replica or a dedicated read model database by changing only `query_handler::db_context`.

---

## Domain map

```
User 1 ──────── * Product
  │
  │ 1
  │
  * Token
  │
  * RoleAssignment * ──── 1 Role
                              │
                              * Tool * ──── 1 Agent
```

- A **user** owns products and tokens, and has a platform `UserRoleType` (`admin` / `user`) used for HTTP authorization.
- A **role** is an agent-facing capability set (distinct from `UserRoleType`).
- A **tool** is attached to one role and one agent. Its behavior is a `ToolConfiguration` value object stored as JSON.

---

## Persistence

### Write tables (infrastructure entity configs)

| Table | Aggregate / entity | Notes |
|---|---|---|
| `users` | `User` | Unique email, role as text enum |
| `tokens` | `Token` | Access + refresh, expiry |
| `products` | `Product` | FK `user_id` |
| `agents` | `Agent` | Name + system prompt |
| `tools` | `Tool` | `role_id`, `agent_id`, JSON `configuration` |

### Read projections (query_handler mappers)

Same physical tables today, different SeaORM models: only columns needed for DTOs are selected. `users` on the read side does not include `password_hash`.

### Migrations

`migration/` is a SeaORM CLI crate (not a workspace member):

| Migration | Creates |
|---|---|
| `m20260630_084121_init` | `users`, `tokens`, `products`, `api_keys` |
| `m20260722_093719_ai_models` | `ai_models` |

Schema evolution for agents / roles / tools is still catching up with the domain model. Run the migrator from `migration/`:

```bash
cargo run -- up
```

---

## Project structure

```
clean-architecture-ddd-rust/
├── Cargo.toml                      workspace (resolver 3)
├── docker-compose.yaml             api + postgres + redis + pgadmin
├── crates/
│   ├── api/                        binary crate — presentation
│   │   ├── Dockerfile
│   │   └── src/
│   │       ├── main.rs
│   │       ├── lib.rs
│   │       ├── state.rs
│   │       ├── controllers/
│   │       │   ├── user_controller.rs
│   │       │   ├── product_controller.rs
│   │       │   └── admin_controllers/
│   │       │       ├── users_controller.rs
│   │       │       └── agent_controller.rs
│   │       └── middlewares/
│   ├── application/                lib — features + services
│   │   └── src/
│   │       ├── features/{users,products,agents}/{commands,queries}
│   │       ├── services/{jwt,current_user,cache,app_config}
│   │       └── common/error.rs
│   ├── domain/                     lib — write model
│   │   └── src/{aggregate_root,base_repository,models}
│   ├── infrastructure/             lib — write adapters (one DB)
│   │   └── src/{db_context,entity_configurations,repository/pg}
│   ├── read_model/                 lib — read contracts
│   │   └── src/{dto,query_request,queries,queryable,mapper}
│   └── query_handler/              lib — read adapters (one DB)
│       └── src/{db_context,entity_configurations,query_service/pg}
└── migration/                      SeaORM migrator (outside workspace)
```

---

## Features

### Users

| Use case | Type | Handler |
|---|---|---|
| Register | command | `register_user_command` — hash password (Argon2), persist user + token, issue JWT |
| Login | command | `login_user_command` — verify hash, persist token, issue JWT |
| Logout | command | `logout_user_command` |
| Change password | command | `change_password_command` |
| Update profile | command | `update_user_command` |
| Get profile | query | `get_user_query` |
| Admin: list users | query | `get_users_list_query` |
| Admin: update by id | command | `update_user_by_id_command` |
| Admin: delete by id | command | `delete_user_by_id_command` |
| Admin: impersonate | command | `login_into_user_command` |

Register assigns `UserRoleType::User`. Auth responses (`AuthResponse` / `UserResponse`) live in the application users feature, not in `read_model`.

### Products

| Use case | Type | Notes |
|---|---|---|
| Create / update / delete | command | Scoped by current `user_id` |
| Get by id / paged list | query | Filtered by `user_id` in the query service |

### Agents

| Use case | Type | Notes |
|---|---|---|
| Create / update / delete | command | Admin-intended; uses `IAgentRepository` |
| Get by id / paged list | query | Uses `IAgentQueryService` |

---

## HTTP surface

Base URL: `http://localhost:5000/api/v1`

Authenticated routes expect `Authorization: Bearer <token>`.

### User

| Method | Path | Auth | Body / query |
|---|---|---|---|
| `POST` | `/user/register` | public | `{ first_name, last_name, email, password, is_active }` |
| `POST` | `/user/login` | public | `{ email, password }` |
| `GET` | `/user/` | JWT | current profile (`UserDto`) |
| `POST` | `/user/logout` | JWT | — |
| `POST` | `/user/change_password` | JWT | `{ old_password, new_password }` |
| `POST` | `/user/update` | JWT | `{ first_name?, last_name?, email? }` |
| `GET` | `/user/id` | JWT | current user id |

Register / login return `{ user, token, refresh_token }`.

### Product (JWT)

| Method | Path | Body / query |
|---|---|---|
| `POST` | `/product` | `{ name, price }` → `201` + id |
| `PATCH` | `/product` | `{ id, name, price, is_active }` |
| `DELETE` | `/product` | `{ id }` → `204` |
| `GET` | `/product/{id}` | `ProductDto` |
| `GET` | `/product?page_number=&page_size=` | `PaginatedResult<ProductDto>` |

### Admin users (JWT + Admin)

| Method | Path | Body / query |
|---|---|---|
| `GET` | `/admin/users?page_number=&page_size=` | paged `UserDto` |
| `PATCH` | `/admin/users` | `{ id, first_name?, last_name?, email? }` |
| `DELETE` | `/admin/users` | `{ id }` → `204` |
| `POST` | `/admin/users/login/{id}` | impersonation tokens |

### Agents (implemented, not yet mounted)

`IAgentController` defines admin-gated routes for create / update / delete / get / paged list. Wire it with `.nest("/agents", Router::new().agent_router())` on `admin_router` when exposing them.

---

## Technology stack

| Concern | Choice |
|---|---|
| HTTP | Axum 0.8 |
| Runtime | Tokio |
| Write/read ORM | SeaORM 1.1 (sqlx + PostgreSQL) |
| Database | PostgreSQL |
| Cache | Redis |
| Auth | JWT (`jsonwebtoken`) + Argon2 |
| IDs | UUID v7 |
| Serialization | Serde |
| Language | Rust edition 2024 |

Workspace release profile: `opt-level = 3`, fat LTO, `codegen-units = 1`, `panic = abort`, stripped symbols.

---

## How to add a feature

Follow the existing agent/product split. Do not put SeaORM types in `application` or HTTP types in `domain`.

**Write side**

1. Aggregate + `IXxxRepository` in `domain`.
2. SeaORM entity + `From` mappings in `infrastructure/entity_configurations`.
3. `PGXxxRepository` using `base_command!` / `base_query!`.
4. Command structs + `handle<R: IXxxRepository>` in `application/features/<name>/commands`.
5. Controller method that calls `handle::<XxxRepository>(...)`.

**Read side**

1. DTO, query request, and `IXxxQueryService` in `read_model`.
2. Read mapper (`IMapper<Dto>`) in `query_handler/entity_configurations` — read-only, no ActiveModel writes.
3. `PGXxxQueryService` using `base_query_service!` + pagination.
4. Query `handle<S: IXxxQueryService>` in `application/features/<name>/queries`.
5. Controller method that calls `handle::<XxxQueryService>(...)`.

Keep write mappings and read mappings in their own crates even if they hit the same table. That is what lets each side own one database.

---

## Setup

### Prerequisites

- Rust (stable, edition 2024)
- PostgreSQL 12+
- Redis (required at startup — `application::add_application()` registers the cache)
- Optional: Docker + Docker Compose

### Environment

Copy `.env.example` and set at least:

| Variable | Required | Purpose |
|---|---|---|
| `DATABASE_URL` | yes | Write and read PostgreSQL URL |
| `REDIS_URL` | yes | Cache (`redis://…`) |
| `JWT_SECRET` | yes | Access-token signing key |
| `JWT_EXP` | yes | Access-token lifetime in seconds |
| `PUBLIC_BASE_URL` | yes | Public API base URL |
| `OPENAI_API_KEY` / `DEEPSEEK_API_KEY` / `IVIRA_API_KEY` | reserved | External model providers |

### Local

```bash
# from migration/
cargo run -- up

# from repo root
cargo run --release -p api
```

Server: `http://0.0.0.0:5000`

### Docker

```bash
docker compose up --build
```

Compose starts `api` (`5000`), PostgreSQL (`5432`), Redis (`6379`), and pgAdmin (`5050`).

### Tests

```bash
cargo test
cargo test -p application
cargo test -p domain
```

---

## Layer rules (do not break)

1. `domain` and `read_model` do not depend on each other or on any outer crate.
2. `infrastructure` depends only on `domain` and owns only the write database.
3. `query_handler` depends only on `read_model` and owns only the read database.
4. `application` orchestrates both sides through traits. It may depend on adapter crates for wiring types, but use-case functions are generic over traits.
5. `api` is the composition root: HTTP, middleware, and choosing `PG*` adapters.
6. Commands mutate aggregates. Queries return DTOs. Do not load an aggregate to answer a list/get-by-id query.
7. Write entity configurations map aggregates ↔ rows. Read entity configurations only map rows → DTOs.
