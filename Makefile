# Load environment variables from .env file
ifneq (,$(wildcard ./.env))
    include .env
    export
endif

# Validate system by checking if go, npm, docker and docker compose are installed
validate-system:
	@echo "Validating system..."
	@if command -v cargo > /dev/null; then \
					echo "Cargo is installed..."; \
			else \
					echo "Cargo is not installed..."; \
					exit 1; \
			fi
	@if command -v npm > /dev/null; then \
					echo "Npm is installed..."; \
			else \
					echo "Npm is not installed..."; \
					exit 1; \
			fi \


install-dependencies: validate-system
	@echo "Installing dependencies..."
	@echo "Installing database dependencies..." && \
	cargo install sqlx-cli --no-default-features --features rustls,postgres
	@cd src/frontend && \
	echo "Installing frontend dependencies..." && \
	npm install
	@cd src/desktop && \
	echo "Installing desktop dependencies..." && \
	npm install

## Running project
# Start with docker
start-docker:
	@docker compose up --build

# Start just database
start-database:
	@docker compose up -d database

# Start desktop app
start-desktop:
	@cd src/desktop && npm run dev

# Stop with docker
stop-docker:
	@docker compose down

## Database
# Check status of database
db-status:
	@sqlx migrate info

# Reset database ( rollback all migrations )
db-reset:
	@sqlx database reset

# Rollback last migration
db-down:
	@sqlx migrate revert

# Run all migrations
db-up:
	@sqlx migrate run

# Create new migration
db-mig-create:
	@sqlx migrate add

# Prepare offline database info
db-prepare-offline:
	@cargo sqlx prepare --workspace
