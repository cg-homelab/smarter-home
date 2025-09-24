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
- **sqlx-cli**: Database migration tool (installed via `cargo install sqlx-cli`)

### Critical Build Order
**ALWAYS** follow this exact sequence to avoid build failures:

1. **System Validation**: `make validate-system` 
2. **Environment Setup**: Copy `.env.example` to `.env` and configure variables
3. **Dependencies Installation**: `make install-dependencies`
4. **Database Setup**: `make start-database` then `make db-up`
5. **Rust Build**: `cargo build` (from repository root)

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
cd src/services/frontend

# Install dependencies (automatically done by make install-dependencies)
npm install

# Development server
npm run dev

# Production build
npm run build

# Start production server
npm run start

# Linting
npm run lint
```

#### Desktop App (Tauri)
```bash
cd src/services/desktop

# Install dependencies (automatically done by make install-dependencies)
npm install

# Development mode
npm run dev

# Build desktop app
npm run build

# Tauri commands
npm run tauri dev    # Run in Tauri dev mode
npm run tauri build  # Build native app

# Or use Makefile
make start-desktop   # Run desktop app in development mode
```

#### Database Operations (SQLx-based)
```bash
# Start database only
make start-database

# Check migration status
make db-status

# Run all migrations
make db-up

# Reset database (rollback all)
make db-reset

# Rollback last migration
make db-down

# Create new migration (prompts for name)
make db-mig-create

# For offline mode (prepare SQL queries)
make db-prepare-offline
```

#### Docker Operations
```bash
# Full stack with Docker
make start-docker

# Stop all services
make stop-docker

# Start just database
make start-database
```

### Known Issues and Workarounds

#### Frontend Dependency Vulnerabilities
**ISSUE**: npm audit shows vulnerabilities in frontend dependencies
**BEHAVIOR**: This is common with frontend dependencies and doesn't affect functionality. Run `npm audit fix` if needed.

#### Port Conflicts
**ISSUE**: Frontend dev server may find port 3000 in use
**BEHAVIOR**: Next.js automatically tries port 3001, 3002, etc. This is normal behavior.

#### Database Connection for Development
**ISSUE**: Database URL needs different hostnames for Docker vs. host development
**SOLUTION**: 
- For Docker: `postgresql://postgres:postgres@database:5432/smarter-home`
- For host development: `postgresql://postgres:postgres@localhost:5432/smarter-home`

#### Build Timing
- Frontend build: ~30-60 seconds
- Rust build (first time): ~2-3 minutes  
- Rust build (incremental): ~10-30 seconds
- Desktop build: ~30-60 seconds
- Docker build: ~3-5 minutes
- Database Docker pull: ~2-3 minutes (first time)
- sqlx-cli installation: ~2-3 minutes (first time)

## Project Layout

### Architecture Overview
```
smarter-home/
├── src/
│   ├── services/api/          # Rust backend API server
│   ├── services/              # Shared Rust libraries
│   │   ├── api/               # Data models
│   │   ├── frontend/          # Next.js web application
│   │   └── desktop/           # Tauri desktop application
│   └── lib/                   # Shared Rust libraries
│       ├── lib-models/        # Data models
│       └── lib-utils/         # Utility functions
├── migrations/                # Database schema migrations (SQLx)
├── docker-compose.yml         # Multi-container setup
├── Dockerfile.api            # Backend container definition
└── Makefile                  # Build automation (modernized)
```

### Key Configuration Files
- **Backend**: `src/services/api/Cargo.toml`, `Cargo.toml` (workspace)
- **Frontend**: `src/services/frontend/package.json`, `next.config.ts`, `tailwind.config.ts`
- **Desktop**: `src/services/desktop/package.json`, `vite.config.ts`, `src-tauri/tauri.conf.json`
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
- **DB_HOST/DB_PORT/DB_NAME/DB_USERNAME/DB_PASSWORD**: Database connection details
- **AUTH_SECRET**: NextAuth.js secret key
- **AUTH_GITHUB_ID/SECRET**: GitHub OAuth credentials

