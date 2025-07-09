# rucli - Rust CLI Tool

🎯 **100 PR Challenge**: Building a feature-rich CLI tool in 100 PRs

## Progress: 46/100 PRs

[■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□]

## Current Phase: Phase 3 - Advanced Features (46-65)

Implementing pipes, redirection, and scripting support.

## Completed Features ✅

### Phase 1: Foundation (PR 1-25) ✅
- [x] Basic REPL with help, echo, repeat commands
- [x] Command structure and prompt
- [x] File operations (cat, write, ls)
- [x] Command metadata and auto-generated help
- [x] Module organization
- [x] Comprehensive error handling (RucliError)
- [x] Logging framework with --debug flag
- [x] Complete test suite (unit + integration)

### Phase 2: Basic Features (PR 26-45) ✅
- [x] Working directory management (pwd, cd with ~, -, ..)
- [x] Directory operations (mkdir with -p option)
- [x] File operations (rm with -r, -f, -rf options)
- [x] Copy operations (cp with -r for directories)
- [x] Move/rename operations (mv)
- [x] Search operations (find with wildcards, grep with regex)
- [x] Command aliases system
- [x] Version command
- [x] Parser refactoring for maintainability
- [x] Phase 2 integration tests

### Phase 3: Advanced Features (PR 46-65) 🚀
- [x] PR #46: Pipeline infrastructure foundation

## Latest Changes (PR #46)

- Added pipeline module with `PipelineCommand` and `PipelineExecutor` structures
- Implemented pipeline detection and command splitting logic
- Parser now recognizes pipe symbols and prepares for pipeline execution
- Added comprehensive tests for pipeline parsing

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
  alias [name=command]          - Set or show command aliases
  version                       - Show version information
  repeat <count> <message...>   - Repeat message count times
  exit                          - Exit the program
  quit                          - Exit the program

Options:
  --debug                       - Enable debug mode with detailed logging

# Pipeline support (coming soon!)
> echo hello | grep h
Parse error: Pipeline commands not yet implemented

# Example workflow
> mkdir -p project/src
> cd project/src
> write main.rs fn main() { println!("Hello, rucli!"); }
> cd ..
> find *.rs
./src/main.rs
> grep println src/main.rs
1: fn main() { println!("Hello, rucli!"); }
```

## Command Summary

**File Operations:**
- `cat` - Display file contents
- `write` - Write content to file
- `cp` - Copy files (with `-r` for directories)
- `mv` - Move/rename files and directories
- `rm` - Remove files (with `-r`, `-f`, `-rf` options)

**Directory Operations:**
- `ls` - List directory contents
- `cd` - Change directory (supports `~`, `-`)
- `pwd` - Print working directory
- `mkdir` - Make directory (with `-p` for parents)

**Search Operations:**
- `find` - Find files by name pattern (wildcards: *, ?)
- `grep` - Search text in files (regex support)

**Utility Commands:**
- `echo` - Display message
- `repeat` - Repeat message multiple times
- `alias` - Manage command shortcuts
- `version` - Show version information
- `help` - Show available commands
- `exit`/`quit` - Exit the program

**Pipeline Support (In Progress):**
- Pipe symbol (`|`) detection implemented
- Command splitting logic ready
- Execution coming in next PR

## Example Scripts

Check out the `examples/` directory for practical usage examples:
- `file_organizer.rucli` - Organize files by extension
- `backup_script.rucli` - Backup project files

Run examples with:
```bash
rucli < examples/file_organizer.rucli
```

## Debug Mode

```bash
# Run with debug logging enabled
$ cargo run -- --debug

# Debug output includes:
# - Command parsing steps
# - Pipeline detection
# - Alias expansion
# - File operations details
# - Search pattern matching
# - Execution timing

# Example debug output:
> echo hello | grep h
[DEBUG] Parsing input: 'echo hello | grep h'
[DEBUG] Pipeline detected in input
[ERROR] Parse error: Pipeline commands not yet implemented
```

## Dependencies

```toml
[dependencies]
env_logger = "0.11"
log = "0.4"
regex = "1.11"
once_cell = "1.19"

