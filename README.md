rucli - Rust CLI Tool

🎯 100 PR Challenge: Building a feature-rich CLI tool in 100 PRs
Progress: 51/100 PRs 🎉

[■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□]
Current Phase: Phase 3 - Advanced Features (46-65)

Implementing pipes, redirection, and scripting support.
Completed Features ✅
Phase 1: Foundation (PR 1-25) ✅

    Basic REPL with help, echo, repeat commands
    Command structure and prompt
    File operations (cat, write, ls)
    Command metadata and auto-generated help
    Module organization
    Comprehensive error handling (RucliError)
    Logging framework with --debug flag
    Complete test suite (unit + integration)

Phase 2: Basic Features (PR 26-45) ✅

    Working directory management (pwd, cd with ~, -, ..)
    Directory operations (mkdir with -p option)
    File operations (rm with -r, -f, -rf options)
    Copy operations (cp with -r for directories)
    Move/rename operations (mv)
    Search operations (find with wildcards, grep with regex)
    Command aliases system
    Version command
    Parser refactoring for maintainability
    Phase 2 integration tests

Phase 3: Advanced Features (PR 46-65) 🚀

    PR #46: Pipeline infrastructure foundation
    PR #47: Basic pipe implementation (|)
    PR #48: Multiple pipe support
    PR #49: Output redirection (>)
    PR #50: Append redirection (>>)
    PR #51: Input redirection (<)

