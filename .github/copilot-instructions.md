# GitHub Copilot Instructions for Smarter Home

## Repository Overview

**Smarter Home** is an energy monitoring and analytics system that provides data analytics for home energy usage. The project helps users understand their energy consumption patterns and make smarter choices about their energy usage.

### High Level Details
- **Project Type**: Energy monitoring system with web dashboard and desktop application
- **Repository Size**: Medium (multi-component architecture)
- **Languages**: Rust (backend), TypeScript/JavaScript (frontend), React (desktop app)
- **Target Runtime**: Docker containers, web browsers, desktop environments
- **Main Frameworks**: 
  - Backend: Axum (Rust web framework), SQLx (database), TimescaleDB
  - Frontend: Next.js 15 with App Router, TailwindCSS, Shadcn UI
  - Desktop: Tauri v2 with React and Vite
  - Database: TimescaleDB (PostgreSQL extension for time-series data)

## Build Instructions

### Prerequisites
Always ensure these tools are installed before building:
- **Rust**: Latest stable version with Cargo
- **Node.js**: v18+ with npm
- **Docker**: Latest version with Docker Compose v2
- **Go**: Required by some tooling (noted in Makefile, though unused in current codebase)

### Critical Build Order
**ALWAYS** follow this exact sequence to avoid build failures:

1. **System Validation**: `make validate-system` 
2. **Environment Setup**: Copy `.env.example` to `.env` and configure variables
3. **Database Setup**: `make docker-start-db` then `make db-up`
4. **Frontend Dependencies**: `cd src/frontend && npm install`
5. **Desktop Dependencies**: `cd src/desktop && npm install` (or `pnpm install`)
6. **Rust Build**: `cargo build` (from repository root)

### Core Commands

#### Backend (Rust API)
```bash
# Build backend
cargo build --release

# Run tests (minimal test suite exists)
cargo test

# Run API locally (requires database)
cargo run --bin api
```

#### Frontend (Next.js)
```bash
cd src/frontend

# Install dependencies - ALWAYS run before building
npm install

# Development server
npm run dev

# Production build
npm run build

# Start production server
npm run start

# Linting (requires initial ESLint configuration)
npm run lint
```

#### Desktop App (Tauri)
```bash
cd src/desktop

# Install dependencies (uses pnpm)
npm install  # or pnpm install

# Development mode
npm run dev

# Build desktop app
npm run build

# Tauri commands
npm run tauri dev    # Run in Tauri dev mode
npm run tauri build  # Build native app
```

#### Database Operations
```bash
# Start database only
make docker-start-db

# IMPORTANT: First install goose and fix .env file
go install github.com/pressly/goose/v3/cmd/goose@latest
export PATH=$HOME/go/bin:$PATH

# Update .env file: change MIGRATION_DIR to "migrations" and add DB_DRIVER=postgres

# Check migration status (after fixing .env)
make db-status

# Run all migrations
make db-up

# Reset database (rollback all)
make db-reset

# Create new migration
make db-mig-create <migration_name>

# Alternative: Use goose directly
goose postgres "postgres://postgres:postgres!@localhost:5432/smarter-home?sslmode=disable" -dir migrations up
```

#### Docker Operations
```bash
# Full stack with Docker
docker compose up --build

# Stop all services
docker compose down
```

### Known Issues and Workarounds

#### Critical Makefile Bug
**ISSUE**: `make install-dependencies` fails with "can't cd to src/go-backend"
**WORKAROUND**: The Makefile references a non-existent `src/go-backend` directory. Install dependencies manually:
1. `cd src/frontend && npm install`
2. `cd src/desktop && npm install`
3. `cargo build` (for Rust dependencies)

#### Database Migration Setup
**ISSUE**: Migration commands require correct environment setup and goose installation
**WORKAROUND**: 
1. Install goose: `go install github.com/pressly/goose/v3/cmd/goose@latest`
2. Add to PATH: `export PATH=$HOME/go/bin:$PATH`
3. Use correct connection format: `postgres://postgres:postgres!@localhost:5432/smarter-home?sslmode=disable`
4. Fix .env MIGRATION_DIR to point to `migrations` not `src/backend/internal/database/migrations`

#### Frontend ESLint Configuration
**ISSUE**: Running `npm run lint` prompts for ESLint configuration
**WORKAROUND**: Choose "Strict (recommended)" when prompted, or configure ESLint manually

#### Port Conflicts
**ISSUE**: Frontend dev server may find port 3000 in use
**BEHAVIOR**: Next.js automatically tries port 3001, 3002, etc. This is normal behavior.

#### Build Timing
- Frontend build: ~30-60 seconds
- Rust build (first time): ~2-3 minutes  
- Rust build (incremental): ~10-30 seconds
- Desktop build: ~30-60 seconds
- Docker build: ~3-5 minutes
- Database Docker pull: ~2-3 minutes (first time)

