# Architecture

## Overview

`fred-client-rs` is an async Rust client for the Federal Reserve Economic Data (FRED) API. It is built on top of `reqwest` and `tokio`.

## Design Principles

1. **Type Safety**: All API parameters and responses are strongly typed.
2. **Ergonomics**: Builder patterns for complex query parameters.
3. **Performance**: Zero-copy deserialization where possible, connection pooling via `reqwest`.
4. **Testability**: Wiremock-based integration tests, no live API required for unit tests.

## Module Structure

```
src/
├── lib.rs          # Public API exports
├── client.rs       # FredClient and builder
├── error.rs        # Error types
├── types/          # Data models
│   ├── common.rs   # Shared types (SortOrder, Frequency, etc.)
│   ├── category.rs
│   ├── release.rs
│   ├── series.rs
│   ├── observation.rs
│   ├── source.rs
│   └── tag.rs
├── endpoints/      # API endpoint implementations
│   ├── categories.rs
│   ├── releases.rs
│   ├── series.rs
│   ├── sources.rs
│   ├── tags.rs
│   └── v2.rs
└── params/         # Query parameter builders
    └── query.rs
```

## Client Lifecycle

1. **Build**: `FredClient::builder()` → configure API key, base URL, timeout → `build()`
2. **Request**: Method on `FredClient` constructs `RequestBuilder` with path + query params
3. **Execute**: `client.execute<T>()` sends request, checks status, deserializes JSON
4. **Response**: Typed `T` returned or `FredError` on failure

## Error Handling

- `FredError::Request` — HTTP layer errors (timeout, DNS, etc.)
- `FredError::Api` — FRED API returned non-2xx status
- `FredError::Validation` — Invalid parameters before sending
- `FredError::MissingApiKey` — Builder called without API key
- `FredError::Parse` — JSON deserialization failure
