# Smarter Home - GitHub Copilot Instructions

Smarter Home is a multi-component energy analytics platform consisting of a Rust API backend, Next.js frontend, Tauri desktop application, and TimescaleDB database for time-series energy data.

**Always reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.**

## Working Effectively

### Essential Setup and Build Commands
**NEVER CANCEL any build or test commands** - builds can take significant time but must complete.

#### Environment Setup
```bash
# Copy and configure environment
cp .env.example .env
# Update MIGRATION_DIR=migrations (not src/backend/internal/database/migrations)
# Update DATABASE_URL=postgresql://postgres:postgres@localhost:5432/smarter-home (remove the !)
```

#### Database Setup
```bash
# Start database only
docker compose up -d database
# TAKES: 2-3 minutes for initial TimescaleDB image download. NEVER CANCEL.

# Install goose for migrations
go install github.com/pressly/goose/v3/cmd/goose@latest
export PATH=$PATH:~/go/bin

# Run migrations  
GOOSE_DRIVER=postgres GOOSE_DBSTRING="postgresql://postgres:postgres@localhost:5432/smarter-home" goose -dir=migrations up
```

#### Rust Backend Build
```bash
# Build backend (CRITICAL: use SQLX_OFFLINE=true)
export SQLX_OFFLINE=true
cargo build --release
# TAKES: 7-10 minutes. NEVER CANCEL. Set timeout to 15+ minutes.
```

#### Frontend Build
```bash
cd src/frontend
npm install
# TAKES: 15-30 seconds

npm run build  
# TAKES: 1-2 minutes. NEVER CANCEL. Set timeout to 5+ minutes.
```

#### Desktop Application
```bash
# Install pnpm if not available
npm install -g pnpm

cd src/desktop
pnpm install
pnpm build
# TAKES: 30-60 seconds

# Desktop full build requires GTK system libraries
pnpm tauri build --no-bundle
# FAILS in CI environments due to missing GTK libraries - this is expected
# Document as: "Desktop build requires GTK libraries not available in CI environments"
```

### Running Applications

#### Backend API
```bash
# Run from repository root
cd /path/to/smarter-home
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/smarter-home target/release/api
# STARTS: Listening on 0.0.0.0:3001
# TEST: curl http://localhost:3001/health should return "healthy"
```

#### Frontend Development Server
```bash
cd src/frontend  
npm run dev
# STARTS: Available at http://localhost:3000
# TAKES: 2-3 seconds to start. Ready when you see "Ready in [time]ms"
# TEST: curl http://localhost:3000 should return HTML with "Smarter Home" title
```

## Validation Requirements

### MANUAL VALIDATION REQUIREMENT
**After building and running applications, you MUST test actual functionality:**

1. **API Health Check**: `curl http://localhost:3001/health` → should return `healthy`
2. **Frontend Loading**: Visit `http://localhost:3000` → should display "Smarter Home" dashboard UI
3. **Database Connection**: API logs should show "Connected to database" when started
4. **Migration Status**: `goose -dir=migrations status` should show applied migrations

### Build Time Expectations
- **Rust Backend**: 7-10 minutes (NEVER CANCEL, set 15+ minute timeout)
- **Frontend**: 1-2 minutes (NEVER CANCEL, set 5+ minute timeout)  
- **Desktop Frontend**: 30-60 seconds
- **Database Startup**: 2-3 minutes for initial pull
- **Migration Apply**: 5-10 seconds

## Troubleshooting and Known Issues

### Critical Build Requirements
- **Rust**: MUST use `SQLX_OFFLINE=true` or builds fail with database connection errors
- **Database Password**: Use `postgres:postgres@localhost` NOT `postgres:postgres!@database` 
- **Migration Format**: Migrations must have proper goose headers (`-- +goose Up`, etc.)
- **Workspace Config**: Desktop Tauri app must be excluded from main workspace

### Common Failures and Solutions
- **SQLX compilation errors**: Ensure `export SQLX_OFFLINE=true` before any Rust builds
- **Password authentication failed**: Check DATABASE_URL has correct password format
- **Migration parse errors**: Verify goose headers are present in migration files
- **Tauri workspace conflicts**: Ensure `src/desktop/src-tauri` is in workspace.exclude
- **Desktop build failures**: GTK system libraries missing - expected in CI environments

### Docker Development (Limited)
```bash
# Database only (reliable)
docker compose up -d database

# Full stack build FAILS due to network restrictions in CI environments  
# Document as: "Docker builds fail in CI due to network limitations"
```

## Project Structure

### Key Components
- **`src/services/api/`**: Rust Axum backend with TimescaleDB integration
- **`src/frontend/`**: Next.js 15 dashboard with Tailwind CSS and Auth.js
- **`src/desktop/`**: Tauri React desktop application
- **`src/lib/`**: Shared Rust libraries (lib-models, lib-utils)
- **`migrations/`**: Goose database migrations for power metrics schema
- **`Cargo.toml`**: Workspace configuration (excludes desktop)

### Important Files
- **`.env`**: Database connection and app configuration
- **`Makefile`**: Legacy commands (references non-existent go-backend)
- **`docker-compose.yml`**: Database and backend service definitions
- **`src/frontend/package.json`**: Frontend dependencies and scripts

## Linting and Testing

### Frontend Linting
```bash
cd src/frontend
npm run lint
# NOTE: First run will prompt to configure ESLint - select "Strict (recommended)"
# ALWAYS run before committing frontend changes
```

### Rust Formatting  
```bash
cargo fmt --all
# ALWAYS run before committing Rust changes
```

### No Automated Tests
The repository currently has no test suite configured. Focus on manual validation scenarios above.

## Development Workflow

### Making Changes
1. **ALWAYS** run the bootstrap/build sequence first to validate current state
2. Make targeted changes to specific components
3. Run relevant build commands with proper timeouts
4. **ALWAYS** perform manual validation scenarios
5. Run appropriate linting before committing
6. Test both development and production build modes where applicable

### Component Interaction
- Frontend communicates with backend API on port 3001
- Backend connects to TimescaleDB for power metrics storage  
- Desktop app is standalone React/Tauri application
- All components share configuration via .env file

## Time Series Data Schema
The application focuses on power/energy metrics with this schema:
- `power_metrics` table with timestamps, power consumption, solar production
- TimescaleDB hypertables for efficient time-series queries
- Currency-aware cost calculations since midnight

Always validate that database schema changes are compatible with existing power metrics structure.