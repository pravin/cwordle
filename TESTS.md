# Testing Documentation for cwordle

This document describes the testing strategy, conventions, and how to run tests for the cwordle project.

## Table of Contents

- [Overview](#overview)
- [Running Tests](#running-tests)
- [Test Structure](#test-structure)
- [Test Coverage](#test-coverage)
- [Writing Tests](#writing-tests)
- [Testing Conventions](#testing-conventions)
- [Continuous Integration](#continuous-integration)

## Overview

The cwordle project has comprehensive test coverage including:
- **Unit Tests**: Test individual modules and functions in isolation
- **Integration Tests**: Test interaction between different modules
- **Edge Case Tests**: Test boundary conditions and unusual scenarios

### Test Statistics

```
Total Tests: 56
- Unit Tests: 42
  - Config Module: 17 tests
  - GameState Module: 12 tests
  - Stats Module: 11 tests
  - Words Module: 2 tests
- Integration Tests: 14 tests
```

## Running Tests

### Run All Tests

```bash
cargo test
```

### Run Unit Tests Only

```bash
cargo test --lib
```

### Run Integration Tests Only

```bash
cargo test --test integration_test
```

### Run Tests for a Specific Module

```bash
# Test only gamestate module
cargo test gamestate

# Test only stats module
cargo test stats

# Test only config module
cargo test config

# Test only words module
cargo test words
```

### Run a Specific Test

```bash
cargo test test_name
```

### Run Tests with Output

```bash
# Show println! output even for passing tests
cargo test -- --nocapture

# Show test execution details
cargo test -- --test-threads=1 --nocapture
```

### Generate Test Coverage Report

```bash
# Install tarpaulin (if not already installed)
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html
```

## Test Structure

### Unit Tests

Unit tests are located in the same file as the code they test, within a `#[cfg(test)]` module:

```
src/
├── config.rs          # Contains config unit tests
├── gamestate.rs       # Contains gamestate unit tests
├── stats.rs           # Contains stats unit tests
└── words.rs           # Contains words unit tests
```

### Integration Tests

Integration tests are in the `tests/` directory and test interactions between modules:

```
tests/
└── integration_test.rs    # All integration tests
```

## Test Coverage

### Config Module Tests (`src/config.rs`)

Tests configuration validation and command-line argument parsing:

- ✅ Default configuration validation
- ✅ Max guesses validation (0, >10, 1-10 range)
- ✅ Custom word validation (length, case, special characters)
- ✅ Flag validation (hard mode, stats, reset stats, no color)
- ✅ Combined options validation
- ✅ Constants verification

### GameState Module Tests (`src/gamestate.rs`)

Tests core game logic and state management:

- ✅ New game initialization
- ✅ Winning guess handling
- ✅ Losing game scenario
- ✅ Keyboard helper state (correct, wrong position, incorrect)
- ✅ Hard mode revealed letters tracking
- ✅ Hard mode constraint enforcement
- ✅ Multiple guess handling
- ✅ Custom max guesses
- ✅ Keyboard state priority (GREEN > AMBER > GRAY)

### Stats Module Tests (`src/stats.rs`)

Tests statistics tracking and persistence:

- ✅ Default stats initialization
- ✅ Recording wins/losses
- ✅ Win streak tracking
- ✅ Streak breaking on loss
- ✅ Max streak preservation
- ✅ Guess distribution tracking
- ✅ Win percentage calculation
- ✅ Average guesses calculation
- ✅ Stats reset functionality
- ✅ Complex game scenario simulation

### Words Module Tests (`src/words.rs`)

Tests word selection and validation:

- ✅ Word validation (valid/invalid words)
- ✅ Random word selection
- ✅ Word list integrity

### Integration Tests (`tests/integration_test.rs`)

Tests module interactions and complete game flows:

**Game Flow Tests:**
- ✅ Complete winning game
- ✅ Complete losing game
- ✅ Hard mode full game with constraints

**Word Validation Tests:**
- ✅ Word list integrity
- ✅ Random word selection variety
- ✅ Word validation consistency

**Stats Integration Tests:**
- ✅ Stats with game results
- ✅ Guess distribution accuracy

**Config Integration Tests:**
- ✅ Config with game initialization
- ✅ Invalid config error handling

**Keyboard Helper Tests:**
- ✅ Keyboard state progression

**Edge Case Tests:**
- ✅ Win on last guess
- ✅ Words with repeated letters
- ✅ Immediate win (first guess)

## Writing Tests

### Test Naming Convention

```rust
#[test]
fn test_{feature_being_tested}() {
    // Test implementation
}
```

### Test Structure

Follow the Arrange-Act-Assert pattern:

```rust
#[test]
fn test_example() {
    // Arrange: Set up test data
    let config = create_test_config();

    // Act: Execute the function being tested
    let result = config.validate();

    // Assert: Verify the results
    assert!(result.is_ok());
}
```

### Helper Functions

Create helper functions to reduce test boilerplate:

```rust
fn create_test_config() -> Config {
    Config {
        hard_mode: false,
        max_guesses: 6,
        stats: false,
        reset_stats: false,
        word: None,
        no_color: false,
    }
}
```

### Testing Error Cases

Use `assert!(result.is_err())` and check error messages:

```rust
#[test]
fn test_invalid_input() {
    let result = validate_word("TEST"); // uppercase
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("lowercase"));
}
```

## Testing Conventions

### 1. Test Independence

Each test should be independent and not rely on the state from other tests:

```rust
// Good: Each test creates its own state
#[test]
fn test_win() {
    let mut stats = GameStats::default();
    stats.record_game(true, Some(3));
    assert_eq!(stats.games_won, 1);
}

#[test]
fn test_loss() {
    let mut stats = GameStats::default(); // Fresh state
    stats.record_game(false, None);
    assert_eq!(stats.games_won, 0);
}
```

### 2. Descriptive Test Names

Test names should clearly describe what is being tested:

```rust
// Good
#[test]
fn test_word_validation_uppercase()

// Bad
#[test]
fn test1()
```

### 3. One Assertion Focus

Each test should focus on one specific behavior (though multiple assertions are OK if testing the same feature):

```rust
// Good: Testing one behavior with related assertions
#[test]
fn test_win_updates_stats() {
    let mut stats = GameStats::default();
    stats.record_game(true, Some(3));

    assert_eq!(stats.games_played, 1);
    assert_eq!(stats.games_won, 1);
    assert_eq!(stats.current_streak, 1);
}
```

### 4. Test Both Success and Failure

Always test both valid and invalid cases:

```rust
#[test]
fn test_valid_word() {
    assert!(word_isvalid("apple"));
}

#[test]
fn test_invalid_word_too_short() {
    assert!(!word_isvalid("app"));
}
```

### 5. Use Meaningful Test Data

Use realistic data that clearly demonstrates the test case:

```rust
// Good: Clear what's being tested
let word = "APPLE"; // Uppercase should fail

// Bad: Unclear
let word = "test";
```

## Continuous Integration

### Pre-commit Checklist

Before committing code, run:

```bash
# Run all tests
cargo test

# Check for warnings
cargo clippy

# Format code
cargo fmt

# Build in release mode
cargo build --release
```

### GitHub Actions (Future)

Recommended CI workflow:

```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Run tests
        run: cargo test --verbose
      - name: Run clippy
        run: cargo clippy -- -D warnings
      - name: Check formatting
        run: cargo fmt -- --check
```

## Test Development Workflow

### Adding New Features

1. Write tests first (TDD approach):
   ```bash
   # Write failing test
   cargo test new_feature_test
   ```

2. Implement the feature

3. Run tests to verify:
   ```bash
   cargo test new_feature_test
   ```

4. Run all tests to ensure no regressions:
   ```bash
   cargo test
   ```

### Debugging Failed Tests

```bash
# Run specific failing test with output
cargo test failing_test_name -- --nocapture

# Run with backtrace for panics
RUST_BACKTRACE=1 cargo test failing_test_name
```

### Test Performance

```bash
# Time test execution
cargo test -- --test-threads=1

# Run tests in parallel (default)
cargo test
```

## Common Testing Patterns

### Testing with Custom Configuration

```rust
fn create_config_with(hard_mode: bool, max_guesses: usize) -> Config {
    Config {
        hard_mode,
        max_guesses,
        stats: false,
        reset_stats: false,
        word: Some("tests".to_string()),
        no_color: false,
    }
}

#[test]
fn test_hard_mode() {
    let config = create_config_with(true, 6);
    let game = GameData::new(&config);
    assert!(game.is_hard_mode());
}
```

### Testing State Transitions

```rust
#[test]
fn test_game_state_transition() {
    let config = create_test_config();
    let mut game = GameData::new(&config);

    // Initial state
    assert_eq!(game.status(), GameStatus::PLAYING);

    // After winning guess
    game.new_guess(&"apple".to_string()).unwrap();
    assert_eq!(game.status(), GameStatus::WON);
}
```

### Testing Collections

```rust
#[test]
fn test_guess_distribution() {
    let mut stats = GameStats::default();

    stats.record_game(true, Some(3));
    stats.record_game(true, Some(3));

    assert_eq!(stats.guess_distribution.get(&3), Some(&2));
}
```

## Troubleshooting

### Common Issues

**Issue: Tests pass individually but fail together**
- Cause: Tests are sharing state
- Solution: Ensure each test creates its own fresh state

**Issue: Test fails on CI but passes locally**
- Cause: Environment differences
- Solution: Check file paths, test with `--release` flag

**Issue: Slow test execution**
- Cause: Too many integration tests or slow operations
- Solution: Mock external dependencies, use unit tests where possible

## Best Practices

1. **Keep tests fast**: Unit tests should run in milliseconds
2. **Test edge cases**: Test boundary conditions (0, max, empty)
3. **Use descriptive assertions**: Include helpful failure messages
4. **Maintain test code quality**: Tests are code too - keep them clean
5. **Update tests with code changes**: Keep tests in sync with implementation
6. **Document complex test scenarios**: Add comments for non-obvious tests
7. **Group related tests**: Use nested modules to organize tests

## Resources

- [Rust Book - Testing Chapter](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Rust API Guidelines - Testing](https://rust-lang.github.io/api-guidelines/necessities.html#c-test)
- [Cargo Book - Tests](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#tests)

## Contributing

When adding new features:
1. Write tests that cover the new functionality
2. Ensure all existing tests still pass
3. Add integration tests if the feature involves multiple modules
4. Update this documentation if new testing patterns are introduced

---

**Last Updated**: 2025-09-30
**Test Count**: 56 tests (42 unit + 14 integration)
**Coverage**: Core functionality, edge cases, and module interactions
