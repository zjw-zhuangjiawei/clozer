# Common Operations

**Summary**: Build, run, and development commands for the Clozer project.

**Why**: Quick reference for common development tasks.

---

## Build Commands

```bash
# Build debug
cargo build

# Run main application
cargo run --bin clozer

# Release build
cargo build --release

# Check for errors
cargo check

# Run clippy
cargo clippy

# Run tests
cargo test
```

---

## Binary Commands

This project contains 3 binaries:

| Binary | Purpose |
|--------|---------|
| `clozer` | Main desktop application |
| `inspect-db` | Inspect database contents |
| `create-sample-db` | Create database from JSON |

### Main Application

```bash
cargo run --bin clozer
```

### Inspect Database

```bash
# Inspect all tables
cargo run --bin inspect-db -- <DB_PATH>

# Inspect specific table
cargo run --bin inspect-db -- -t <TABLE> <DB_PATH>

# Available tables: words, meanings, clozes, tags
```

### Create Sample Database

```bash
cargo run --bin create-sample-db -- <JSON_FILE> <DB_PATH>
```

---

## Formatting

```bash
# Format all code
cargo fmt --all

# Check formatting without making changes
cargo fmt --all --check
```

---

## Running with Custom Data Directory

```bash
# Use custom data directory
cargo run --bin clozer -- --data-dir /path/to/data

# Use custom config file
cargo run --bin clozer -- --config-file /path/to/config.toml

# Set log level
cargo run --bin clozer -- --log-level debug
```

---

## Environment Variables

The application supports these environment variables:

| Variable | Description |
|----------|-------------|
| `CLOZER_DATA_DIR` | Override data directory |
| `CLOZER_CONFIG_FILE` | Override config file path |
| `CLOZER_LOG_LEVEL` | Set log level (trace, debug, info, warn, error) |

---

## Related Rules

- [Architecture](./1-architecture.md) - Project structure
- [Logging](./7-logging.md) - Log configuration
