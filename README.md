# rucli - Rust CLI Tool

100 Commit Challenge: Building a feature-rich CLI tool in 100 commits

Progress: 67/100 Commits

## Latest Changes (Commit #67)

- **Script mode multi-line support**: Scripts can now use multi-line control structures
- Unified processing between interactive and script modes using `BlockInputCollector`
- Scripts can be written with natural formatting:
  ```bash
  for i in 1 2 3
  do
      echo "Number: $i"
  done
  ```
- Added error detection for incomplete blocks at end of file
- Maintained backward compatibility with single-line commands

## Usage

### Script Files

Create more readable scripts with proper formatting:

**example.rsh:**

```bash
#!/usr/bin/env rucli
# Multi-line script example

echo "Starting script..."

# For loops with proper indentation
for file in *.txt
do
    echo "Processing: $file"
    cat $file | grep ERROR
done

# If statements across multiple lines
if pwd
then
    echo "Current directory:"
    pwd
else
    echo "Cannot get directory"
fi

# Function definitions
function cleanup()
{
    echo "Cleaning up..."
    rm -f *.tmp
}

# Call the function
cleanup
echo "Script complete!"
```

Run with:

```bash
cargo run -- example.rsh
# or
./rucli example.rsh
```

### Interactive Mode

Interactive mode continues to work as before with multi-line input:

```bash
> for i in 1 2 3
>> do
>>   echo $i
>> done
1
2
3
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

_Note: Nested control structures are not supported in the current implementation_

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

### Script Examples

**backup_script.rsh:**

```bash
#!/usr/bin/env rucli
# Backup script with functions

function backup_file()
{
    if test -f $1
    then
        cp $1 $1.bak
        echo "Backed up: $1"
    else
        echo "File not found: $1"
    fi
}

# Process all text files
for file in *.txt
do
    backup_file $file
done

echo "Backup complete"
```

**system_check.rsh:**

```bash
#!/usr/bin/env rucli
# System check script

echo "System Check Report"
echo "=================="

# Check current directory
if pwd
then
    echo "Working directory: $(pwd)"
fi

# List files
echo ""
echo "Files in directory:"
ls

# Check for log files
echo ""
if find . *.log
then
    echo "Log files found"
else
    echo "No log files"
fi
```

## Project Structure

```
rucli/
├── src/
│   ├── main.rs         # Entry point (unified script/interactive handling)
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
    ├── multi_line.rsh  # Multi-line examples
    ├── functions.rsh   # Function examples
    └── system.rsh      # System scripts
```

## Testing

```bash
cargo test              # Run all tests
cargo test script       # Test script handling
cargo clippy           # Check code quality
cargo run -- test.rsh   # Run a script file
```

## Known Limitations

- **No nested control structures**: Loops and conditionals cannot be nested within each other
- **No arithmetic operations**: Mathematical calculations are not supported
- **Limited pattern matching**: Glob patterns are basic
- **No arrays or complex data types**: Only simple string variables

These limitations are due to the string-based parser implementation. A future version with a proper tokenizer and AST-based parser would address these issues.

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
- ✅ Script mode multi-line support (67)
- Error handling improvements (68)
- Documentation & flow diagrams (69)
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

## Next: Error Handling Improvements (Commit #68)

Improve error handling consistency:

- Unify error types and messages
- Add better error context
- Improve error recovery in interactive mode
- Add proper exit codes for different error types
