# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-08-20

### Added
- Initial release of `fred-client-rs`
- Async client for FRED API with `reqwest`
- Support for all FRED API v1 endpoints:
  - Categories, Releases, Series, Sources, Tags
- Support for FRED API v2 bulk endpoint
- Type-safe query parameter builders
- Comprehensive error handling with `thiserror`
- Integration tests with `wiremock`
- Examples for common use cases
- `cargo-deny` security and license auditing

[Unreleased]: https://github.com/johnathonfox/fred-client-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/johnathonfox/fred-client-rs/releases/tag/v0.1.0