Latest Changes (PR #51)

    Implemented input redirection with < operator
    Commands can read input from files instead of arguments
    Modified execute_redirect to return String for pipeline integration
    Updated handle_cat to support input parameter for stdin
    Fixed execute_command_get_output to properly handle all redirect types
    Full support for combining input redirection with pipelines

Usage

bash

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

# NEW: Input redirection support!
> cat < input.txt
Contents of input.txt displayed here

> grep pattern < data.txt
Lines matching pattern from data.txt

# Combine with pipelines
> grep error < log.txt | grep -v warning
> cat < config.txt | grep setting

Command Summary

File Operations:

    cat - Display file contents
    write - Write content to file
    cp - Copy files (with -r for directories)
    mv - Move/rename files and directories
    rm - Remove files (with -r, -f, -rf options)

Directory Operations:

    ls - List directory contents
    cd - Change directory (supports ~, -)
    pwd - Print working directory
    mkdir - Make directory (with -p for parents)

Search Operations:

    find - Find files by name pattern (wildcards: *, ?)
    grep - Search text in files (regex support)
        File input: outputs with line numbers
        Pipe input: outputs without line numbers (UNIX-compatible)

Utility Commands:

    echo - Display message
    repeat - Repeat message multiple times
    alias - Manage command shortcuts
    version - Show version information
    help - Show available commands
    exit/quit - Exit the program

Pipeline & Redirection Support:

    | - Pipe output of one command to another
    > - Redirect output to file (overwrites existing file)
    >> - Redirect output to file (appends to existing file)
    < - Redirect input from file (reads file as input)
    Combine operations: cmd1 | cmd2 > file, cmd < input.txt | cmd2, etc.

Redirection Examples

bash

# Basic output redirection (overwrite)
> echo "Hello, World!" > greeting.txt
> cat greeting.txt
Hello, World!

# Append redirection
> echo "First entry" >> diary.txt
> echo "Second entry" >> diary.txt
> cat diary.txt
First entry
Second entry

# Input redirection (NEW!)
> cat < file.txt                    # Same as cat file.txt but via stdin
> grep "error" < server.log         # Search in file via stdin
> grep "TODO" < main.rs | grep -v "DONE"  # Combine with pipeline

# Traditional usage vs input redirection
> grep pattern file.txt             # File as argument (shows filename in output)
> grep pattern < file.txt           # File as stdin (no filename in output)

# Complex pipeline with input redirection
> cat < input.txt | grep keyword | sort > output.txt

All Redirection Types

bash

# Output redirection (>) - overwrites file
> echo "new content" > file.txt
> ls > directory_list.txt

# Append redirection (>>) - adds to file
> echo "line 1" >> log.txt
> echo "line 2" >> log.txt

# Input redirection (<) - reads from file
> cat < input.txt
> grep pattern < data.txt

# Combining redirections with pipelines
> cat < input.txt | grep pattern > output.txt
> grep error < log.txt | grep -v debug >> filtered_errors.txt
> find . *.txt < /dev/null | sort > sorted_files.txt

Example Scripts

Check out the examples/ directory for practical usage examples:

    file_organizer.rucli - Organize files by extension
    backup_script.rucli - Backup project files
    log_analyzer.rucli - Analyze and aggregate log files
    data_processor.rucli - Process data with input redirection (NEW!)

Run examples with:

bash

rucli < examples/data_processor.rucli

Debug Mode

bash

# Run with debug logging enabled
$ cargo run -- --debug

# Debug output includes:
# - Command parsing steps
# - Pipeline detection and splitting
# - Redirection parsing (>, >>, <)
# - File read/write operations
# - Command execution flow
# - Alias expansion
# - Execution timing

# Example debug output:
> cat < input.txt
[DEBUG] Parsing input: 'cat < input.txt'
[DEBUG] Redirect detected: < input.txt (input mode)
[DEBUG] Input redirect from file: 'input.txt'
[DEBUG] Executing command: Cat { filename: "" }
[DEBUG] 処理時間: 0.5ms

Dependencies

toml

[dependencies]
env_logger = "0.11"
log = "0.4"
regex = "1.11"
once_cell = "1.19"

[dev-dependencies]
assert_cmd = "2.0"
predicates = "3.0"
tempfile = "3.8"

Project Structure

src/
├── main.rs       # Entry point and REPL loop
├── lib.rs        # Library root (exposes public API)
├── commands.rs   # Command definitions and execution
├── parser.rs     # Command parsing with pipeline & all redirect support
├── handlers.rs   # Command implementation handlers (input-aware)
├── error.rs      # Custom error types
├── alias.rs      # Alias management module
├── pipeline.rs   # Pipeline execution logic (with execute_get_output)
└── redirect.rs   # Redirection handling (>, >>, <)

tests/
├── cli_tests.rs         # Basic integration tests
└── integration_tests.rs # Comprehensive workflow tests

examples/
├── file_organizer.rucli  # File organization example
├── backup_script.rucli   # Backup automation example
├── log_analyzer.rucli    # Log aggregation example
└── data_processor.rucli  # Input redirection example (NEW!)

Code Quality

The codebase follows Rust best practices:

    Named constants instead of magic strings
    Comprehensive error handling with custom types
    Modular architecture with clear separation of concerns
    Extensive logging for debugging
    Thorough test coverage (unit + integration)
    Atomic file operations where possible
    Efficient pattern matching algorithms
    Memory-efficient file processing
    Optimized regex compilation
    Thread-safe global state management
    Well-structured parser with dedicated parsing functions
    Clean separation between data structures and execution logic
    Output-based command handlers for pipeline support
    Input-aware command handlers for stdin support
    UNIX-compatible output formatting
    Proper handling of complex command structures (pipes + all redirects)
    Consistent file operation behavior (>, >>, <)
    String-returning redirect operations for pipeline integration

Error Handling

The project uses a custom RucliError type with complete Result-based error handling:

    Type-safe error propagation throughout the application
    Unified error handling in main loop
    Automatic conversion from io::Error
    Consistent error messages
    All commands return Result<()> or Result<String> for consistency
    InvalidRegex error type for pattern compilation failures
    Pipeline-specific error handling
    File operation error handling for all redirect types
    Graceful handling of non-existent input files
    Proper error propagation in complex pipelines

Testing

bash

# Run all tests
cargo test

# Run unit tests only
cargo test --lib

# Run integration tests only
cargo test --test cli_tests
cargo test --test integration_tests

# Run input redirect tests specifically
cargo test input_redirect

# Run all redirect tests
cargo test redirect

# Run with output
cargo test -- --nocapture

Test Coverage Summary

    Unit tests: 20 tests (parser: 18, commands: 2)
    Basic integration tests: 11 tests
    Workflow integration tests: 16 comprehensive tests
    Pipeline tests: 4 tests
    Redirect tests: 12 tests (all redirect types)
    Total: 63 tests ensuring reliability

Logging

The project uses env_logger for configurable logging:

bash

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
RUST_LOG=rucli::redirect=debug cargo run  # Debug for all redirect operations

Log Categories:

    ERROR: Command execution failures, invalid regex patterns, file read/write errors
    WARN: Invalid operations (e.g., cat on directory)
    INFO: Important operations (file reads, writes, copies, moves, grep matches, alias operations, program start/exit)
    DEBUG: Command parsing, validation, operation details, alias expansion, pipeline detection, redirect execution (all types)
    TRACE: Detailed command lookup and parsing steps

Roadmap 🗺️
Phase 1: Foundation (PR 1-25) ✅ COMPLETED!

Established core infrastructure, error handling, logging, and testing framework.
Phase 2: Basic Features (PR 26-45) ✅ COMPLETED!

Implemented essential file and directory operations, search capabilities, and command aliases.
Phase 3: Advanced Features (PR 46-65) 🚀 IN PROGRESS!

    PR #46: Pipeline infrastructure foundation
    PR #47: Basic pipe implementation
    PR #48: Multiple pipe support
    PR #49: Output redirection (>)
    PR #50: Append redirection (>>)
    PR #51: Input redirection (<)
    PR #52: Background execution (&)
    PR #53: Job management
    PR #54: Environment variables (env)
    PR #55: Export command
    PR #56-57: Refactoring
    PR #58-65: Scripting support

Phase 4: Interactive Features (PR 66-85) - 20 PRs

History, completion, visual enhancements:

    Command history
    Tab completion
    Syntax highlighting
    Multi-line editing
    Custom prompts
    Git integration

Phase 5: Extensions (PR 86-100) - 15 PRs

Plugins, themes, configuration system:

    Plugin architecture
    Theme support
    Configuration files
    Import/export functionality
    Performance optimization
    Final polish

Building

bash

# Development
cargo build
cargo run

# Release
cargo build --release

# Run tests
cargo test

Contributing

This is a learning project following the 100 PR Challenge. Each PR focuses on a single, well-defined improvement.

Next: Implementing background execution (&) in PR #52! 🚀
