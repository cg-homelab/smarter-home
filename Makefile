# Load environment variables from .env file
ifneq (,$(wildcard ./.env))
    include .env
    export
endif

# Validate system by checking if go, npm, docker and docker compose are installed
validate-system:
	@echo "Validating system..."
	@if command -v go > /dev/null; then \
					echo "Go is installed..."; \
			else \
					echo "Go is not installed..."; \
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
	@cd src/go-backend && \
	echo "Installing backend dependencies..." && \
	pwd && \
	go mod tidy
	@if command -v air > /dev/null; then \
            echo "Air is installed, skipping...";\
        else \
            go install github.com/air-verse/air@latest; \
        fi
	@if command -v goose > /dev/null; then \
            echo "Goose is installed, skipping...";\
        else \
            go install github.com/pressly/goose/v3/cmd/goose@latest; \
        fi
	@cd src/frontend && \
	echo "Installing frontend dependencies..." && \
	npm install

# Start with docker
docker-up:
	@docker compose up --build

# Stop with docker
docker-down:
	@docker compose down

# Start just database
docker-start-db:
	@docker compose up -d database

# Check status of database
db-status:
	@GOOSE_DRIVER=$(DB_DRIVER) GOOSE_DBSTRING=$(DATABASE_URL) \
		go run github.com/pressly/goose/v3/cmd/goose@latest -dir=$(MIGRATION_DIR) status

# Reset database ( rollback all migrations )
db-reset:
	@GOOSE_DRIVER=$(DB_DRIVER) GOOSE_DBSTRING=$(DATABASE_URL) \
		go run github.com/pressly/goose/v3/cmd/goose@latest -dir=$(MIGRATION_DIR) reset

# Rollback last migration
db-down:
	@GOOSE_DRIVER=$(DB_DRIVER) GOOSE_DBSTRING=$(DATABASE_URL) \
		go run github.com/pressly/goose/v3/cmd/goose@latest -dir=$(MIGRATION_DIR) down

# Run all migrations
db-up:
	@GOOSE_DRIVER=$(DB_DRIVER) GOOSE_DBSTRING=$(DATABASE_URL) \
		go run github.com/pressly/goose/v3/cmd/goose@latest -dir=$(MIGRATION_DIR) up

# Create new migration
db-mig-create:
	@GOOSE_DRIVER=$(DB_DRIVER) GOOSE_DBSTRING=$(DATABASE_URL) \
		go run github.com/pressly/goose/v3/cmd/goose@latest -dir=$(MIGRATION_DIR) create $(filter-out $@,$(MAKECMDGOALS)) sql
