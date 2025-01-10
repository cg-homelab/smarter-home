install-deps:
	@go mod tidy
	@if command -v air > /dev/null; then \
            echo "Air is installed...";\
        else \
            go install github.com/air-verse/air@latest; \
        fi

# Test the application
test:
	@echo "Testing..."
	@go test ./... -v

# Integrations Tests for the application
itest:
	@echo "Running integration tests..."
	@go test ./internal/database -v

# Clean the binary
clean:
	@echo "Cleaning..."
	@rm -f main

build:
	@echo "Building go project..."
	@go build -o main cmd/api/main.go

# Run with hot reload
watch:
	@echo "Watching..."
	@air
