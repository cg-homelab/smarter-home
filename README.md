# Smarter Home - Energy Monitoring and Analytics

A comprehensive energy monitoring and analytics system that helps homeowners understand their energy consumption patterns and make smarter decisions about energy usage.

## 🏠 Project Overview

Smarter Home provides:
- **Real-time energy monitoring** with TimescaleDB time-series database
- **Web dashboard** built with Next.js 16 and modern React components
- **RESTful API** powered by Rust and Axum framework
- **JWT + rotating refresh token flow** for secure API access
- **Docker containerization** for easy deployment

## 🏗️ Architecture

```
┌─────────────────┐    ┌─────────────────┐
│   Web Frontend  │    │   REST API      │
│   (Next.js)     │◄──►│   (Rust/Axum)   │
└─────────────────┘    └─────────────────┘
                              │
                     ┌─────────────────┐
                     │   TimescaleDB   │
                     │   (PostgreSQL)  │
                     └─────────────────┘
```

### Tech Stack

- **Backend**: Rust + Axum + SQLx + TimescaleDB
- **Frontend**: Next.js 16 + TypeScript + TailwindCSS + Shadcn UI
- **Database**: TimescaleDB (PostgreSQL with time-series extensions)
- **Containerization**: Docker + Docker Compose
- **Authentication**: Backend JWT access token + rotating refresh token in httpOnly cookies

## 🚀 Quick Start

### Prerequisites

