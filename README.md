# rucli - Rust CLI Tool

🎯 **100 PR Challenge**: Building a feature-rich CLI tool in 100 PRs

## Progress: 13/100 PRs
[■■■■■■■■■■■■■□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□]

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

## Latest Changes (PR #13)
- Added unit tests for CommandInfo consistency
- Test for duplicate command names
- Test for valid argument constraints
- Focused on metadata validation rather than I/O testing

## Usage

```bash
$ cargo run
Hello, rucli!
> help
Available commands:
  help                          - Show this help message
  echo              - Display message
  cat                 - Display file contents
  write   - Write content to file
  ls                            - List directory contents
  repeat     - Repeat message count times
  exit                          - Exit the program
  quit                          - Exit the program

> echo Hello World
Hello World

> cat README.md
[File contents displayed]

> write test.txt Some content here
File written successfully: test.txt
```

## Project Structure
```
src/
├── main.rs       # Entry point and REPL loop
├── commands.rs   # Command definitions and execution (with tests)
├── parser.rs     # Command parsing and validation (with tests)
└── handlers.rs   # Command implementation handlers
```

## Testing

```bash
# Run all tests
cargo test

# Run specific module tests
cargo test parser
cargo test commands

# Run with output
cargo test -- --nocapture
```

## Roadmap 🗺️

### Phase 1: Foundation (PR 11-25) - 15 PRs
- [x] PR #11: Argument validation
- [x] PR #12: Unit tests for parser
- [x] PR #13: Unit tests for commands
- [ ] PR #14-15: Integration test framework
- [ ] PR #16-17: Custom error types
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