[dev-dependencies]
assert_cmd = "2.0"
predicates = "3.0"
tempfile = "3.8"
```

## Project Structure

```
src/
├── main.rs       # Entry point and REPL loop
├── lib.rs        # Library root (exposes public API)
├── commands.rs   # Command definitions and execution
├── parser.rs     # Command parsing with pipeline support
├── handlers.rs   # Command implementation handlers
├── error.rs      # Custom error types
├── alias.rs      # Alias management module
└── pipeline.rs   # Pipeline infrastructure (NEW)

tests/
├── cli_tests.rs         # Basic integration tests
└── integration_tests.rs # Comprehensive workflow tests

examples/
├── file_organizer.rucli # File organization example
└── backup_script.rucli  # Backup automation example
```

## Code Quality

The codebase follows Rust best practices:
- Named constants instead of magic strings
- Comprehensive error handling with custom types
- Modular architecture with clear separation of concerns
- Extensive logging for debugging
- Thorough test coverage (unit + integration)
- Atomic file operations where possible
- Efficient pattern matching algorithms
- Memory-efficient file processing
- Optimized regex compilation
- Thread-safe global state management
- Well-structured parser with dedicated parsing functions
- Clean separation between data structures and execution logic

## Error Handling

The project uses a custom `RucliError` type with complete Result-based error handling:
- Type-safe error propagation throughout the application
- Unified error handling in main loop
- Automatic conversion from `io::Error`
- Consistent error messages
- All commands return Result<()> for consistency
- InvalidRegex error type for pattern compilation failures

## Testing

```bash
# Run all tests
cargo test

# Run unit tests only
cargo test --lib

# Run integration tests only
cargo test --test cli_tests
cargo test --test integration_tests

# Run pipeline tests specifically
cargo test pipeline

# Run with output
cargo test -- --nocapture
```

## Test Coverage Summary

- **Unit tests**: 13 tests (parser: 11, commands: 2)
- **Basic integration tests**: 11 tests
- **Workflow integration tests**: 7 comprehensive tests
- **Pipeline tests**: 4 tests (NEW)
- **Total**: 35 tests ensuring reliability

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
RUST_LOG=rucli::pipeline=debug cargo run  # Debug for pipeline module
```

### Log Categories:
- **ERROR**: Command execution failures, invalid regex patterns
- **WARN**: Invalid operations (e.g., cat on directory)
- **INFO**: Important operations (file writes, reads, copies, moves, grep matches, alias operations, program start/exit)
- **DEBUG**: Command parsing, validation, operation details, alias expansion, pipeline detection
- **TRACE**: Detailed command lookup and parsing steps

## Roadmap 🗺️

### Phase 1: Foundation (PR 1-25) ✅ COMPLETED!

Established core infrastructure, error handling, logging, and testing framework.

### Phase 2: Basic Features (PR 26-45) ✅ COMPLETED!

Implemented essential file and directory operations, search capabilities, and command aliases.

### Phase 3: Advanced Features (PR 46-65) 🚀 IN PROGRESS!

- [x] PR #46: Pipeline infrastructure foundation
- [ ] PR #47: Basic pipe implementation
- [ ] PR #48: Multiple pipe support
- [ ] PR #49: Output redirection (>)
- [ ] PR #50: Append redirection (>>)
- [ ] PR #51: Input redirection (<)
- [ ] PR #52: Background execution (&)
- [ ] PR #53: Job management
- [ ] PR #54: Environment variables (env)
- [ ] PR #55: Export command
- [ ] PR #56-57: Refactoring
- [ ] PR #58-65: Scripting support

### Phase 4: Interactive Features (PR 66-85) - 20 PRs

History, completion, visual enhancements:
- Command history
- Tab completion
- Syntax highlighting
- Multi-line editing
- Custom prompts
- Git integration

### Phase 5: Extensions (PR 86-100) - 15 PRs

Plugins, themes, configuration system:
- Plugin architecture
- Theme support
- Configuration files
- Import/export functionality
- Performance optimization
- Final polish

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

---

**Next**: Implementing actual pipe execution in PR #47! 🚀