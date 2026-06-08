# Load environment variables from .env file
ifneq (,$(wildcard ./.env))
    include .env
    export
endif

# Validate system by checking if go, npm, docker and docker compose are installed
system-validate:
	@echo "Validating system..."
	@if command -v cargo > /dev/null; then \
					echo "Cargo is installed..."; \
			else \
					echo "Cargo is not installed..."; \
					exit 1; \
			fi
	@if command -v bun > /dev/null; then \
					echo "Bun is installed..."; \
			else \
					echo "Bun is not installed..."; \
					exit 1; \
			fi \


deps-install: system-validate
	@echo "Installing dependencies..."
	# TODO: Have been some problems related to this.
	# @echo "Installing database dependencies..." && \
	# cargo install sqlx-cli --no-default-features --features rustls,postgres
	@cd src/services/webapp && \
	echo "Installing webapp dependencies..." && \
	bun install
# 	@echo "Installing github dependencies..." && \
# 	npm i -D @actions/github-script@github:actions/github-script

## Running project
# Start with docker
docker-start:
	@docker compose up --build

# Stop with docker
docker-stop:
	@docker compose down


# API
api-dev:
	@cargo run --bin api
api-lint:
	@cargo clippy --bin api
api-format:
	@cargo fmt
api-check: api-lint api-format

# Webapp
web-dev:
	@cd src/services/webapp && bun run dev
web-lint:
	@cd src/services/webapp && bun run lint
web-format:
	@cd src/services/webapp && bun run format
web-check:
	@cd src/services/webapp && bun run check && bunx tsc --noEmit

## Database
# Start just database
db-start:
	@docker compose up -d database
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
