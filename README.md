# rucli - Rust CLI Tool

100 Commit Challenge: Building a feature-rich CLI tool in 100 commits

Progress: 72/100 Commits

## Latest Changes (Commit #72)

- **History Expansion**: Bash-style history expansion is now supported!
- Use `!!` to execute the last command
- Use `!n` to execute the nth command from history
- Use `!-n` to execute the nth command from the end
- Use `!string` to execute the most recent command starting with string
- Comprehensive error handling for invalid expansion patterns
- Full integration with existing command processing pipeline

## Usage

### History Expansion

**Basic expansion patterns:**

```bash
$ rucli
> echo hello world
hello world
> echo goodbye
goodbye
> cat file.txt
file content
> !!
file content
> !1
hello world
> !echo
hello world
```

**Error handling:**

```bash
> !999
bash: !999: event not found
> !nonexistent
bash: !nonexistent: event not found
> !
bash: !: event not found
```

**Complex scenarios:**

```bash
> for i in 1 2 3; do echo $i; done
1
2
3
> pwd
/home/user
> !for
1
2
3
> echo "Current dir: $(pwd)"
Current dir: /home/user
> !!
Current dir: /home/user
```

### History Navigation Combined

**Multiple ways to access history:**

```bash
> echo test1
test1
> echo test2
test2
> echo test3
test3
> history
   1  echo test1
   2  echo test2
   3  echo test3
   4  history

# Different access methods:
> history 2
test2
> !test2
test2
> !-1
test2
```

### History Features

- **View history**: `history` - displays numbered command list
- **Search history**: `history search <query>` - case-insensitive partial matching
- **Execute from history**: `history n` - re-execute the nth command
- **History expansion**: `!!`, `!n`, `!-n`, `!string` - bash-style expansion ← NEW!
- Persistence between sessions via RUCLI_HISTFILE
- Automatic deduplication of consecutive commands
- Up to 1000 commands stored

### Complete Feature Set

**Interactive Features:**

- Command history with `history` command
- History persistence between sessions
- History search with `history search <query>`
- History navigation with `history n`
- **Bash-style history expansion (!!, !n, !string)** ← NEW!
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
- **History expansion**: `!!`, `!n`, `!-n`, `!string` ← NEW!

## Examples

### History Expansion Examples

**Quick re-execution:**

```bash
$ rucli
> ls -la
total 16
drwxr-xr-x  4 user  group  128 Aug  1 08:00 .
drwxr-xr-x  3 user  group   96 Aug  1 07:30 ..
-rw-r--r--  1 user  group   42 Aug  1 08:00 file.txt
> !!
total 16
drwxr-xr-x  4 user  group  128 Aug  1 08:00 .
drwxr-xr-x  3 user  group   96 Aug  1 07:30 ..
-rw-r--r--  1 user  group   42 Aug  1 08:00 file.txt
```

**Numbered access:**

```bash
> echo "Step 1"
Step 1
> echo "Step 2"
Step 2
> echo "Step 3"
Step 3
> !1
Step 1
> !-1
Step 1
> !3
Step 3
```

**Prefix matching:**

```bash
> write config.txt "debug=true"
File written successfully: config.txt
> cat config.txt
debug=true
> find . -name "*.txt"
./config.txt
> !write
File written successfully: config.txt
> !cat
debug=true
> !find
./config.txt
```

**Complex command expansion:**

```bash
> for i in $(echo 1 2 3); do echo "Number: $i"; done
Number: 1
Number: 2
Number: 3
> echo "Repeating previous loop"
Repeating previous loop
> !for
Number: 1
Number: 2
Number: 3
```

## Environment Variables

- `RUCLI_HISTFILE` - Custom history file location (default: `./.rucli_history`)
- `HOME` - Used for `cd ~` command
- `OLDPWD` - Previous directory for `cd -`

## Project Structure

```
rucli/
├── src/
│   ├── main.rs         # Entry point with history expansion processing
│   ├── commands.rs     # Command definitions with HistoryAction enum
│   ├── parser/         # Modular parser
│   │   ├── mod.rs      # Public interface with expansion exports
│   │   ├── basic.rs    # Basic commands & history parsing
│   │   ├── file_ops.rs # File operations
│   │   ├── control.rs  # Control structures
│   │   ├── operators.rs# Operators
│   │   ├── expansion.rs# History expansion logic ← NEW!
│   │   └── utils.rs    # Utilities
│   ├── handlers.rs     # Command implementations with history execution
│   ├── history.rs      # History with navigation & expansion support
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
cargo test expansion    # Test history expansion
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

- ✅ History navigation (number-based execution) (71)
- ✅ History expansion (!n, !!, !string) (72) ← DONE!
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

## Next: Arrow Key Navigation (Commit #73)

Begin implementing terminal control for interactive line editing:

- Basic arrow key detection and handling
- Foundation for cursor movement and line editing
- Terminal raw mode management
- Preparation for advanced line editing features

---

**Progress: 72/100 commits completed** 🎯
**Current Phase: Interactive Features (Phase 4)** ⚡
**Next Milestone: Arrow Key Navigation** 🔥
