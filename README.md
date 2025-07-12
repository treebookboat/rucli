# rucli - Rust CLI Tool

🎯 **100 PR Challenge**: Building a feature-rich CLI tool in 100 PRs

## Progress: 57/100 PRs 🎉

[■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□]

## Latest Changes (PR #57)

- Added here document support with `<<EOF` syntax
- Implemented strip indent feature with `<<-EOF`
- Multi-line input mode with `heredoc>` prompt
- Variable expansion and command substitution work in heredocs
- Refactored main loop for cleaner code structure
- Comprehensive test coverage for all heredoc patterns

## Usage

```bash
$ cargo run
> cat <<EOF
heredoc> Hello World
heredoc> This is a multi-line
heredoc> text input
heredoc> EOF
Hello World
This is a multi-line
text input

> env NAME=Alice
> cat <<MESSAGE
heredoc> Dear $NAME,
heredoc> Welcome to $(echo rucli)!
heredoc> MESSAGE
Dear Alice,
Welcome to rucli!
```

## Here Documents

**Basic Syntax**:
```bash
> cat <<EOF
heredoc> Line 1
heredoc> Line 2
heredoc> EOF
Line 1
Line 2
```

**Strip Leading Tabs** (`<<-`):
```bash
> cat <<-END
heredoc> 	This tab will be removed
heredoc> 		Two tabs: only first removed
heredoc>     Spaces are preserved
heredoc> END
This tab will be removed
	Two tabs: only first removed
    Spaces are preserved
```

**Variable Expansion**:
```bash
> env USER=Bob
> env DIR=/home/bob
> cat <<DOC
heredoc> User: $USER
heredoc> Home: $DIR
heredoc> Shell: $(echo $0)
heredoc> DOC
User: Bob
Home: /home/bob
Shell: rucli
```

**With Redirects**:
```bash
> cat <<CONFIG > app.conf
heredoc> server=localhost
heredoc> port=8080
heredoc> debug=true
heredoc> CONFIG
> cat app.conf
server=localhost
port=8080
debug=true
```

**Custom Delimiters**:
```bash
> grep error <<END_OF_LOG
heredoc> [INFO] Starting application
heredoc> [ERROR] Connection failed
heredoc> [INFO] Retrying...
heredoc> [ERROR] Timeout
heredoc> END_OF_LOG
[ERROR] Connection failed
[ERROR] Timeout
```

## Complete Expansion System

**Processing Order**:
1. **Variable Expansion** (`$VAR`, `${VAR}`) - First pass
2. **Command Substitution** (`$(command)`) - Second pass
3. **Command Parsing** - Final pass

This applies to both regular commands and here document content.

## Advanced Examples

**Configuration Files**:
```bash
> cat <<EOF > config.yaml
heredoc> database:
heredoc>   host: $(echo localhost)
heredoc>   port: 5432
heredoc>   user: $DB_USER
heredoc> EOF
```

**Multi-line Scripts**:
```bash
> cat <<SCRIPT > setup.sh
heredoc> #!/bin/bash
heredoc> echo "Setting up environment..."
heredoc> mkdir -p $(pwd)/data
heredoc> export PATH=$PATH:$(pwd)/bin
heredoc> echo "Setup complete!"
heredoc> SCRIPT
```

**SQL Queries** (simulated):
```bash
> cat <<SQL
heredoc> SELECT * FROM users
heredoc> WHERE created_at > '2024-01-01'
heredoc>   AND status = 'active'
heredoc> ORDER BY name;
heredoc> SQL
```

## Commands

**File Operations**: `cat`, `write`, `cp`, `mv`, `rm`  
**Directory Operations**: `ls`, `cd`, `pwd`, `mkdir`  
**Search Operations**: `find`, `grep`  
**Environment**: `env` - manage environment variables
**Job Control**: `jobs`, `fg`  
**Utilities**: `echo`, `repeat`, `sleep`, `alias`, `version`, `help`, `exit`

**Operators**:
- `|` - Pipe commands together
- `>` - Redirect output to file
- `>>` - Append output to file
- `<` - Input from file
- `&` - Background execution
- `<<` - Here document
- `<<-` - Here document with tab stripping

**Expansion Features**:
- `$VAR` - Basic variable expansion
- `${VAR}` - Brace notation for clear boundaries
- `$(command)` - Command substitution with full nesting support

## Project Structure

```
rucli/
├── src/
│   ├── main.rs         # Entry point with heredoc support
│   ├── commands.rs     # Command definitions & execution
│   ├── parser.rs       # Input parsing with heredoc detection
│   ├── handlers.rs     # Command implementations
│   ├── environment.rs  # Variables & substitution engine
│   ├── pipeline.rs     # Pipeline execution logic
│   ├── redirect.rs     # I/O redirection handling
│   ├── job.rs          # Background job management
│   ├── alias.rs        # Command alias system
│   └── error.rs        # Error types & handling
└── tests/
    ├── cli_tests.rs
    └── integration_tests.rs
```

## Testing

```bash
cargo test              # Run all tests
cargo test heredoc      # Test here documents
cargo test environment  # Test variables & substitutions
cargo test integration  # Test combined features
cargo run -- --debug    # Run with debug logging
```

## Roadmap

**Phase 3: Advanced Features (46-65)** 🚀
- [x] Pipelines (46-48)
- [x] Redirections (49-51)
- [x] Background execution (52)
- [x] Job management (53)
- [x] Environment variables (54)
- [x] Variable expansion (55)
- [x] Command substitution (56)
- [x] Here documents (57)
- [ ] Script file execution (58)
- [ ] If conditions (59)
- [ ] While loops (60)
- [ ] For loops (61)
- [ ] Functions (62)
- [ ] Error handling in scripts (63)
- [ ] Script debugging (64)
- [ ] Phase 3 integration tests (65)

**Phase 4: Interactive Features (66-85)**
- [ ] Command history & navigation
- [ ] Tab completion
- [ ] Syntax highlighting

**Phase 5: Extensions (86-100)**
- [ ] Plugin system
- [ ] Configuration files
- [ ] Performance optimization

---

**Next**: Script file execution (`rucli script.rsh`) in PR #58! 📜