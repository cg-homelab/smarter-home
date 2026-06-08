# Rust Best Practices

Scope: src/services/api and src/lib

## Documentation

- Add and maintain Rust documentation comments for public APIs.
- Use module-level docs for important domain modules.
- Update docs comments whenever behavior changes.

## Testing

- Add focused unit tests for deterministic utility/domain behavior.
- Prioritize tests in src/lib/lib-utils for shared helpers.
- Add integration-oriented tests for critical query or route behavior when practical.

## API correctness

- Keep Utoipa annotations in sync with handler behavior.
- Register new endpoints in OpenAPI aggregation module.
- Keep request/response models aligned with lib-models definitions.

## Code quality

Required:

- cargo fmt
- cargo clippy --bin api

Recommended:

- cargo test

## Structure

- Keep modules small and domain-focused.
- Keep HTTP orchestration in API routes and heavy data logic in lib-db.
- Reuse shared models/errors in lib-models and helpers in lib-utils.
