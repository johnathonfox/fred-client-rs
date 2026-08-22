# Contributing

Thank you for your interest in contributing to `fred-client-rs`!

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/{your-username}/fred-client-rs.git`
3. Run the bootstrap script: `./scripts/bootstrap.sh`
4. Create a branch: `git checkout -b feature/my-feature`

## Development Workflow

1. Make your changes
2. Run tests: `./scripts/test.sh`
3. Ensure checks pass: `cargo fmt --check`, `cargo clippy`, `cargo test`
4. Update `CHANGELOG.md` if applicable
5. Submit a pull request

## Code Style

- Format with `rustfmt`
- Follow `clippy` lints
- Write doc comments for all public APIs
- Add tests for new functionality

## Testing

- Unit tests: `cargo test`
- Integration tests: `cargo test --test integration_tests`
- Integration tests use `wiremock` and do not require an API key
- Examples and live API calls require the `FRED_API_KEY` environment variable

## Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Tag: `git tag vX.Y.Z`
4. Push: `git push origin vX.Y.Z`
5. Publish: `cargo publish`
