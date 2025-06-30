# Golf Score Tracker Examples

This directory contains runnable examples demonstrating various Rust concepts and features of the golf score tracker.

## Running Examples

```bash
# Run a specific example
cargo run --example basic_usage
cargo run --example advanced_stats
cargo run --example iterator_patterns

# List all available examples
cargo run --example
```

## Data Storage

Examples create a local `examples_data/` directory to store temporary data. This directory is:

- ✅ Created automatically when examples run
- ✅ Excluded from git (in `.gitignore`)
- ✅ Safe to delete at any time

The main application uses `golf_data/` for persistent storage, which is also git-ignored.

## Available Examples

### `basic_usage.rs`

Demonstrates core functionality:

- Creating players and scorecards
- Recording scores
- Viewing results
- Saving to repository

### `advanced_stats.rs`

Shows advanced features:

- Multiple rounds simulation
- Comprehensive statistics
- Iterator patterns (map, filter, fold)
- Complex data analysis

### `iterator_patterns.rs`

Teaches Rust iterator patterns through golf scenarios:

- `map()` - Transform data
- `filter()` - Conditional selection
- `filter_map()` - Transform + filter
- `fold()` - Accumulation
- Chaining operations

## Modifying Examples

Feel free to modify examples to experiment! Changes to examples are tracked by git, but the data they generate
(`examples_data/`) is not.
