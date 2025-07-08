# rucli - Rust CLI Tool

🎯 **100 PR Challenge**: Building a feature-rich CLI tool in 100 PRs

## Progress: 41/100 PRs

[■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□]

## Current Phase: Basic Features (26-45)

Implementing file operations and search capabilities.

## Completed Features ✅

- [x] PR #1-2: Basic REPL with help, echo, repeat commands
- [x] PR #3-4: Command structure and prompt
- [x] PR #5-7: File operations (cat, write, ls)
- [x] PR #8: Command metadata and auto-generated help
- [x] PR #9-10: Module organization
- [x] PR #11: Argument validation automation
- [x] PR #12: Unit tests for parser module
- [x] PR #13: Unit tests for commands module
- [x] PR #14: Integration test framework
- [x] PR #15: Complete integration test suite
- [x] PR #16: Custom error type (RucliError)
- [x] PR #17: Apply custom error type throughout
- [x] PR #18: Result type in handlers (Part 1)
- [x] PR #19: Complete Result type implementation
- [x] PR #20: Logging framework introduction
- [x] PR #21: Implement comprehensive logging
- [x] PR #22: Add --debug flag
- [x] PR #23: Debug information features
- [x] PR #24: First refactoring (Part 1)
- [x] PR #25: First refactoring (Part 2) - Documentation
- [x] PR #26: Working directory management foundation
- [x] PR #27: pwd command implementation
- [x] PR #28: cd command basic implementation
- [x] PR #29: cd advanced features (-, ~, ..)
- [x] PR #30: mkdir command basic implementation
- [x] PR #31: mkdir -p option (recursive)
- [x] PR #32: rm command basic implementation (files only)
- [x] PR #33: rm extended options (-r, -f, -rf)
- [x] PR #34: cp command basic implementation
- [x] PR #35: cp command with directory support
- [x] PR #36: mv command implementation
- [x] PR #37: PR numbering adjustment
- [x] PR #38: find command basic implementation
- [x] PR #39: find command with wildcard support
- [x] PR #40: grep command basic implementation
- [x] PR #41: grep with regex support

## Latest Changes (PR #41)

- Added regular expression support to grep command
- Integrated regex crate for pattern matching
- Support for common regex patterns: ^, $, ., *, +, [], etc.
- Efficient regex compilation (once per file)
- Proper error handling for invalid regex patterns

## Usage

```bash
$ cargo run
Hello, rucli!
> help
Available commands:
  help                          - Show this help message
  echo <message...>             - Display message
  cat <filename>                - Display file contents
  write <filename> <content...> - Write content to file
  ls                            - List directory contents
  pwd                           - Print working directory
  cd <directory>                - Change directory
  cd                            - Change to home directory
  cd -                          - Change to previous directory
  cd ~                          - Change to home directory
  mkdir <directory>             - Make directory
  mkdir -p <directory>          - Make directory (create parents)
  rm <file>                     - Remove file
  rm -r <directory>             - Remove directory recursively
  rm -f <file>                  - Force remove (ignore errors)
  rm -rf <path>                 - Force recursive removal
  cp <source> <destination>     - Copy files
  cp -r <source> <destination>  - Copy directories recursively
  mv <source> <destination>     - Move/rename files or directories
  find [directory] <pattern>    - Find files by name (wildcards: *, ?)
  grep <pattern> <file...>      - Search for pattern in files (regex)
  repeat <count> <message...>   - Repeat message count times
  exit                          - Exit the program
  quit                          - Exit the program

Options:
  --debug                       - Enable debug mode with detailed logging

# Regex grep examples
> write code.rs fn main() {\n    println!(\"Hello\");\n}\nfn test() {}
File written successfully: code.rs

# Basic regex patterns
> grep ^fn code.rs
1: fn main() {
4: fn test() {}

> grep main.* code.rs
1: fn main() {

> grep [0-9]+ numbers.txt
3: Version 1.2.3
5: Count: 42

# Character classes
> grep [a-z]+ file.txt
# Matches lowercase words

# Anchors
> grep ^# README.md
# Matches lines starting with #

# Repetition
> grep lo+ test.txt
# Matches "lo", "loo", "looo", etc.

# Note: Currently quotes must be omitted in patterns
# Use: grep ^test file.txt
# Not: grep "^test" file.txt
```

### Regular Expression Support

Common regex patterns:
- `.` - Any character
- `*` - Zero or more of preceding
- `+` - One or more of preceding
- `?` - Zero or one of preceding
- `^` - Start of line
- `$` - End of line
- `[abc]` - Character class
- `[a-z]` - Character range
- `\d` - Digit
- `\w` - Word character
- `|` - Alternation (OR)

### Debug Mode

```bash
# Run with debug logging enabled
$ cargo run -- --debug

# Debug output for grep includes:
# - Regex compilation status
# - Pattern being searched
# - Match results per line

# Example debug output:
> grep [0-9]+ test.txt
[DEBUG] Regex compiled: '[0-9]+'
[DEBUG] Searching in file: test.txt
[DEBUG] Line 5 matches pattern
5: Count: 123
[DEBUG] 処理時間: 2.1ms
```

