# rucli - Rust CLI Tool

100 Commit Challenge: Building a feature-rich CLI tool in 100 commits

Progress: 71/100 Commits

## Latest Changes (Commit #71)

- **Command History Basics**: Added comprehensive command history functionality
- Users can now view command history with the `history` command
- Automatic history tracking for all commands (interactive and script modes)
- Smart deduplication prevents consecutive identical commands from cluttering history
- Clean formatting with right-aligned numbering (up to 1000 commands)
- Compatible with all existing features: pipelines, redirects, functions, variables, etc.
- Each session maintains independent history storage
- Empty commands and whitespace-only input are filtered out

## Usage

### Command History

**View command history:**

```bash
> echo hello
hello
> pwd
/home/user
> ls
file1.txt file2.txt
> history
   1  echo hello
   2  pwd
   3  ls
   4  history
```

**History features:**

- **Automatic tracking**: All commands are automatically added to history
- **Smart deduplication**: Consecutive identical commands appear only once
- **Clean formatting**: Commands numbered with right-aligned formatting
- **Session-scoped**: Each rucli session has independent history
- **Error tolerance**: Even commands that fail are recorded in history
- **Capacity limit**: Stores up to 1000 commands (oldest removed when exceeded)

**Works with all command types:**

```bash
> echo test | grep t        # Pipelines
> echo hello > file.txt     # Redirects
> sleep 5 &                 # Background jobs
> if pwd; then echo ok; fi  # Control structures
> function test() { echo hi; }  # Function definitions
> env VAR=value             # Environment variables
> history                   # All recorded in history
```

### Script Files

History works in script mode too:

**example.rsh:**

```bash
#!/usr/bin/env rucli
echo "Script starting..."
pwd
history  # Shows script commands executed so far
echo "Script complete!"
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

**History:** `history` - view command history

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

### History Examples

**Basic usage:**

```bash
> echo "Hello World"
Hello World
> pwd
/home/user/projects
> echo "Testing history"
Testing history
> history
   1  echo "Hello World"
   2  pwd
   3  echo "Testing history"
   4  history
```

**With complex commands:**

```bash
> write data.txt "sample content"
File written successfully: data.txt
> cat data.txt | grep sample
sample content
> echo result > output.txt
> history
   1  write data.txt "sample content"
   2  cat data.txt | grep sample
   3  echo result > output.txt
   4  history
```

**Script with history:**

```bash
#!/usr/bin/env rucli
# history_demo.rsh

echo "Demonstrating history in scripts"
pwd
ls
echo "Commands executed:"
history
```

## Project Structure

```
rucli/
├── src/
│   ├── main.rs         # Entry point with history integration
│   ├── commands.rs     # Command definitions (including History)
│   ├── parser/         # Modular parser
│   │   ├── mod.rs      # Public interface (history parsing)
│   │   ├── basic.rs    # Basic commands
│   │   ├── file_ops.rs # File operations
│   │   ├── control.rs  # Control structures
│   │   ├── operators.rs# Operators
│   │   └── utils.rs    # Utilities
│   ├── handlers.rs     # Command implementations (handle_history)
│   ├── history.rs      # History storage and management
│   ├── functions.rs    # Function storage
│   ├── environment.rs  # Variables & expansions
│   ├── pipeline.rs     # Pipeline execution
│   ├── redirect.rs     # I/O redirection
│   ├── job.rs          # Background jobs
│   ├── alias.rs        # Command aliases
│   └── error.rs        # Error handling
├── tests/
│   ├── integration_tests.rs  # Comprehensive integration tests
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

- **No nested control structures**: Loops and conditionals cannot be nested within each other
- **No arithmetic operations**: Mathematical calculations are not supported
- **Limited pattern matching**: Glob patterns are basic
- **No arrays or complex data types**: Only simple string variables
- **Session-only history**: History is not persisted between application restarts

These limitations are due to the string-based parser implementation. A future version with a proper tokenizer and AST-based parser would address these issues.

## Roadmap

### Phase 3: Advanced Features & Refactoring (46-70) - COMPLETED ✅

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

### Phase 4: Interactive Features (71-85) - IN PROGRESS 🚧

- ✅ **Command history basics (71)** ← COMPLETED!
- History persistence (file save/load) (72)
- History search (Ctrl+R equivalent) (73)
- History navigation (number-based execution) (74)
- Arrow key navigation basics (75)
- Line editing with arrows (76)
- Tab completion framework (77)
- Command/file completion (78)
- Syntax highlighting basics (79)
- Error highlighting (80)
- Prompt customization (81)
- Shell shortcuts (Ctrl+A, Ctrl+E) (82)
- Terminal resize handling (83)
- Session management (84)
- Phase 4 integration (85)

### Phase 5: Extensions & Polish (86-100) - PLANNED 🔮

- Plugin system architecture (86-87)
- Configuration file support (.ruclirc) (88-89)
- Performance optimizations (90-91)
- Advanced glob patterns (92-93)
- Network capabilities (94-95)
- Final polish (96-97)
- Comprehensive documentation (98)
- Benchmarks & profiling (99)
- 🎉 Project completion celebration! (100)

## Next: History Persistence (Commit #72)

Add file-based history persistence:

- Save history to ~/.rucli_history on exit
- Load previous history on startup
- Configurable history file location
- Handle file permissions and errors gracefully
- Merge session history with file history

---

**Progress: 71/100 commits completed** 🎯
**Current Phase: Interactive Features (Phase 4)** ⚡
**Next Milestone: History Persistence** 💾