**CRITICAL**: The `.env.example` file provides correct defaults, but you may need to adjust `DATABASE_URL` based on your development setup:
- For Docker development: use `database` as hostname
- For local development: use `localhost` as hostname

### Dependencies and Tools
- **TimescaleDB**: PostgreSQL extension for time-series data (required for database)
- **sqlx-cli**: Database migration tool (replaces older goose-based setup)
- **Vercel Analytics**: Integrated in frontend for usage tracking
- **Auth.js**: Authentication system for the frontend
- **Shadcn UI**: Component library for the frontend

### Validation Steps
1. **System Prerequisites**: `make validate-system` should pass
2. **Database Health**: `docker compose ps` should show healthy database
3. **API Health**: `cargo build` should complete successfully
4. **Frontend**: Navigate to `http://localhost:3000` should load dashboard
5. **Build Artifacts**: Check for `.next/` (frontend) and `target/` (Rust) directories

### Repository Root Files
- `README.md`: Basic project description 
- `Cargo.toml`: Rust workspace configuration with 3 members
- `Cargo.lock`: Rust dependency lock file
- `Makefile`: Build automation (modernized, sqlx-based)
- `docker-compose.yml`: Multi-service container setup
- `Dockerfile.api`: Rust API container definition
- `.env.example`: Environment variable template
- `.gitignore`: Standard ignore patterns for Rust/Node.js
- `.dockerignore`: Docker build context exclusions

### Important Source Files
- **API Entry Point**: `src/services/api/src/main.rs` - Axum server setup
- **Frontend Layout**: `src/services/frontend/app/layout.tsx` - Next.js root layout  
- **Desktop Entry**: `src/services/desktop/src/main.tsx` - React app entry
- **Database Schema**: `migrations/0001_init.sql` - TimescaleDB table setup
- **Shared Models**: `src/lib/lib-models/` - Common data structures
- **Utilities**: `src/lib/lib-utils/` - Shared utility functions

## Instructions for Coding Agents

**TRUST THESE INSTRUCTIONS** - Only search for additional information if these instructions are incomplete or proven incorrect.

When working on this repository:

1. **Always start with `make validate-system`** to ensure prerequisites
2. **Use `make install-dependencies`** to install all required dependencies
3. **Copy `.env.example` to `.env`** before any database operations
4. **Use Docker Compose v2 syntax** (`docker compose`, not `docker-compose`)
5. **Use exact build order specified above** to prevent failures
6. **Test both dev and production builds** after making changes
7. **Verify Docker container health** when using containerized services

### Modern Makefile Commands
The project has been modernized to use sqlx-cli instead of goose. Available commands:

**System & Dependencies:**
- `make validate-system` - Check prerequisites (Cargo, npm)
- `make install-dependencies` - Install all project dependencies

**Running Services:**
- `make start-docker` - Start full stack with Docker
- `make start-database` - Start only the database
- `make start-desktop` - Start desktop app in dev mode
- `make stop-docker` - Stop Docker services

**Database Operations:**
- `make db-status` - Check migration status
- `make db-up` - Apply all migrations
- `make db-down` - Rollback last migration  
- `make db-reset` - Rollback all migrations
- `make db-mig-create` - Create new migration
- `make db-prepare-offline` - Prepare offline SQL queries

### Common Development Tasks
- **New API endpoint**: Modify `src/services/api/src/routes/`
- **Frontend UI changes**: Work in `src/services/frontend/app/` or `src/services/frontend/components/`
- **Database changes**: Create new migration with `make db-mig-create`
- **Styling**: Use TailwindCSS classes, extend in `tailwind.config.ts`
- **Desktop features**: Modify `src/services/desktop/src/` React components
- **Shared models**: Add to `src/lib/lib-models/`
- **Utility functions**: Add to `src/lib/lib-utils/`

### Migration from Legacy Setup
If you encounter references to older tooling:
- **goose** → **sqlx-cli** (database migrations)
- **Go backend** → **Rust backend** (no Go code in current version)
- Manual dependency installation → `make install-dependencies`
