# rucli - Rust CLI Tool

100 Commit Challenge: Building a feature-rich CLI tool in 100 commits

Progress: 62/100 Commits

## Latest Changes (Commit #62)

- Added **shell functions** with definition and call support
- Function syntax: `function name() { command; }`
- Positional parameters: `$1`, `$2`, `$3`...
- Functions can be redefined (overwritten)
- Functions persist within session

## Usage

### Functions (NEW!)

Define and call custom functions:

```bash
# Basic function definition
> function greet() { echo Hello, World!; }
> greet
Hello, World!

# Function with arguments
> function welcome() { echo Welcome, $1 to $2!; }
> welcome Alice "Rust CLI"
Welcome, Alice to Rust CLI!

# Practical examples
> function backup() { cp $1 $1.bak; }
> backup important.txt

> function show_file() { cat $1; }
> show_file config.txt
```

**Current limitations:**
- Function definition and calling
- Positional parameters ($1, $2, ...)
- Function overwriting
- Multiple commands per function - not yet supported
- Local variables - not yet supported
- Return values - not yet supported
- Special variables ($#, $@) - not yet supported

### Control Flow Features

**Conditionals:**
```bash
if condition; then action; else alternative; fi
```

**While loops:**
```bash
while condition; do action; done
```

**For loops:**
```bash
for var in list; do action; done
```

### Complete Feature Set

**Control Flow:**
- If-then-else conditionals
- While loops
- For loops
- Functions (NEW!)
- Background execution with `&`
- Pipeline chaining with `|`

**File Operations:** `cat`, `write`, `cp`, `mv`, `rm`

**Directory Operations:** `ls`, `cd`, `pwd`, `mkdir`

**Search Operations:** `find`, `grep`

**Environment:** `env` - manage environment variables

**Job Control:** `jobs`, `fg` - background job management

**Utilities:** `echo`, `repeat`, `sleep`, `alias`, `version`, `help`, `exit`

### Operators

- `|` - Pipe commands together
- `>` - Redirect output to file
- `>>` - Append output to file
- `<` - Input from file
- `&` - Background execution
- `<<` - Here document
- `<<-` - Here document with tab stripping
- `;` - Command separator
- `if-then-fi` - Conditional execution
- `while-do-done` - Loop execution
- `for-in-do-done` - List iteration
- `function name() { }` - Function definition

## Examples

### Function Scripts

**backup_utils.rsh:**
```bash
#!/usr/bin/env rucli
# Backup utility functions

function backup() { 
    cp $1 $1.bak
}

function restore() {
    cp $1.bak $1
}

# Usage
write important.txt "Critical data"
backup important.txt
echo Backup created
```

**file_utils.rsh:**
```bash
#!/usr/bin/env rucli
# File utility functions

function show() { cat $1; }
function count() { cat $1 | wc -l; }
function find_error() { grep ERROR $1; }

# Process log file
find_error system.log
```

**project_setup.rsh:**
```bash
#!/usr/bin/env rucli
# Project setup functions

function create_dir() { mkdir -p $1; }
function create_file() { write $1 "// TODO"; }

# Setup project
create_dir src
create_file src/main.rs
echo Project created
```

## Project Structure

```
rucli/
├── src/
│   ├── main.rs         # Entry point
│   ├── commands.rs     # Command definitions
│   ├── parser.rs       # Input parsing
│   ├── handlers.rs     # Command implementations
│   ├── functions.rs    # Function storage (NEW!)
│   ├── environment.rs  # Variables & expansions
│   ├── pipeline.rs     # Pipeline execution
│   ├── redirect.rs     # I/O redirection
│   ├── job.rs          # Background jobs
│   ├── alias.rs        # Command aliases
│   └── error.rs        # Error handling
├── tests/
│   └── integration_tests.rs
└── examples/
    ├── functions.rsh   # Function examples (NEW!)
    ├── loops.rsh       # For/while loops
    └── scripts.rsh     # General scripts
```

## Testing

```bash
cargo test              # Run all tests
cargo test function     # Test functions
cargo run -- test.rsh   # Run a script
```

## Roadmap

### Phase 3: Advanced Features (46-65)
- Pipelines (46-48)
- Redirections (49-51)
- Background execution (52)
- Job management (53)
- Environment variables (54)
- Variable expansion (55)
- Command substitution (56)
- Here documents (57)
- Script file execution (58)
- If conditions (59)
- While loops (60)
- For loops (61)
- Functions (62)
- Error handling in scripts (63)
- Script debugging (64)
- Phase 3 integration tests (65)

### Phase 4: Interactive Features (66-85)
- Command history & navigation
- Tab completion
- Syntax highlighting

### Phase 5: Extensions (86-100)
- Plugin system
- Configuration files
- Performance optimization

## Next: Error Handling (Commit #63)

Improve error handling in scripts:
- `set -e` equivalent (exit on error)
- Error propagation
- Exit codes
- Error messages improvement