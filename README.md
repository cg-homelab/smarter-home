# Smarter Home - Energy Monitoring and Analytics

A comprehensive energy monitoring and analytics system that helps homeowners understand their energy consumption patterns and make smarter decisions about energy usage.

## 🏠 Project Overview

Smarter Home provides:
- **Real-time energy monitoring** with TimescaleDB time-series database
- **Web dashboard** built with Next.js 15 and modern React components
- **Desktop application** using Tauri for native performance
- **RESTful API** powered by Rust and Axum framework
- **Docker containerization** for easy deployment

## 🏗️ Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Web Frontend  │    │ Desktop App     │    │   REST API      │
│   (Next.js)     │◄──►│   (Tauri)       │◄──►│   (Rust/Axum)   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                                                        │
                                               ┌─────────────────┐
                                               │   TimescaleDB   │
                                               │   (PostgreSQL)  │
                                               └─────────────────┘
```

### Tech Stack

- **Backend**: Rust + Axum + SQLx + TimescaleDB
- **Frontend**: Next.js 15 + TypeScript + TailwindCSS + Shadcn UI
- **Desktop**: Tauri v2 + React + Vite
- **Database**: TimescaleDB (PostgreSQL with time-series extensions)
- **Containerization**: Docker + Docker Compose
- **Authentication**: Auth.js with GitHub OAuth

## 🚀 Quick Start

### Prerequisites

Before starting, ensure you have:
- **Rust** (latest stable) - [Install here](https://rustup.rs/)
- **Node.js** v18+ with npm - [Install here](https://nodejs.org/)
- **Docker** with Docker Compose v2 - [Install here](https://docs.docker.com/get-docker/)

### 1. Validate System

```bash
make validate-system
```

### 2. Install Dependencies

```bash
make install-dependencies
```

This will install:
- `sqlx-cli` for database operations
- Frontend npm packages
- Desktop npm packages

### 3. Setup Environment

```bash
cp .env.example .env
```

### 4. Start Database

```bash
make start-database
```

### 5. Run Migrations

```bash
make db-up
```

### 6. Choose Your Development Path

**Option A: Full Stack with Docker**
```bash
make start-docker
```

**Option B: Individual Services**
```bash
# Terminal 1: Start API
cargo run --bin api

# Terminal 2: Start Frontend
cd src/services/frontend && npm run dev

# Terminal 3: Start Desktop (optional)
make start-desktop
```

## 📖 Available Commands

### System Management
| Command | Description |
|---------|-------------|
| `make validate-system` | Check if all prerequisites are installed |
| `make install-dependencies` | Install all project dependencies |

### Running Services
| Command | Description |
|---------|-------------|
| `make start-docker` | Start full stack with Docker Compose |
| `make start-database` | Start only the TimescaleDB database |
| `make start-desktop` | Start desktop app in development mode |
| `make stop-docker` | Stop all Docker services |

### Database Operations
| Command | Description |
|---------|-------------|
| `make db-status` | Check current migration status |
| `make db-up` | Apply all pending migrations |
| `make db-down` | Rollback the last migration |
| `make db-reset` | Rollback all migrations |
| `make db-mig-create` | Create a new migration file |
| `make db-prepare-offline` | Prepare offline SQL query metadata |

### Development Commands
```bash
# Build everything
cargo build

# Run tests
cargo test

# Frontend development
cd src/services/frontend
npm run dev        # Development server
npm run build      # Production build
npm run lint       # Code linting

# Desktop development
cd src/services/desktop
npm run dev        # Development mode
npm run build      # Build desktop app
npm run tauri dev  # Run in Tauri dev mode
```

## 🗂️ Project Structure

```
smarter-home/
├── 📁 src/
│   ├── 📁 services/api/      # All Project Services
│   │   ├── 📁 api/           # Rust backend REST API
│   │   ├── 📁 frontend/      # Next.js web application
│   │   └── 📁 desktop/       # Tauri desktop app
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
# Backend settings
BACKEND_PORT=3001
APP_ENV=local

# Database settings
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/smarter-home
DB_HOST=localhost
DB_PORT=5432
DB_NAME=smarter-home
DB_USERNAME=postgres
DB_PASSWORD=postgres

# Frontend settings
FRONTEND_PORT=3000
NEXTAUTH_URL=http://localhost:3000

# Authentication (get from GitHub OAuth app)
AUTH_SECRET=your-secret-key
AUTH_GITHUB_ID=your-github-client-id
AUTH_GITHUB_SECRET=your-github-client-secret
```

### Database URL Notes
- **Docker development**: Use `database` as hostname
- **Local development**: Use `localhost` as hostname

## 🐳 Docker Deployment

### Development
```bash
make start-docker
```

### Production
```bash
docker compose up --build -d
```

Services will be available at:
- Frontend: http://localhost:3000
- API: http://localhost:3001
- Database: localhost:5432

## 🧪 Testing

### Backend Tests
```bash
cargo test
```

### Frontend Tests
```bash
cd src/services/frontend
npm test
```

### Integration Tests
```bash
# Start services first
make start-docker

# Run your integration tests here
```

## 🔧 Troubleshooting

### Common Issues

**Port Already in Use**
- Next.js will automatically try ports 3001, 3002, etc.
- Check `docker compose ps` for conflicting services

**Database Connection Failed**
- Ensure database is running: `make start-database`
- Check DATABASE_URL in `.env` file
- Verify hostname (localhost vs database)

**Frontend Dependencies**
- npm audit warnings are common and usually safe to ignore
- Run `npm audit fix` if needed

**Rust Compilation Issues**
- Ensure you have the latest stable Rust: `rustup update`
- Clear target directory: `cargo clean`

### Build Times
- Initial Rust build: ~2-3 minutes
- Incremental Rust builds: ~10-30 seconds
- Frontend build: ~30-60 seconds
- sqlx-cli installation: ~2-3 minutes (first time)

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature`
3. Make your changes following the project structure
4. Test your changes: `make validate-system && cargo test`
5. Commit and push your changes
6. Create a Pull Request

### Development Workflow

1. **System validation**: `make validate-system`
2. **Install dependencies**: `make install-dependencies`
3. **Start database**: `make start-database`
4. **Run migrations**: `make db-up`
5. **Make your changes**
6. **Test thoroughly**
7. **Create PR**

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
