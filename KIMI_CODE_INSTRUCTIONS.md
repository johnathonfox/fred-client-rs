# Kimi Code Build Instructions

## Overview

This artifact contains a complete, production-ready Rust client for the FRED API.
Crate name: `fred-client-rs` ( `fred-rs` was already taken on crates.io )

## Quick Start for Kimi Code

### Step 1: Initialize Repository
```bash
cd fred-client-rs
git init
git add .
git commit -m "Initial commit: fred-client-rs v0.1.0"
```

### Step 2: Verify Build
```bash
cargo check --all-features
cargo test --all-features
```

### Step 3: Fix Any Issues
If compilation fails:
1. Check `Cargo.toml` dependencies are compatible
2. Run `cargo update` to resolve versions
3. Fix any type mismatches in endpoint implementations

### Step 4: Before Publishing
1. Replace `{org}` and `{Author}` placeholders in:
   - `Cargo.toml`
   - `README.md`
   - `CHANGELOG.md`
   - `LICENSE-*`
   - `.github/CODEOWNERS`
   - All `docs/*.md` files
2. Set up repository secrets:
   - `FRED_API_KEY` — for integration tests
   - `CARGO_REGISTRY_TOKEN` — for crates.io publishing
3. Get a FRED API key: https://fred.stlouisfed.org/docs/api/api_key.html
4. Get a crates.io token: https://crates.io/settings/tokens

### Step 5: Publish
```bash
git tag v0.1.0
git push origin v0.1.0
```
CI will automatically publish to crates.io.

## File Manifest

```
fred-client-rs/
├── Cargo.toml                    # Package manifest
├── deny.toml                     # cargo-deny policy
├── rustfmt.toml                  # Formatting config
├── clippy.toml                   # Clippy config
├── LICENSE-MIT                   # MIT license
├── LICENSE-APACHE                # Apache 2.0 license
├── CHANGELOG.md                  # Version history
├── README.md                     # Project readme
├── .github/
│   ├── CODEOWNERS                # Code ownership
│   ├── FUNDING.yml               # Sponsorship config
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.md
│   │   └── feature_request.md
│   ├── PULL_REQUEST_TEMPLATE.md
│   └── workflows/
│       ├── ci.yml                # PR checks (fmt, clippy, test)
│       ├── test.yml              # Full test matrix (daily)
│       ├── audit.yml             # Security audit (daily)
│       ├── deps.yml              # Dependency updates (weekly)
│       ├── docs.yml              # Docs publishing
│       └── release.yml           # crates.io publish on tag
├── docs/
│   ├── ARCHITECTURE.md           # Design docs
│   ├── CONTRIBUTING.md           # Contribution guide
│   └── EXAMPLES.md               # Usage examples
├── scripts/
│   ├── bootstrap.sh              # Dev environment setup
│   └── test.sh                   # Local test runner
├── src/
│   ├── lib.rs                    # Library entry point
│   ├── client.rs                 # FredClient + builder
│   ├── error.rs                  # Error types
│   ├── types/
│   │   ├── mod.rs
│   │   ├── common.rs             # Shared types
│   │   ├── category.rs
│   │   ├── release.rs
│   │   ├── series.rs
│   │   ├── observation.rs
│   │   ├── source.rs
│   │   └── tag.rs
│   ├── endpoints/
│   │   ├── mod.rs
│   │   ├── categories.rs         # /category/* endpoints
│   │   ├── releases.rs           # /release/* endpoints
│   │   ├── series.rs             # /series/* endpoints
│   │   ├── sources.rs            # /source/* endpoints
│   │   ├── tags.rs               # /tag/* endpoints
│   │   └── v2.rs                 # /v2/* endpoints
│   └── params/
│       ├── mod.rs
│       └── query.rs              # Parameter builders
├── tests/
│   ├── integration_tests.rs      # Wiremock integration tests
│   └── fixtures/
│       ├── category.json
│       ├── series.json
│       └── observations.json
├── benches/
│   └── client_benchmark.rs       # Criterion benchmarks
└── examples/
    ├── basic_usage.rs            # Basic client usage
    ├── search_series.rs          # Series search example
    └── observations.rs           # Observations example
```

## API Coverage

### Categories
- `category(id)` → `/category`
- `category_children(id, params)` → `/category/children`
- `category_related(id, params)` → `/category/related`
- `category_series(id, params)` → `/category/series`
- `category_tags(id, params)` → `/category/tags`
- `category_related_tags(id, tag_names, params)` → `/category/related_tags`

### Releases
- `releases(params)` → `/releases`
- `releases_dates(params)` → `/releases/dates`
- `release(id)` → `/release`
- `release_dates(id, params)` → `/release/dates`
- `release_series(id, params)` → `/release/series`
- `release_sources(id, params)` → `/release/sources`
- `release_tags(id, params)` → `/release/tags`
- `release_related_tags(id, tag_names, params)` → `/release/related_tags`
- `release_tables(id, params)` → `/release/tables`

### Series
- `series(id)` → `/series`
- `series_categories(id, params)` → `/series/categories`
- `series_observations(id, params)` → `/series/observations`
- `series_release(id, params)` → `/series/release`
- `series_search(text, params)` → `/series/search`
- `series_search_tags(text, params)` → `/series/search/tags`
- `series_search_related_tags(text, tag_names, params)` → `/series/search/related_tags`
- `series_tags(id, params)` → `/series/tags`
- `series_updates(params)` → `/series/updates`
- `series_vintage_dates(id, params)` → `/series/vintagedates`

### Sources
- `sources(params)` → `/sources`
- `source(id)` → `/source`
- `source_releases(id, params)` → `/source/releases`

### Tags
- `tags(params)` → `/tags`
- `related_tags(tag_names, params)` → `/related_tags`
- `tags_series(tag_names, params)` → `/tags/series`

### v2 Bulk
- `release_observations_v2(id, params)` → `/v2/release/observations`

## Automation Summary

| Task | Trigger | Action |
|------|---------|--------|
| CI | PR / push to main | fmt, clippy, test, deny |
| Full Test | Daily 2 AM UTC | Test matrix: stable/beta/nightly × ubuntu/macos/windows |
| Security Audit | Daily midnight UTC | `cargo audit` |
| Dependency Check | Weekly Monday 6 AM UTC | `cargo outdated`, create issue if stale |
| Docs | Push to main / tag | Publish to GitHub Pages |
| Release | Push tag `v*` | Verify version, test, publish to crates.io, GitHub release |

## Known Issues / TODO

1. Some endpoints return `serde_json::Value` where types are complex/variable:
   - `release_tables()`
   - `series_vintage_dates()`
   - `release_observations_v2()`
   Consider adding proper typed structs for these.

2. The `blocking` feature is declared but not fully implemented.
   Add a `blocking::FredClient` if needed.

3. Rate limiting: FRED API has rate limits.
   Consider adding retry logic with `reqwest-retry`.

4. Caching: Consider adding optional response caching.

5. Pagination helper: Add an async stream wrapper for paginated endpoints.
