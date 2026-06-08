# Smarter Home - Energy Monitoring and Analytics

A comprehensive energy monitoring and analytics system that helps homeowners understand their energy consumption patterns and make smarter decisions about energy usage.

## 🏠 Project Overview

Smarter Home provides:
- **Real-time energy monitoring** with TimescaleDB time-series database
- **Web dashboard** built with Next.js 16 and modern React components
- **RESTful API** powered by Rust and Axum framework
- **JWT + session-cookie auth flow** for secure API access
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
- **Authentication**: Backend JWT + httpOnly session cookie

## 🚀 Quick Start

### Prerequisites

Before starting, ensure you have:
- **Rust** (latest stable) - [Install here](https://rustup.rs/)
- **Node.js** v18+ - [Install here](https://nodejs.org/)
- **Bun** (latest stable) - [Install here](https://bun.sh)
- **Docker** with Docker Compose v2 - [Install here](https://docs.docker.com/get-docker/)

### macOS/Linux/Windows setup notes

Use one of these quick setup paths, then continue with the project commands below.

- macOS:
    - Install Homebrew and run:
        - `brew install rustup-init node bun`
        - `brew install --cask docker`
- Linux:
    - Install Rust with rustup: `curl https://sh.rustup.rs -sSf | sh`
    - Install Node.js from your distro package manager or official binary.
    - Install Bun: `curl -fsSL https://bun.sh/install | bash`
    - Install Docker Engine and Docker Compose plugin from official Docker docs for your distro.
- Windows:
    - Install Rust via rustup installer (MSVC toolchain).
    - Install Node.js LTS from official installer.
    - Install Bun using PowerShell installer from bun.sh.
    - Install Docker Desktop for Windows.

After installation, verify tool availability:

```bash
make system-validate
```

On Windows without `make`, use:

```powershell
cargo --version
bun --version
docker --version
docker compose version
```

### 1. Validate System

```bash
make system-validate
```

### 2. Install Dependencies

```bash
make deps-install
```

This will install:
- Frontend Bun packages

If `sqlx` is not installed yet, install it once with:

```bash
cargo install sqlx-cli --no-default-features --features rustls,postgres
```

### 3. Setup Environment

```bash
cp .env.example .env
```

**Windows - manually load environtment variables before running app !!!*
```powershell
Get-Content .env | Where-Object { $_ -match '=' -and $_ -notmatch '^#' } | ForEach-Object { $name, $value = $_ -split '=', 2; Set-Item "env:$($name.Trim())" $value.Trim() }
```
### 4. Start Database

```bash
make db-start
```

### 5. Run Migrations

```bash
make db-up
```

### 6. Choose Your Development Path

**Option A: Full Stack with Docker**
```bash
make docker-start
```

**Option B: Individual Services**
```bash
# Terminal 1: Start API
cargo run --bin api

# Terminal 2: Start Frontend
cd src/services/webapp && bun run dev
```

## 📖 Available Commands

### System Management
| Command | Description |
|---------|-------------|
| `make system-validate` | Validate required local tooling for repository commands |
| `make deps-install` | Install webapp dependencies |

### Running Services
| Command | Description |
|---------|-------------|
| `make docker-start` | Start full stack with Docker Compose |
| `make docker-stop` | Stop all Docker services |
| `make db-start` | Start only the TimescaleDB database |
| `make api-dev` | Run API locally and not through docker |
| `make api-lint` | Run API lint checks with clippy |
| `make api-format` | Format Rust code |
| `make api-check` | Run API lint and format checks |
| `make web-dev` | Start Next.js development server |
| `make web-lint` | Run webapp lint checks |
| `make web-format` | Run webapp formatting |
| `make web-check` | Run webapp checks and TypeScript typecheck |

### Database Operations
| Command | Description |
|---------|-------------|
| `make db-status` | Check current migration status |
| `make db-up` | Apply all pending migrations |
| `make db-down` | Rollback the last migration |
| `make db-reset` | Rollback all migrations |
| `make db-mig-create` | Create a new migration file |
| `make db-prepare-offline` | Prepare offline SQL query metadata |

### Windows (PowerShell) Equivalents (No make)

Use these commands directly when `make` is not available:

| Make Target | PowerShell Command |
|---------|-------------|
| `make system-validate` | `cargo --version; bun --version` |
| `make deps-install` | `Push-Location src/services/webapp; bun install; Pop-Location` |
| `make docker-start` | `docker compose up --build` |
| `make docker-stop` | `docker compose down` |
| `make db-start` | `docker compose up -d database` |
| `make api-dev` | `cargo run --bin api` |
| `make api-lint` | `cargo clippy --bin api` |
| `make api-format` | `cargo fmt` |
| `make api-check` | `cargo clippy --bin api; cargo fmt` |
| `make web-dev` | `Push-Location src/services/webapp; bun run dev; Pop-Location` |
| `make web-lint` | `Push-Location src/services/webapp; bun run lint; Pop-Location` |
| `make web-format` | `Push-Location src/services/webapp; bun run format; Pop-Location` |
| `make web-check` | `Push-Location src/services/webapp; bun run check; bunx tsc --noEmit; Pop-Location` |
| `make db-status` | `sqlx migrate info` |
| `make db-up` | `sqlx migrate run` |
| `make db-down` | `sqlx migrate revert` |
| `make db-reset` | `sqlx database reset` |
| `make db-mig-create` | `sqlx migrate add <migration_name>` |
| `make db-prepare-offline` | `cargo sqlx prepare --workspace` |

### Development Commands
```bash
# Build everything
cargo build

# Run tests
cargo test

# Frontend development
cd src/services/webapp
bun run dev            # Development server
bun run build          # Production build
bun run lint           # Code linting
bunx tsc --noEmit      # Type checking
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
├── 📄 Makefile               # Build automation
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
```

For a full variable-by-variable reference, see `docs/environment-variables.md`.

### Database URL Notes
- **Docker development**: Use `database` as hostname
- **Local development**: Use `localhost` as hostname

## 🐳 Docker Deployment

### Development
```bash
make docker-start
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
make db-start
```
Then start services as needed:
- **API**:
```bash
make api-dev
```

## 🧪 Testing

### Backend Tests
```bash
cargo test
```

### Frontend Validation
```bash
cd src/services/webapp
bun run lint
bunx tsc --noEmit
```

### Integration Tests
```bash
# Start services first
make docker-start

# Run your integration tests here
```

## 🔧 Troubleshooting

### Common Issues

**Port Already in Use**
- Next.js will automatically try ports 3001, 3002, etc.
- Check `docker compose ps` for conflicting services

**Database Connection Failed**
- Ensure database is running: `make db-start`
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

- Canonical AI agent instructions: `AGENTS.md`
- Contribution rules: `CONTRIBUTING.md`
- Detailed engineering docs: `docs/README.md`
- Environment variable reference: `docs/environment-variables.md`

## 🤝 Contributing

See `CONTRIBUTING.md` for branch naming, PR policy, and validation checklist.

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
