# rucli - Rust CLI Tool

🎯 **100 PR Challenge**: Building a feature-rich CLI tool in 100 PRs

## Progress: 59/100 PRs 🎉

[■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□]

## Latest Changes (PR #59)

- Added if-then-else conditional statements
- Support for single-line if syntax
- Condition command output is preserved
- Fixed duplicate pipeline checks in parser
- Comprehensive test coverage for conditionals

## Usage

### Conditional Statements (New!)

**Basic if-then syntax**:
```bash
> if echo "Checking..."; then echo "Success!"; fi
Checking...
Success!

> if cat nonexistent.txt; then echo "Found"; else echo "Not found"; fi
Not found
```

**With command success/failure**:
```bash
> if pwd; then echo "Directory exists"; fi
/home/user/rucli
Directory exists

> if cat missing.txt; then echo "OK"; else echo "File missing"; fi
File missing
```

**In scripts**:
```bash
#!/usr/bin/env rucli
# check.rsh - Conditional logic example

if ls; then echo "Files found in directory"; fi

env STATUS=active
if echo $STATUS; then echo "Status is $STATUS"; fi

if cat config.txt; then
    echo "Config loaded"
else
    echo "No config file, using defaults"
fi
```

**Current limitations**:
- ✅ Single-line if statements
- ✅ Basic then/else branches
- ❌ elif (else if) - not yet supported
- ❌ Multi-line if in scripts - not yet supported
- ❌ Nested if statements - not yet supported
- ❌ Conditions with pipelines - not yet supported

### Interactive Mode
```bash
$ cargo run
Hello, rucli!
> if echo test; then echo OK; fi
test
OK
> exit
good bye
```

### Script Mode
```bash
$ cargo run -- script.rsh
# or after building:
$ rucli script.rsh
```

## Complete Feature Set

**Control Flow**:
- If-then-else conditionals (NEW!)
- Background execution with `&`
- Pipeline chaining with `|`

**File Operations**: `cat`, `write`, `cp`, `mv`, `rm`  
**Directory Operations**: `ls`, `cd`, `pwd`, `mkdir`  
**Search Operations**: `find`, `grep`  
**Environment**: `env` - manage environment variables  
**Job Control**: `jobs`, `fg` - background job management  
**Utilities**: `echo`, `repeat`, `sleep`, `alias`, `version`, `help`, `exit`

**Operators**:
- `|` - Pipe commands together
- `>` - Redirect output to file
- `>>` - Append output to file
- `<` - Input from file
- `&` - Background execution
- `<<` - Here document
- `<<-` - Here document with tab stripping
- `;` - Command separator
- `if-then-fi` - Conditional execution

**Expansion Features**:
- `$VAR` - Basic variable expansion
- `${VAR}` - Brace notation for clear boundaries
- `$(command)` - Command substitution with full nesting support

## Examples

### Conditional Scripts

**backup_smart.rsh**:
```bash
#!/usr/bin/env rucli
# Smart backup with existence checking

env BACKUP_DIR=backups
if mkdir $BACKUP_DIR; then
    echo "Created new backup directory"
else
    echo "Using existing backup directory"
fi

if cp -r src $BACKUP_DIR/; then
    echo "Source files backed up successfully"
else
    echo "Warning: Failed to backup source files"
fi
```

**deploy_safe.rsh**:
```bash
#!/usr/bin/env rucli
# Safe deployment with checks

echo "Starting deployment..."

if cat VERSION; then
    echo "Version file found"
    if mkdir dist; then
        echo "Created dist directory"
        cp -r src dist/
        echo "Deployment complete"
    else
        echo "Error: Could not create dist directory"
    fi
else
    echo "Error: VERSION file not found"
fi
```

## Project Structure

```
rucli/
├── src/
│   ├── main.rs         # Entry point with mode detection
│   ├── commands.rs     # Command definitions + if execution
│   ├── parser.rs       # Input parsing + if statement parser
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
    ├── conditionals.rsh # If-then-else examples
    ├── setup.rsh        # Project setup script
    └── deploy.rsh       # Deployment script
```

## Testing

```bash
cargo test              # Run all tests
cargo test if_condition # Test conditionals
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
- [x] If conditions (59)
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

**Next**: While loops (`while condition; do commands; done`) in PR #60! 🔁