Before starting, ensure you have:
- **Rust** (latest stable) - [Install here](https://rustup.rs/)
- **Node.js** v18+ - [Install here](https://nodejs.org/)
- **Bun** (latest stable) - [Install here](https://bun.sh)
- **Docker** with Docker Compose v2 - [Install here](https://docs.docker.com/get-docker/)
- **just** v1.48+ (task runner) - [Install here](https://just.systems/man/en/packages.html)

### macOS/Linux/Windows setup notes

Use one of these quick setup paths, then continue with the project commands below.

- macOS:
    - Install Homebrew and run:
        - `brew install rustup-init node bun just`
        - `brew install --cask docker`
- Linux:
    - Install Rust with rustup: `curl https://sh.rustup.rs -sSf | sh`
    - Install Node.js from your distro package manager or official binary.
    - Install Bun: `curl -fsSL https://bun.sh/install | bash`
    - Install just: `cargo install just` (or your distro package)
    - Install Docker Engine and Docker Compose plugin from official Docker docs for your distro.
- Windows:
    - Install Rust via rustup installer (MSVC toolchain).
    - Install Node.js LTS from official installer.
    - Install Bun using PowerShell installer from bun.sh.
    - Install just: `winget install --id Casey.Just --exact` (or `scoop install just`).
    - Install Docker Desktop for Windows.

Every project command runs through `just`, which behaves identically on macOS,
Linux, and Windows. After installation, verify tool availability:

```bash
just system-validate
```

### 1. Validate System

```bash
just system-validate
```

### 2. Install Dependencies

```bash
just deps-install
```

This will install:
- Frontend Bun packages

If `sqlx` is not installed yet, install it once with:

```bash
just install-sqlx
```

### 3. Setup Environment

```bash
cp .env.example .env
```

`just` loads `.env` automatically for every recipe, on every platform.

### 4. Start Database

```bash
just db start
```

### 5. Run Migrations

```bash
just db up
```

### 6. Choose Your Development Path

**Option A: Full Stack with Docker**
```bash
just docker up
```

**Option B: Individual Services**
```bash
# Terminal 1: Start API
just api dev

# Terminal 2: Start Frontend
just web dev
```

## 📖 Available Commands

Run `just` to list every recipe, or `just --show db::up` to see the exact
command a recipe runs. The same commands work on macOS, Linux, and Windows.

**`check` recipes never write files. `fmt`, `fix`, and `mig-create` do.**

### System Management
| Command | Description |
|---------|-------------|
| `just` | List all recipes, including module recipes |
| `just system-validate` | Validate required local tooling for repository commands |
| `just deps-install` | Validate tooling, then install webapp dependencies |
| `just install-sqlx` | Install sqlx-cli (one time) |

### Validation
| Command | Description |
|---------|-------------|
| `just check` | Read-only pre-PR gate: backend fmt/lint/test + webapp lint/typecheck |
| `just fix` | Auto-format and auto-fix everything (writes files) |

### Running Services
| Command | Description |
|---------|-------------|
| `just docker up` | Start full stack with Docker Compose |
| `just docker down` | Stop all Docker services |
| `just docker ps` | Show container status |
| `just docker logs [service]` | Show recent container logs |
| `just db start` | Start only the TimescaleDB database |
| `just api dev` | Run API locally and not through docker |
| `just api lint` | Lint the api binary with clippy |
| `just api lint-ci` | Lint the whole workspace including tests |
| `just api fmt` | Format Rust code (writes files) |
| `just api fmt-check` | Verify Rust formatting (read-only) |
| `just api test` | Run the Rust test suite |
| `just api check` | Read-only backend gate: fmt-check, lint-ci, test |
| `just web dev` | Start Next.js development server |
| `just web install` | Install webapp dependencies |
| `just web lint` | Run webapp lint checks (read-only) |
| `just web typecheck` | Run TypeScript typecheck (read-only) |
| `just web fmt` | Format the webapp with prettier (writes files) |
| `just web fix` | Auto-format and auto-fix the webapp (writes files) |
| `just web check` | Read-only webapp gate: lint + typecheck |

### Database Operations
| Command | Description |
|---------|-------------|
| `just db status` | Check current migration status |
| `just db up` | Apply all pending migrations |
| `just db down` | Rollback the last migration |
| `just db reset` | Drop and recreate the database (asks for confirmation) |
| `just db mig-create <name>` | Create a new migration file |
| `just db prepare-offline` | Prepare offline SQL query metadata |

### Development Commands
```bash
# Build everything
cargo build

# Run the full read-only gate before opening a PR
just check

# Frontend development
just web dev           # Development server
just web lint          # Code linting
just web typecheck     # Type checking
```

## 🗂️ Project Structure

```
smarter-home/
├── 📁 src/
│   ├── 📁 services/
│   │   ├── 📁 api/           # Rust backend REST API
│   │   ├── 📁 webapp/        # Next.js web application
│   └── 📁 lib/               # Shared Rust libraries
│       ├── 📁 lib-models/    # Data models
│       ├── 📁 lib-db/        # Database models and functions
│       └── 📁 lib-utils/     # Utility functions
├── 📁 migrations/            # Database migrations
├── 📄 docker-compose.yml     # Container orchestration
├── 📄 Dockerfile.api         # API container config
├── 📄 justfile               # Task runner entrypoint
├── 📁 just/                  # Task runner modules (db, api, web, docker)
├── 📄 Cargo.toml             # Rust workspace config
└── 📄 .env.example           # Environment template
```

## ⚙️ Configuration

### Environment Variables

Copy `.env.example` to `.env` and configure:

```bash
# General settings
LOG_LEVEL=debug

# Backend settings
BACKEND_PORT=3001
APP_ENV=local

# Database settings
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/smarter-home
DB_HOST=database
DB_PORT=5432
DB_NAME=smarter-home
DB_USERNAME=postgres
DB_PASSWORD=postgres
DB_SCHEMA=public

# Frontend settings
FRONTEND_PORT=3000

# Webapp settings
API_URL=http://localhost:3001

# Auth settings
AUTH_SECRET=your-secret-key
AUTH_ACCESS_TTL_SECS=900
AUTH_REFRESH_TTL_SECS=2592000
```

For a full variable-by-variable reference, see `docs/environment-variables.md`.

### Database URL Notes
- **Docker development**: Use `database` as hostname
- **Local development**: Use `localhost` as hostname

## 🐳 Docker Deployment

### Development
```bash
just docker up
```

### Production
```bash
docker compose up --build -d
```

Services will be available at:
- Frontend: http://localhost:3000
- API: http://localhost:3001
- Database: localhost:5432

## 🔧 Local Development (running app services individually)
Always ensure the database is running first:
```bash
just db start
```
Then start services as needed:
- **API**:
```bash
just api dev
```

## 🧪 Testing

### Backend Tests
```bash
just api test
```

### Frontend Validation
```bash
just web check
```

### Integration Tests
```bash
# Start services first
just docker up

# Run your integration tests here
```

## 🔧 Troubleshooting

### Common Issues

**Port Already in Use**
- Next.js will automatically try ports 3001, 3002, etc.
- Check `docker compose ps` for conflicting services

**Database Connection Failed**
- Ensure database is running: `just db start`
- Check DATABASE_URL in `.env` file
- Verify hostname (localhost vs database)

**Frontend Dependencies**
- Use `bun install` to refresh frontend dependencies
- Keep `bun.lock` updated when dependency changes are intentional

**Rust Compilation Issues**
- Ensure you have the latest stable Rust: `rustup update`
- Clear target directory: `cargo clean`

### Build Times
- Initial Rust build: ~2-3 minutes
- Incremental Rust builds: ~10-30 seconds
- Frontend build: ~30-60 seconds
- sqlx-cli installation: ~2-3 minutes (first time)

## 📚 Documentation

- Canonical AI agent instructions: `CLAUDE.md`
- Contribution rules: `CONTRIBUTING.md`
- Community expectations: `CODE_OF_CONDUCT.md`
- Security policy: `SECURITY.md`
- Detailed engineering docs: `docs/README.md`
- Environment variable reference: `docs/environment-variables.md`

## 🤝 Contributing

See `CONTRIBUTING.md` for branch naming, PR policy, and validation checklist.
By participating in this project, you agree to follow `CODE_OF_CONDUCT.md`.

## 📋 Roadmap

- [ ] Real-time energy monitoring dashboard
- [ ] Historical data visualization and analytics
- [ ] Energy usage predictions and recommendations
- [ ] Mobile app development
- [ ] Integration with smart home devices
- [ ] Advanced reporting and export features

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🆘 Support

If you encounter any issues:

1. Check the troubleshooting section above
2. Review the GitHub Issues for known problems
3. Create a new issue with detailed information about your problem
4. Include your OS, Rust version, Node.js version, and Docker version

---

**Happy monitoring! 🏡⚡**
