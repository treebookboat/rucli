# rucli - Rust CLI Tool

100 Commit Challenge: Building a feature-rich CLI tool in 100 commits

Progress: 65/100 Commits

## Latest Changes (Commit #65)

- **Parser refactoring**: Split 800+ line parser.rs into modular structure
- Organized into logical modules:
  - `basic.rs`: Basic commands (echo, cat, write)
  - `file_ops.rs`: File operations (cp, mv, rm, mkdir)
  - `control.rs`: Control structures (if, while, for, function)
  - `operators.rs`: Operators (pipe, redirect, background)
  - `utils.rs`: Shared utilities and constants
- Each module contains its own tests for better maintainability
- Improved code organization without functional changes

## Usage

### Interactive Multi-line Input

Execute complex commands with natural multi-line syntax:

```bash
# For loops
> for i in 1 2 3
>> do
>>   echo Number: $i
>> done
Number: 1
Number: 2
Number: 3

# While loops
> while test -f flag
>> do
>>   cat flag
>>   rm flag
>> done

# If statements
> if pwd
>> then
>>   echo "Directory exists"
>> else
>>   echo "Error"
>> fi

# Functions
> function greet()
>> {
>>   echo Hello
>>   echo World
>> }
```

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
- Functions
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
│   ├── parser/         # Modular parser (NEW!)
│   │   ├── mod.rs      # Public interface
│   │   ├── basic.rs    # Basic commands
│   │   ├── file_ops.rs # File operations
│   │   ├── control.rs  # Control structures
│   │   ├── operators.rs# Operators
│   │   └── utils.rs    # Utilities
│   ├── handlers.rs     # Command implementations
│   ├── functions.rs    # Function storage
│   ├── environment.rs  # Variables & expansions
│   ├── pipeline.rs     # Pipeline execution
│   ├── redirect.rs     # I/O redirection
│   ├── job.rs          # Background jobs
│   ├── alias.rs        # Command aliases
│   └── error.rs        # Error handling
├── tests/
│   └── integration_tests.rs
└── examples/
    ├── functions.rsh   # Function examples
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

### Phase 3: Advanced Features & Refactoring (46-70)
- ✅ Command pipelines basic (46)
- ✅ Pipeline error handling (47)
- ✅ Pipeline performance optimization (48)
- ✅ Output redirection (>) (49)
- ✅ Append redirection (>>) (50)
- ✅ Input redirection (<) (51)
- ✅ Background processes (&) (52)
- ✅ Job control (jobs, fg) (53)
- ✅ Environment variables (env) (54)
- ✅ Variable expansion ($VAR, ${VAR}) (55)
- ✅ Command substitution ($()) (56)
- ✅ Here documents (<<) (57)
- ✅ Script file execution (58)
- ✅ If conditions (if-then-else-fi) (59)
- ✅ While loops (while-do-done) (60)
- ✅ For loops (for-in-do-done) (61)
- ✅ Functions (62)
- ✅ Multiple commands (;) (63)
- ✅ Interactive multi-line input (64)
- ✅ Parser refactoring (65)
- Command execution unification (66)
- Error handling improvements (67)
- Documentation & flow diagrams (68)
- Test organization (69)
- Phase 3 integration tests (70)

### Phase 4: Interactive Features (71-85)
- Command history basics (71-72)
- History persistence & search (73-74)
- Arrow key navigation (75-76)
- Tab completion framework (77-78)
- Syntax highlighting basics (79-80)
- Prompt customization (81-82)
- Terminal handling (83-84)
- Phase 4 integration (85)

### Phase 5: Extensions & Polish (86-100)
- Plugin system architecture (86-87)
- Configuration file support (88-89)
- Performance optimizations (90-91)
- Advanced patterns (92-93)
- Network capabilities (94-95)
- Final polish (96-97)
- Comprehensive documentation (98)
- Benchmarks & profiling (99)
- 🎉 Project completion! (100)

## Next: Command Execution Unification (Commit #66)

Unify `execute_command` and `execute_command_get_output`:
- Create a common internal execution function
- Reduce code duplication
- Improve error handling consistency