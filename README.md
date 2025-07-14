# rucli - Rust CLI Tool

🎯 **100 PR Challenge**: Building a feature-rich CLI tool in 100 PRs

## Progress: 58/100 PRs 🎉

[■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□]

## Latest Changes (PR #58)

- Added script file execution with `rucli script.rsh`
- Support for shebang `#!/usr/bin/env rucli`
- Comments and empty lines are skipped
- Scripts continue on error (bash-compatible)
- Refactored main into interactive and script modes
- Comprehensive test coverage for all script features

## Usage

### Interactive Mode (default)
```bash
$ cargo run
Hello, rucli!
> echo "Interactive mode"
Interactive mode
> exit
good bye
```

### Script Mode (new!)
```bash
$ cargo run -- script.rsh
# or after building:
$ rucli script.rsh
```

## Script Files

### Basic Script Example
```bash
#!/usr/bin/env rucli
# setup.rsh - Project setup script

echo "Setting up new project..."

# Create directory structure
mkdir -p src tests docs
cd src

# Create main file
write main.rs "fn main() {
    println!(\"Hello, world!\");
}"

# Create README
cd ..
write README.md "# My Project

A new Rust project created with rucli."

echo "Project setup complete!"
```

### Running Scripts

```bash
# Direct execution
$ rucli setup.rsh

# With debug output
$ rucli setup.rsh --debug

# Make executable (with shebang)
$ chmod +x setup.rsh
$ ./setup.rsh
```

### Script Features

**Comments and Shebang**:
```bash
#!/usr/bin/env rucli
# This is a comment
echo "Comments are ignored"  # Inline comments too

# Empty lines are also skipped

echo "Only commands are executed"
```

**Error Handling**:
```bash
echo "Before error"
cat nonexistent.txt  # Error, but script continues
echo "After error - still running!"
```

**All Shell Features Work**:
```bash
# Variables
env NAME=World
echo "Hello, $NAME"

# Command substitution
echo "Current time: $(date)"

# Pipelines
cat data.txt | grep pattern | wc -l

# Redirections
echo "Log entry" >> app.log

# Background jobs
sleep 5 &
echo "Background job started"
```

## Real-World Script Examples

### Backup Script
```bash
#!/usr/bin/env rucli
# backup.rsh - Daily backup script

env DATE=$(date +%Y%m%d)
env BACKUP_DIR=backups/$DATE

echo "Starting backup for $DATE..."
mkdir -p $BACKUP_DIR

# Backup source files
cp -r src $BACKUP_DIR/
cp -r docs $BACKUP_DIR/

# Create archive
cd backups
echo "Creating archive..."
# tar would go here in real implementation

echo "Backup completed: $BACKUP_DIR"
```

### Deployment Script
```bash
#!/usr/bin/env rucli
# deploy.rsh - Build and deploy script

echo "Building project..."
mkdir -p dist

# Copy files
cp -r src dist/
cp README.md dist/
cp LICENSE dist/

# Create version file
env VERSION=1.0.0
write dist/VERSION "Version: $VERSION
Built: $(date)"

echo "Build complete!"
echo "Files in dist:"
ls dist
```

### Test Runner Script
```bash
#!/usr/bin/env rucli
# test.rsh - Run various tests

echo "Running test suite..."

# Check required files
echo "Checking project structure..."
cat Cargo.toml > /dev/null
cat src/main.rs > /dev/null
echo "✓ Required files exist"

# Run different test categories
echo ""
echo "Running unit tests..."
# cargo test would go here

echo ""
echo "Running integration tests..."
# integration tests would go here

echo ""
echo "All tests completed!"
```

## Complete Feature Set

**Execution Modes**:
- Interactive shell mode (REPL)
- Script file execution
- Command line arguments support

**Script Features**:
- Shebang support (`#!/usr/bin/env rucli`)
- Comments (`#`) and empty lines ignored
- Error resilience (continues on error)
- All interactive features available

**Shell Features**:
- File operations: `cat`, `write`, `cp`, `mv`, `rm`
- Directory operations: `ls`, `cd`, `pwd`, `mkdir`
- Search operations: `find`, `grep`
- Environment variables: `env`, `$VAR`, `${VAR}`
- Command substitution: `$(command)`
- Pipelines: `cmd1 | cmd2`
- Redirections: `>`, `>>`, `<`
- Background execution: `&`
- Here documents: `<<EOF`

## Project Structure

```
rucli/
├── src/
│   ├── main.rs         # Entry point with mode detection
│   ├── commands.rs     # Command definitions
│   ├── parser.rs       # Input parsing
│   ├── handlers.rs     # Command implementations
│   ├── environment.rs  # Variables & expansions
│   ├── pipeline.rs     # Pipeline execution
│   ├── redirect.rs     # I/O redirection
│   ├── job.rs          # Background jobs
│   ├── alias.rs        # Command aliases
│   └── error.rs        # Error handling
├── tests/
│   └── integration_tests.rs
└── examples/
    ├── setup.rsh       # Project setup script
    ├── backup.rsh      # Backup script
    └── deploy.rsh      # Deployment script
```

## Testing

```bash
cargo test              # Run all tests
cargo test script       # Test script execution
cargo run -- test.rsh   # Run a test script
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
- [x] Script file execution (58)
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

**Next**: If conditions (`if command; then; fi`) in PR #59! 🔀