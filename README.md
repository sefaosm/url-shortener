# 🔗 URL Shortener

A production-quality URL shortener service built in Rust, featuring Redis caching, async click tracking, rate limiting, and background maintenance tasks.

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
| Rate Limiting | tower_governor |
| Logging | tracing + tower_http |

## Architecture

Clean layered architecture:
Routes (HTTP handling) → Services (business logic) → Repositories (database operations)

### Key Design Decisions

- **302 Found** for redirects — no browser caching, enables accurate click tracking
- **Redis cache-first** strategy — cache hit skips DB lookup for redirects
- **Redis click counter buffering** — click counts accumulate in Redis, flushed to PostgreSQL every 30 seconds
- **Async click tracking** — `tokio::spawn` fire-and-forget, doesn't block redirect response
- **Graceful cache degradation** — Redis failure never breaks the app
- **Soft delete** — URLs are deactivated, not removed from DB
- **nanoid + base62** for short code generation with collision retry logic
- **Three-tier rate limiting** — separate limits for URL creation, API queries, and redirects
- **Background tasks** — expired URL cleanup (hourly) and click count flush (every 30s)

## Features

### Phase 1 & 2 — Foundation & Core ✅

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

### Phase 3 — Enhanced Features ✅

- [x] URL statistics endpoint with recent click details
- [x] URL listing with pagination
- [x] Soft delete with cache invalidation

### Phase 4 — Production Hardening ✅

- [x] Rate limiting with three separate tiers (shorten / API / redirect)
- [x] Structured request/response logging with TraceLayer
- [x] CORS middleware
- [x] Background task: expired URL cleanup (runs hourly)
- [x] Background task: Redis click counter flush to PostgreSQL (runs every 30s)

### Planned

- [ ] **Phase 5:** OpenAPI/Swagger docs, GitHub Actions CI/CD pipeline
- [ ] **Phase 6:** Integration tests

## Project Structure

    src/
    ├── config/
    │   └── mod.rs              # AppConfig from environment variables
    ├── dto/
    │   ├── mod.rs
    │   ├── request.rs          # ShortenRequest, PaginationParams
    │   └── response.rs         # HealthResponse, ShortenResponse, UrlStatsResponse, etc.
    ├── errors/
    │   ├── mod.rs
    │   └── app_error.rs        # Central AppError enum with IntoResponse
    ├── models/
    │   ├── mod.rs
    │   ├── url.rs              # Url model
    │   └── click_event.rs      # ClickEvent model
    ├── repositories/
    │   ├── mod.rs
    │   ├── url_repository.rs   # URL CRUD + bulk operations
    │   └── click_repository.rs # Click event recording & queries
    ├── routes/
    │   ├── mod.rs              # Router setup with rate limiters & middleware
    │   ├── health.rs           # GET /api/v1/health
    │   ├── shorten.rs          # POST /api/v1/shorten
    │   ├── redirect.rs         # GET /:code → 302 redirect
    │   ├── stats.rs            # GET /api/v1/stats/:code
    │   └── urls.rs             # GET /api/v1/urls, DELETE /api/v1/urls/:code
    ├── services/
    │   ├── mod.rs
    │   ├── code_generator.rs   # nanoid generation + validation
    │   ├── cache_service.rs    # Redis get/set/delete + click counter operations
    │   └── url_service.rs      # Core business logic
    ├── tasks/
    │   ├── mod.rs
    │   ├── cleanup.rs          # Expired URL cleanup task
    │   └── click_flush.rs      # Redis → PostgreSQL click count flush task
    ├── lib.rs                  # AppState
    └── main.rs                 # Entry point with graceful shutdown

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

# Shorten with expiration (24 hours)
curl -X POST http://localhost:3000/api/v1/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "https://example.com", "expires_in_hours": 24}'

# Redirect (follow with browser or check headers)
curl -I http://localhost:3000/axum-repo

# Get URL statistics
curl http://localhost:3000/api/v1/stats/axum-repo

# List all URLs (with pagination)
curl "http://localhost:3000/api/v1/urls?page=1&per_page=10"

# Soft delete a URL
curl -X DELETE http://localhost:3000/api/v1/urls/axum-repo
```

## API Endpoints

| Method | Path | Description | Status Codes |
|--------|------|-------------|--------------|
| GET | `/api/v1/health` | Health check | 200 |
| POST | `/api/v1/shorten` | Create short URL | 201, 409, 422 |
| GET | `/:code` | Redirect to original URL | 302, 404, 410 |
| GET | `/api/v1/stats/:code` | URL statistics with recent clicks | 200, 404 |
| GET | `/api/v1/urls` | List all URLs (paginated) | 200 |
| DELETE | `/api/v1/urls/:code` | Soft delete a URL | 204, 404 |

## Rate Limits

| Scope | Rate | Burst |
|-------|------|-------|
| URL Creation (`POST /shorten`) | 6 req/sec | 10 |
| API Queries (stats, list, delete) | 2 req/sec | 30 |
| Redirects (`GET /:code`) | 1 req/sec | 60 |

Exceeding the limit returns `429 Too Many Requests`.

## License

MIT
