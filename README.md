# rucli - Rust CLI Tool

100 Commit Challenge: Building a feature-rich CLI tool in 100 commits

Progress: 71/100 Commits

## Latest Changes (Commit #71)

- **History Navigation**: Execute previous commands by their history number!
- Use `history n` to re-execute the nth command from history
- Provides bash-like `!n` functionality with familiar syntax
- Error handling for invalid history positions
- Foundation for advanced history expansion features

## Usage

### History Navigation

**Basic navigation:**
```bash
$ rucli
> echo hello world
hello world
> echo goodbye
goodbye
> cat file.txt
file content
> history
   1  echo hello world
   2  echo goodbye
   3  cat file.txt
   4  history
> history 1
hello world
> history 3
file content
```

**Error handling:**
```bash
> history 0
history: 0: history position out of range
> history 999
history: 999: history position out of range
> history abc
Usage: history [number | search <query>]
```

**Complex commands:**
```bash
> echo test | grep t
test
> for i in 1 2 3; do echo $i; done
1
2
3
> history
   1  echo test | grep t
   2  for i in 1 2 3; do echo $i; done
   3  history
> history 2
1
2
3
```

### History Features

- **View history**: `history` - displays numbered command list
- **Search history**: `history search <query>` - case-insensitive partial matching
- **Execute from history**: `history n` - re-execute the nth command ← NEW!
- Persistence between sessions via RUCLI_HISTFILE
- Automatic deduplication of consecutive commands
- Up to 1000 commands stored

### Complete Feature Set

**Interactive Features:**
- Command history with `history` command
- History persistence between sessions
- History search with `history search <query>`
- **History navigation with `history n`** ← NEW!
- Automatic deduplication of consecutive commands
- Up to 1000 commands stored

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

**Environment:** 
- `env` - manage environment variables
- Variable expansion with `$VAR` and `${VAR}`
- Command substitution with `$(command)`

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

### History Navigation Examples

**Re-execute previous commands:**
```bash
$ rucli
> write test.txt "Hello, World!"
File written successfully: test.txt
> cat test.txt
Hello, World!
> rm test.txt
> history
   1  write test.txt "Hello, World!"
   2  cat test.txt
   3  rm test.txt
   4  history
> history 1
File written successfully: test.txt
> history 2
Hello, World!
```

**Navigate through session history:**
```bash
> pwd
/home/user
> cd /tmp
> pwd
/tmp
> history
   1  pwd
   2  cd /tmp
   3  pwd
   4  history
> history 1
/tmp
```

**Re-execute complex commands:**
```bash
> echo "Line 1" > file.txt
> echo "Line 2" >> file.txt
> cat file.txt | grep Line
Line 1
Line 2
> history 3
Line 1
Line 2
```

## Environment Variables

- `RUCLI_HISTFILE` - Custom history file location (default: `./.rucli_history`)
- `HOME` - Used for `cd ~` command
- `OLDPWD` - Previous directory for `cd -`

## Project Structure

```
rucli/
├── src/
│   ├── main.rs         # Entry point with history persistence
│   ├── commands.rs     # Command definitions with HistoryAction enum
│   ├── parser/         # Modular parser
│   │   ├── mod.rs      # Public interface
│   │   ├── basic.rs    # Basic commands & history parsing
│   │   ├── file_ops.rs # File operations
│   │   ├── control.rs  # Control structures
│   │   ├── operators.rs# Operators
│   │   └── utils.rs    # Utilities
│   ├── handlers.rs     # Command implementations with history execution
│   ├── history.rs      # History with navigation functionality
│   ├── functions.rs    # Function storage
│   ├── environment.rs  # Variables & expansions
│   ├── pipeline.rs     # Pipeline execution
│   ├── redirect.rs     # I/O redirection
│   ├── job.rs          # Background jobs
│   ├── alias.rs        # Command aliases
│   └── error.rs        # Error handling
├── tests/
│   ├── integration_tests.rs  # Comprehensive tests
│   └── cli_tests.rs          # CLI interaction tests
└── examples/
    ├── multi_line.rsh    # Multi-line examples
    ├── functions.rsh     # Function examples
    └── system.rsh        # System scripts
```

## Testing

```bash
cargo test              # Run all tests
cargo test history      # Test history functionality
cargo clippy           # Check code quality
cargo run -- test.rsh   # Run a script file
```

## Known Limitations

- **Session-specific state**: Aliases and functions are not persisted between sessions
- **No nested control structures**: Loops and conditionals cannot be nested within each other
- **No arithmetic operations**: Mathematical calculations are not supported
- **Limited pattern matching**: Glob patterns are basic
- **No arrays or complex data types**: Only simple string variables

## Roadmap

### Phase 3: Advanced Features (46-70) - COMPLETED ✅

- ✅ All commits implemented

### Phase 4: Interactive Features (71-85) - IN PROGRESS 🚧

- ✅ History navigation (number-based execution) (71) ← DONE!
- History expansion (!n, !!, !string) (72)
- Arrow key navigation basics (73)
- Line editing with arrows (74)
- Command line cursor movement (75)
- Tab completion framework (76)
- Command/file completion (77)
- Syntax highlighting basics (78)
- Error highlighting (79)
- Prompt customization (80)
- Shell shortcuts (Ctrl+A, Ctrl+E) (81)
- Terminal resize handling (82)
- Session management (83)
- Configuration loading (.ruclirc) (84)
- Phase 4 integration (85)

### Phase 5: Extensions & Polish (86-100) - PLANNED 🔮

- Plugin system architecture (86-87)
- Performance optimizations (88-89)
- Advanced glob patterns (90-91)
- Network capabilities (92-93)
- Security enhancements (94)
- Final polish (95-96)
- Comprehensive documentation (97)
- Benchmarks & profiling (98)
- Release preparation (99)
- 🎉 Project completion celebration! (100)

## Next: History Expansion (Commit #72)

Implement bash-style history expansion:

- `!!` - Execute the last command
- `!n` - Execute nth command (alternative syntax)
- `!-n` - Execute nth command from the end
- `!string` - Execute most recent command starting with string

---

**Progress: 71/100 commits completed** 🎯
**Current Phase: Interactive Features (Phase 4)** ⚡
**Next Milestone: History Expansion** 🔥