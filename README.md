# rucli - Rust CLI Tool

🎯 **100 PR Challenge**: Building a feature-rich CLI tool in 100 PRs

## Progress: 50/100 PRs 🎉

[■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□]

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
- [x] PR #47: Basic pipe implementation (|)
- [x] PR #48: Multiple pipe support
- [x] PR #49: Output redirection (>)
- [x] PR #50: Append redirection (>>)

## Latest Changes (PR #50)

- Implemented append redirection with `>>` operator
- Files can be appended to instead of overwritten
- Creates new file if it doesn't exist
- Support for combining pipes and append redirection
- Added `execute_get_output` method for pipeline string output
- Fixed command execution to handle all command types

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

# NEW: Append redirection support!
> echo "First line" > log.txt
> echo "Second line" >> log.txt
> echo "Third line" >> log.txt
> cat log.txt
First line
Second line
Third line

# Create new file with append
> echo "New file content" >> new.txt
> cat new.txt
New file content

# Combine pipes and append redirection
> cat data.txt | grep ERROR >> errors.log
> find . *.log | grep 2024 >> logs_2024.txt
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
  - File input: outputs with line numbers
  - Pipe input: outputs without line numbers (UNIX-compatible)

**Utility Commands:**
- `echo` - Display message
- `repeat` - Repeat message multiple times
- `alias` - Manage command shortcuts
- `version` - Show version information
- `help` - Show available commands
- `exit`/`quit` - Exit the program

**Pipeline & Redirection Support:**
- `|` - Pipe output of one command to another
- `>` - Redirect output to file (overwrites existing file)
- `>>` - Redirect output to file (appends to existing file)
- Combine pipes and redirection: `cmd1 | cmd2 > file` or `cmd1 | cmd2 >> file`

## Redirection Examples

```bash
# Basic output redirection (overwrite)
> echo "Hello, World!" > greeting.txt
> cat greeting.txt
Hello, World!

# Append redirection (NEW!)
> echo "First entry" >> diary.txt
> echo "Second entry" >> diary.txt
> cat diary.txt
First entry
Second entry

# Create new file with append
> echo "Log started" >> new_log.txt
> ls
new_log.txt

# Accumulate search results
> grep ERROR app.log >> all_errors.txt
> grep ERROR system.log >> all_errors.txt
> grep ERROR database.log >> all_errors.txt

# Pipeline with append redirection
> cat server.log | grep "404" >> not_found_errors.txt
> find . *.txt | grep readme >> readme_files.txt

# Build a file incrementally
> echo "# Daily Report" > report.txt
> echo "" >> report.txt
> echo "## Morning Tasks" >> report.txt
> ls | grep -v "test" >> report.txt
> echo "" >> report.txt
> echo "## Afternoon Tasks" >> report.txt
> find . *.rs | grep main >> report.txt
```

## Append vs Overwrite

```bash
# Overwrite (>) - replaces entire file
> echo "Line 1" > file.txt
> echo "Line 2" > file.txt
> cat file.txt
Line 2

# Append (>>) - adds to end of file
> echo "Line 1" > file.txt
> echo "Line 2" >> file.txt
> cat file.txt
Line 1
Line 2
```

## Example Scripts

Check out the `examples/` directory for practical usage examples:
- `file_organizer.rucli` - Organize files by extension
- `backup_script.rucli` - Backup project files
- `log_analyzer.rucli` - Analyze and aggregate log files (NEW!)

Run examples with:
```bash
rucli < examples/log_analyzer.rucli
```

## Debug Mode

```bash
# Run with debug logging enabled
$ cargo run -- --debug

# Debug output includes:
# - Command parsing steps
# - Pipeline detection and splitting
# - Redirection parsing (> vs >>)
# - File append operations
# - Command execution flow
# - Alias expansion
# - Execution timing

# Example debug output:
> echo "test" >> output.txt
[DEBUG] Parsing input: 'echo "test" >> output.txt'
[DEBUG] Redirect detected: >> output.txt (append mode)
[DEBUG] Executing command: Echo { message: "test" }
[DEBUG] Appending output to file: output.txt
[DEBUG] 処理時間: 0.8ms
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
├── parser.rs     # Command parsing with pipeline & redirect support
├── handlers.rs   # Command implementation handlers (output-based)
├── error.rs      # Custom error types
├── alias.rs      # Alias management module
├── pipeline.rs   # Pipeline execution logic (with execute_get_output)
└── redirect.rs   # Redirection handling (> and >>)

tests/
├── cli_tests.rs         # Basic integration tests
└── integration_tests.rs # Comprehensive workflow tests

examples/
├── file_organizer.rucli # File organization example
├── backup_script.rucli  # Backup automation example
└── log_analyzer.rucli   # Log aggregation example (NEW!)
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
- Output-based command handlers for pipeline support
- UNIX-compatible output formatting
- Proper handling of complex command structures (pipes + redirects)
- Consistent file operation behavior (> vs >>)

## Error Handling

The project uses a custom `RucliError` type with complete Result-based error handling:
- Type-safe error propagation throughout the application
- Unified error handling in main loop
- Automatic conversion from `io::Error`
- Consistent error messages
- All commands return Result<()> or Result<String> for consistency
- InvalidRegex error type for pattern compilation failures
- Pipeline-specific error handling
- File operation error handling for redirects
- Graceful handling of append to non-existent files

## Testing

```bash
# Run all tests
cargo test

# Run unit tests only
cargo test --lib

# Run integration tests only
cargo test --test cli_tests
cargo test --test integration_tests

# Run append redirect tests specifically
cargo test append_redirect

# Run with output
cargo test -- --nocapture
```

## Test Coverage Summary

- **Unit tests**: 17 tests (parser: 15, commands: 2)
- **Basic integration tests**: 11 tests
- **Workflow integration tests**: 12 comprehensive tests
- **Pipeline tests**: 4 tests
- **Redirect tests**: 8 tests (including append tests)
- **Total**: 52 tests ensuring reliability

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
RUST_LOG=rucli::redirect=debug cargo run  # Debug for redirect module
```

### Log Categories:
- **ERROR**: Command execution failures, invalid regex patterns, file write errors
- **WARN**: Invalid operations (e.g., cat on directory)
- **INFO**: Important operations (file writes, reads, copies, moves, grep matches, alias operations, program start/exit)
- **DEBUG**: Command parsing, validation, operation details, alias expansion, pipeline detection, redirect execution, append operations
- **TRACE**: Detailed command lookup and parsing steps

## Roadmap 🗺️

### Phase 1: Foundation (PR 1-25) ✅ COMPLETED!

Established core infrastructure, error handling, logging, and testing framework.

### Phase 2: Basic Features (PR 26-45) ✅ COMPLETED!

Implemented essential file and directory operations, search capabilities, and command aliases.

### Phase 3: Advanced Features (PR 46-65) 🚀 IN PROGRESS!

- [x] PR #46: Pipeline infrastructure foundation
- [x] PR #47: Basic pipe implementation
- [x] PR #48: Multiple pipe support
- [x] PR #49: Output redirection (>)
- [x] PR #50: Append redirection (>>)
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

**Next**: Implementing input redirection (<) in PR #51! 🚀