## Known Limitations

- Quote handling in the parser needs improvement
- Currently, regex patterns should be entered without quotes
- Complex patterns with spaces require parser updates (future PR)

## Dependencies

```toml
[dependencies]
env_logger = "0.11"
log = "0.4"
regex = "1.11"
```

## Project Structure

```
src/
├── main.rs       # Entry point and REPL loop
├── lib.rs        # Library root (exposes public API)
├── commands.rs   # Command definitions and execution with Result types
├── parser.rs     # Command parsing with RucliError (with tests)
├── handlers.rs   # Command implementation handlers
└── error.rs      # Custom error types

tests/
└── cli_tests.rs  # Integration tests (11 tests)
```

## Code Quality

The codebase follows Rust best practices:
- Named constants instead of magic strings
- Comprehensive error handling with custom types
- Modular architecture with clear separation of concerns
- Extensive logging for debugging
- Thorough test coverage
- Atomic file operations where possible
- Efficient pattern matching algorithms
- Memory-efficient file processing
- Optimized regex compilation

## Error Handling

The project now uses a custom `RucliError` type with complete Result-based error handling:
- Type-safe error propagation throughout the application
- Unified error handling in main loop
- Automatic conversion from `io::Error`
- Consistent error messages
- All commands return Result<()> for consistency
- InvalidRegex error type for pattern compilation failures

## Logging

The project uses `env_logger` for configurable logging:

```bash
# Log levels
RUST_LOG=error cargo run    # Only errors
RUST_LOG=warn cargo run     # Warnings and errors
RUST_LOG=info cargo run     # Info, warnings, and errors
RUST_LOG=debug cargo run    # Debug info and above
RUST_LOG=trace cargo run    # Everything (very verbose)

# Module-specific logging
RUST_LOG=rucli::parser=trace cargo run    # Trace for parser only
RUST_LOG=rucli::handlers=debug cargo run  # Debug for handlers only
```

### Log Categories:
- **ERROR**: Command execution failures, invalid regex patterns
- **WARN**: Invalid operations (e.g., cat on directory)
- **INFO**: Important operations (file writes, reads, copies, moves, grep matches, program start/exit)
- **DEBUG**: Command parsing, validation, operation details, regex compilation
- **TRACE**: Detailed command lookup and parsing steps

## Testing

```bash
# Run all tests
cargo test

# Run unit tests only
cargo test --lib

# Run integration tests only
cargo test --test cli_tests

# Run with output
cargo test -- --nocapture
```

## Test Coverage Summary

- **Unit tests**: 10 tests (parser: 8, commands: 2)
- **Integration tests**: 11 tests (all commands + edge cases)
- **Total**: 21 tests ensuring reliability

## Roadmap 🗺️

### Phase 1: Foundation (PR 11-25) - 15 PRs ✅ COMPLETED!

- [x] PR #11: Argument validation
- [x] PR #12: Unit tests for parser
- [x] PR #13: Unit tests for commands
- [x] PR #14: Integration test framework
- [x] PR #15: Complete integration tests
- [x] PR #16: Custom error type introduction
- [x] PR #17: Apply custom error type
- [x] PR #18: Result type in handlers (Part 1)
- [x] PR #19: Complete Result type implementation
- [x] PR #20: Logging framework introduction
- [x] PR #21: Implement comprehensive logging
- [x] PR #22: Add --debug flag
- [x] PR #23: Debug information features
- [x] PR #24: First refactoring (Part 1)
- [x] PR #25: First refactoring (Part 2) - Documentation

### Phase 2: Basic Features (PR 26-45) - 20 PRs

- [x] PR #26: Working directory management foundation
- [x] PR #27: pwd command implementation
- [x] PR #28: cd command basic implementation
- [x] PR #29: cd advanced features (-, ~, ..)
- [x] PR #30: mkdir command basic implementation
- [x] PR #31: mkdir -p option (recursive)
- [x] PR #32: rm command basic implementation (files only)
- [x] PR #33: rm extended options (-r, -f, -rf)
- [x] PR #34: cp command basic implementation
- [x] PR #35: cp command with directory support
- [x] PR #36: mv command implementation
- [x] PR #37: PR numbering adjustment
- [x] PR #38: find command basic implementation
- [x] PR #39: find command with wildcard support
- [x] PR #40: grep command basic implementation
- [x] PR #41: grep with regex support
- [ ] PR #42: Command aliases
- [ ] PR #43-44: Third refactoring
- [ ] PR #45: Phase 2 completion

### Phase 3: Advanced Features (PR 46-65) - 20 PRs

Pipes, redirection, scripting support

### Phase 4: Interactive Features (PR 66-85) - 20 PRs

History, completion, visual enhancements

### Phase 5: Extensions (PR 86-100) - 15 PRs

Plugins, themes, configuration system

## Building

```bash
# Development
cargo build
cargo run

# Release
cargo build --release

# Run tests
cargo test
```

## Contributing

This is a learning project following the 100 PR Challenge. Each PR focuses on a single, well-defined improvement.