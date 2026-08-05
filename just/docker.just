# Docker Compose recipes for the full stack.

set working-directory := '..'
set windows-shell := ['powershell.exe', '-NoLogo', '-Command']

# Show docker recipes
[group('docker')]
help:
    @just --list docker

# Build and start the full stack in the foreground
[group('docker')]
up:
    docker compose up --build

# Stop and remove all containers
[group('docker')]
down:
    docker compose down

# Show container status
[group('docker')]
ps:
    docker compose ps

# Show recent logs, optionally for one service: just docker logs api
[group('docker')]
logs service='':
    docker compose logs --tail 200 {{ service }}
