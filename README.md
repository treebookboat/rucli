# rucli - Rust CLI Tool

🎯 **100 PR Challenge**: Building a feature-rich CLI tool in 100 PRs

## Progress: 16/100 PRs
[■■■■■■■■■■■■■■■■□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□]

## Current Phase: Foundation (11-25)
Strengthening core infrastructure and command system.

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

## Latest Changes (PR #16)
- Created custom `RucliError` enum for type-safe error handling
- Implemented Display and Error traits
- Added automatic conversion from io::Error
- Prepared foundation for better error handling

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
  repeat <count> <message...>   - Repeat message count times
  exit                          - Exit the program
  quit                          - Exit the program

> echo Hello World
Hello World

> repeat 3 Hi!
Hi!
Hi!
Hi!

> cat README.md
[File contents displayed]

> write test.txt Some content here
File written successfully: test.txt
```

## Project Structure
```
src/
├── main.rs       # Entry point and REPL loop
├── lib.rs        # Library root (exposes public API)
├── commands.rs   # Command definitions and execution (with tests)
├── parser.rs     # Command parsing and validation (with tests)
├── handlers.rs   # Command implementation handlers
└── error.rs      # Custom error types (new!)

tests/
└── cli_tests.rs  # Integration tests (11 tests)
```

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

### Phase 1: Foundation (PR 11-25) - 15 PRs
- [x] PR #11: Argument validation
- [x] PR #12: Unit tests for parser
- [x] PR #13: Unit tests for commands
- [x] PR #14: Integration test framework
- [x] PR #15: Complete integration tests
- [x] PR #16: Custom error type introduction
- [ ] PR #17: Apply custom error type
- [ ] PR #18-19: Result type everywhere
- [ ] PR #20-21: Logging framework
- [ ] PR #22-23: Debug mode
- [ ] PR #24-25: First refactoring

### Phase 2: Basic Features (PR 26-45) - 20 PRs
Basic file operations, process management, search capabilities

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