## Project Layout

### Architecture Overview
```
smarter-home/
├── src/
│   ├── services/api/          # Rust backend API server
│   ├── frontend/              # Next.js web application  
│   ├── desktop/               # Tauri desktop application
│   └── lib/                   # Shared Rust libraries
├── migrations/                # Database schema migrations
├── docker-compose.yml         # Multi-container setup
├── Dockerfile.api            # Backend container definition
└── Makefile                  # Build automation (has bugs)
```

### Key Configuration Files
- **Backend**: `src/services/api/Cargo.toml`, `Cargo.toml` (workspace)
- **Frontend**: `src/frontend/package.json`, `next.config.ts`, `tailwind.config.ts`
- **Desktop**: `src/desktop/package.json`, `vite.config.ts`, `src-tauri/tauri.conf.json`
- **Database**: `migrations/0001_init.sql`, `.env.example`
- **Docker**: `docker-compose.yml`, `Dockerfile.api`
- **CI/CD**: `.github/workflows/run-pr-tag-gen.yml`, `.github/workflows/run-release-deploy.yml`

### Continuous Integration
GitHub Actions workflows:
1. **PR Tag Generation** (`run-pr-tag-gen.yml`): Auto-tags releases on PR merge
2. **Release Deployment** (`run-release-deploy.yml`): Builds and pushes Docker images to Docker Hub

### Environment Variables
Copy `.env.example` to `.env` and configure:
- **BACKEND_PORT**: API server port (default: 3001)
- **FRONTEND_PORT**: Next.js port (default: 3000)  
- **DATABASE_URL**: PostgreSQL connection string
- **MIGRATION_DIR**: Should be `migrations` (NOT `src/backend/internal/database/migrations`)
- **DB_DRIVER**: Should be `postgres`
- **AUTH_SECRET**: NextAuth.js secret key
- **AUTH_GITHUB_ID/SECRET**: GitHub OAuth credentials

**CRITICAL**: After copying .env.example to .env, you must fix:
1. `MIGRATION_DIR=migrations` (remove the incorrect src/backend path)
2. Add `DB_DRIVER=postgres` 
3. For local development: `DATABASE_URL="postgres://postgres:postgres!@localhost:5432/smarter-home?sslmode=disable"`

### Dependencies Not Obvious from Structure
- **TimescaleDB**: PostgreSQL extension for time-series data (required for database)
- **Goose**: Database migration tool (installed via Go)
- **Air**: Rust development server (optional, for hot reload)
- **Vercel Analytics**: Integrated in frontend for usage tracking

### Validation Steps
1. **Database Health**: `docker compose ps` should show healthy database
2. **API Health**: `curl http://localhost:3001/health` (if health endpoint exists)
3. **Frontend**: Navigate to `http://localhost:3000` should load dashboard
4. **Build Artifacts**: Check for `.next/` (frontend) and `target/` (Rust) directories

### Repository Root Files
- `README.md`: Basic project description (minimal)
- `Cargo.toml`: Rust workspace configuration with 3 members
- `Cargo.lock`: Rust dependency lock file
- `Makefile`: Build automation (contains bugs with go-backend reference)
- `docker-compose.yml`: Multi-service container setup
- `Dockerfile.api`: Rust API container definition
- `.env.example`: Environment variable template
- `.gitignore`: Standard ignore patterns for Rust/Node.js
- `.dockerignore`: Docker build context exclusions

### Important Source Files
- **API Entry Point**: `src/services/api/src/main.rs` - Axum server setup
- **Frontend Layout**: `src/frontend/app/layout.tsx` - Next.js root layout  
- **Desktop Entry**: `src/desktop/src/main.tsx` - React app entry
- **Database Schema**: `migrations/0001_init.sql` - TimescaleDB table setup

## Instructions for Coding Agents

**TRUST THESE INSTRUCTIONS** - Only search for additional information if these instructions are incomplete or proven incorrect.

When working on this repository:

1. **Always start with `make validate-system`** to ensure prerequisites
2. **Copy `.env.example` to `.env`** before any database operations
3. **Use Docker Compose v2 syntax** (`docker compose`, not `docker-compose`)
4. **Install frontend dependencies first** before any frontend operations
5. **Avoid `make install-dependencies`** due to the go-backend bug
6. **Use exact build order specified above** to prevent failures
7. **Test both dev and production builds** after making changes
8. **Verify Docker container health** when using containerized services

For common tasks:
- **New API endpoint**: Modify `src/services/api/src/routes/`
- **Frontend UI changes**: Work in `src/frontend/app/` or `src/frontend/components/`
- **Database changes**: Create new migration with `make db-mig-create`
- **Styling**: Use TailwindCSS classes, extend in `tailwind.config.ts`
- **Desktop features**: Modify `src/desktop/src/` React components