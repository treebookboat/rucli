# rucli - Rust CLI Tool

100 Commit Challenge: Building a feature-rich CLI tool in 100 commits

Progress: 70/100 Commits

## Latest Changes (Commit #70)

- **History Search**: Search through command history with partial matching!
- Case-insensitive search across all previous commands
- `history search <query>` finds all commands containing the query
- Excludes the current search command from results
- Empty query shows all history (except current command)

## Usage

### History Search

**Basic search:**
```bash
$ rucli
> echo hello world
hello world
> echo goodbye
goodbye
> cat file.txt
> history search echo
   1  echo hello world
   2  echo goodbye
```

**Case-insensitive matching:**
```bash
> echo HELLO
HELLO
> ECHO test
test
> history search echo
   1  echo HELLO
   2  ECHO test
```

**Partial matching:**
```bash
> cat important_file.txt
> write file.txt content
> rm file.txt
> history search file
   1  cat important_file.txt
   2  write file.txt content
   3  rm file.txt
```

**No results:**
```bash
> echo test
test
> history search xyz
No commands found matching 'xyz'
```

### Complete Feature Set

**Interactive Features:**
- Command history with `history` command
- History persistence between sessions
- **History search with `history search <query>`** ← NEW!
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

### History Search Examples

**Search for specific commands:**
```bash
$ rucli
> cd /home/user
> cd /tmp
> cd ~/documents
> history search cd
   1  cd /home/user
   2  cd /tmp
   3  cd ~/documents
```

**Complex search in scripts:**
```bash
> for i in 1 2 3; do echo $i; done
1
2
3
> while test -f lock; do sleep 1; done
> history search do
   1  for i in 1 2 3; do echo $i; done
   2  while test -f lock; do sleep 1; done
```

**Search with special characters:**
```bash
> echo "hello world" > output.txt
> cat < input.txt
> ls | grep txt
> history search >
   1  echo "hello world" > output.txt
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
│   ├── commands.rs     # Command definitions with History search
│   ├── parser/         # Modular parser
│   │   ├── mod.rs      # Public interface
│   │   ├── basic.rs    # Basic commands & history parsing
│   │   ├── file_ops.rs # File operations
│   │   ├── control.rs  # Control structures
│   │   ├── operators.rs# Operators
│   │   └── utils.rs    # Utilities
│   ├── handlers.rs     # Command implementations
│   ├── history.rs      # History with search functionality
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

- ✅ Commits 46-67: All implemented
- ✅ Command history basics (68)
- ✅ History persistence (69)
- ✅ **History search (70)** ← NEW!

### Phase 4: Interactive Features (71-85) - STARTING 🚧

- History navigation (number-based execution) (71)
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

## Next: History Navigation (Commit #71)

Implement number-based history execution:

- Execute commands by history number (!n equivalent)
- Support for negative indexing (!-n)
- Range-based history display
- Foundation for history expansion

---

**Progress: 70/100 commits completed** 🎯
**Current Phase: Interactive Features (Phase 4)** ⚡
**Next Milestone: History Navigation** 🔢