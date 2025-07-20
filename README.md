# rucli - Rust CLI Tool

100 Commit Challenge: Building a feature-rich CLI tool in 100 commits

Progress: 66/100 Commits

## Latest Changes (Commit #66)

- **Interactive input refactoring**: Simplified multi-line input handling
- Removed unused `BlockState` enum - now uses simple boolean return
- Fixed nested loop processing with proper depth tracking
- Added missing keyword handlers:
  - `then` for if statements
  - `function`, `{`, `}` for function definitions
  - `else` for if-else statements
- Improved code quality by fixing all clippy warnings
- Updated all tests to match new API

## Usage

### Interactive Multi-line Input

Execute commands with natural multi-line syntax:

```bash
# For loops
> for i in 1 2 3
>> do
>>   echo $i
>> done
1
2
3

# If-then-else statements
> if pwd
>> then
>>   echo "Directory exists"
>> else
>>   echo "Error"
>> fi

# Function definitions
> function greet()
>> {
>>   echo Hello
>>   echo World
>> }
```

**Note**: Nested control structures (loops within loops) are not currently supported due to parser limitations. Each control structure must be completed before starting another.

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

**Functions:**
```bash
function name() { commands; }
```

**Limitations:**
- Nested control structures are not supported (e.g., for loops inside for loops)
- Each control structure must be completed before starting another
- Complex scripts should use functions to organize logic

### Complete Feature Set

**Control Flow:**
- If-then-else conditionals
- While loops
- For loops
- Functions
- Background execution with `&`
- Pipeline chaining with `|`

*Note: Nested control structures are not supported in the current implementation*

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

### Basic Scripts

**simple_loop.rsh:**
```bash
#!/usr/bin/env rucli
# Simple for loop example

for file in *.txt
do
    echo "Processing: $file"
    cat $file | grep ERROR
done
```

**backup_script.rsh:**
```bash
#!/usr/bin/env rucli
# Use functions to organize logic

function backup_file() {
    cp $1 $1.bak
    echo "Backed up: $1"
}

function process_directory() {
    for file in *.txt
    do
        backup_file $file
    done
}

# Execute
process_directory
```

## Project Structure

```
rucli/
├── src/
│   ├── main.rs         # Entry point (refactored input handling)
│   ├── commands.rs     # Command definitions
│   ├── parser/         # Modular parser
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
cargo test nested       # Test nested structures
cargo clippy           # Check code quality (now clean!)
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
- ✅ Interactive input refactoring (66)
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

## Next: Error Handling Improvements (Commit #67)

Improve error handling consistency:
- Unify error types and messages
- Add better error context
- Improve error recovery in interactive mode
- Add proper error codes for script mode