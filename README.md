# 🔗 URL Shortener

A production-quality URL shortener service built in Rust.

## Tech Stack

| Component | Technology |
|-----------|------------|
| Language | Rust |
| Web Framework | Axum 0.7 |
| Database | PostgreSQL 16 |
| Cache | Redis 7 |
| ORM | SQLx (compile-time verified) |
| Runtime | Tokio |
| Containerization | Docker Compose |

## Architecture

Clean layered architecture:
Routes (HTTP handling) → Services (business logic) → Repositories (database operations)

### Key Design Decisions

- **302 Found** for redirects — no browser caching, enables click tracking
- **Redis cache-first** strategy — cache hit skips DB lookup for redirects
- **Async click tracking** — `tokio::spawn` fire-and-forget, doesn't block redirect response
- **Graceful cache degradation** — Redis failure never breaks the app
- **Soft delete** — URLs are deactivated, not removed from DB
- **nanoid + base62** for short code generation with collision retry logic

## Features

### Completed (Phase 1 & 2) ✅

- [x] Health check endpoint with DB & Redis status
- [x] Create short URL (random code generation)
- [x] Create short URL (custom code with validation)
- [x] 302 redirect with Location header
- [x] Redis caching with TTL (1 hour)
- [x] Click tracking (ip, user-agent, referer)
- [x] URL expiration support (410 Gone)
- [x] Duplicate custom code detection (409 Conflict)
- [x] URL validation (422 for invalid input)
- [x] Central error handling with structured JSON responses
- [x] Docker Compose setup (PostgreSQL + Redis)
- [x] Database migrations

### Planned

- [ ] **Phase 3:** URL statistics endpoint, URL listing with pagination, soft delete
- [ ] **Phase 4:** Rate limiting, logging refinement, background cleanup tasks
- [ ] **Phase 5:** OpenAPI/Swagger docs, CI/CD pipeline
- [ ] **Phase 6:** Integration tests

## Project Structure
```
src/
├── config/
│   └── mod.rs              # AppConfig from environment variables
├── dto/
│   ├── mod.rs
│   ├── request.rs          # ShortenRequest
│   └── response.rs         # HealthResponse, ShortenResponse
├── errors/
│   ├── mod.rs
│   └── app_error.rs        # Central AppError enum
├── models/
│   ├── mod.rs
│   ├── url.rs              # Url model
│   └── click_event.rs      # ClickEvent model
├── repositories/
│   ├── mod.rs
│   ├── url_repository.rs
│   └── click_repository.rs
├── routes/
│   ├── mod.rs
│   ├── health.rs
│   ├── shorten.rs
│   └── redirect.rs
├── services/
│   ├── mod.rs
│   ├── code_generator.rs
│   ├── cache_service.rs
│   └── url_service.rs
├── lib.rs                  # AppState
└── main.rs                 # Entry point
```
## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Docker Desktop](https://www.docker.com/products/docker-desktop/)

### Run

```bash
# Start PostgreSQL and Redis
docker compose up -d postgres redis

# Copy environment variables
cp .env.example .env

# Run the application (applies migrations automatically)
cargo run
```

### Test Endpoints

```bash
# Health check
curl http://localhost:3000/api/v1/health

# Shorten a URL
curl -X POST http://localhost:3000/api/v1/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "https://www.rust-lang.org"}'

# Shorten with custom code
curl -X POST http://localhost:3000/api/v1/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "https://github.com/tokio-rs/axum", "custom_code": "axum-repo"}'

# Redirect (follow with browser or check headers)
curl -I http://localhost:3000/axum-repo
```

## API Endpoints

| Method | Path | Description | Status |
|--------|------|-------------|--------|
| GET | `/api/v1/health` | Health check | ✅ |
| POST | `/api/v1/shorten` | Create short URL | ✅ |
| GET | `/:code` | Redirect to original URL | ✅ |
| GET | `/api/v1/stats/:code` | URL statistics | 🔜 |
| GET | `/api/v1/urls` | List all URLs | 🔜 |
| DELETE | `/api/v1/urls/:code` | Soft delete URL | 🔜 |
