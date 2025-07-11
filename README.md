# rucli - Rust CLI Tool

🎯 **100 PR Challenge**: Building a feature-rich CLI tool in 100 PRs

## Progress: 55/100 PRs 🎉

[■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□]

## Latest Changes (PR #55)

- Added comprehensive variable expansion with `$VAR` and `${VAR}` syntax
- Integrated variable expansion into parser for all commands
- Support for mixed expansion styles and alphanumeric variable names
- Robust error handling for malformed syntax and missing variables
- Variable expansion works in pipelines, redirects, and background jobs
- Session variables take priority over system variables

## Usage

```bash
$ cargo run
> env NAME=world
> echo Hello $NAME           # Hello world
> echo ${NAME}!              # world!
> cat ${NAME}.txt            # Reads world.txt
> env PREFIX=test
> cp $PREFIX.txt ${PREFIX}_backup.txt  # test.txt -> test_backup.txt
```

## Variable Expansion System

**Basic Expansion (`$VAR`)**:
```bash
> env USER=alice
> echo Welcome $USER         # Welcome alice
> echo $USER/documents       # alice/documents

# Fish-style flexible naming (numeric start allowed)
> env 1ST_USER=bob
> echo $1ST_USER             # bob
> env 123=numeric
> echo $123                  # numeric
```

**Brace Expansion (`${VAR}`)**:
```bash
> env PREFIX=myfile
> echo $PREFIXname           # (empty - looks for PREFIXname variable)
> echo ${PREFIX}name         # myfilename (PREFIX + name)
> echo ${PREFIX}.txt         # myfile.txt
> echo ${123}data            # numericdata (if 123=numeric)
```

**Variable Naming Rules (Fish-Inspired)**:
- **Valid characters**: `a-z`, `A-Z`, `0-9`, `_` (underscore)
- **No restrictions**: Numeric start allowed (`$123`, `$1ST_VAR`)
- **Case sensitive**: `$VAR` ≠ `$var`
- **Priority**: Session variables > System variables

**Mixed Usage**:
```bash
> env HOST=server
> env PORT=8080
> env 1ST_DB=primary
> echo Connect to $HOST:${PORT}/api/${1ST_DB}    # server:8080/api/primary
```

**Error Handling**:
```bash
> echo $NONEXISTENT          # (empty string)
> echo ${MISSING}            # (empty string)
> echo ${INCOMPLETE          # ${INCOMPLETE (preserved as-is)
> echo ${}                   # ${} (preserved as-is)
```

## Environment Variable System

**Session Variables**: Variables set within rucli using `env VAR=value`
- Stored separately from system environment
- Persist throughout rucli session
- Don't affect system environment after exit
- Take precedence over system variables with same name

**Variable Expansion Priority**: Session Variables > System Variables

```bash
# Safe PATH customization with expansion
> env PATH=/custom/bin
> echo Current path: $PATH   # Shows custom path
> echo $PATH/mycommand       # /custom/bin/mycommand
> exit                       # System PATH unchanged
```

## Advanced Usage Examples

**Advanced Usage Examples

**File Operations with Flexible Variables**:
```bash
> env 1ST_SOURCE=data.csv
> env 2ND_DEST=backup
> env 123=logs
> cp $1ST_SOURCE ${2ND_DEST}/$1ST_SOURCE     # Copy data.csv to backup/data.csv  
> cat ${123}/app.log | grep error > ${2ND_DEST}/errors.log
```

**Numeric Variables for Iteration-Style Operations**:
```bash
> env 1=first.txt
> env 2=second.txt  
> env 3=third.txt
> cat $1 $2 $3 > combined.txt                # Combine all files
> echo Processing ${1}, ${2}, ${3}           # Processing first.txt, second.txt, third.txt
```

**Pipeline Integration**:
```bash
> env PATTERN=ERROR
> env LOGFILE=app.log
> cat $LOGFILE | grep $PATTERN | wc -l    # Count errors in log
> find . -name "*.${PATTERN,,}" | head -5 # Find pattern files
```

**Background Jobs with Variables**:
```bash
> env BACKUP_DIR=/backup
> env SOURCE_DIR=/data
> cp -r $SOURCE_DIR $BACKUP_DIR &         # Background backup
[1] ThreadId(2)
> jobs
[1]+  Running    cp -r /data /backup
```

**Complex Variable Scenarios**:
```bash
> env PROJECT=myapp
> env VERSION=1.0
> env BUILD=release
> env 1ST_ENV=prod
> echo Building ${PROJECT}-v${VERSION}-${BUILD}-${1ST_ENV}.tar.gz
Building myapp-v1.0-release-prod.tar.gz

# Numeric sequence variables
> env 1=alpha
> env 2=beta  
> env 3=gamma
> echo Deployment sequence: $1 -> $2 -> $3
Deployment sequence: alpha -> beta -> gamma
```

## Commands

**File Operations**: `cat`, `write`, `cp`, `mv`, `rm`  
**Directory Operations**: `ls`, `cd`, `pwd`, `mkdir`  
**Search Operations**: `find`, `grep`  
**Environment**: `env` - manage environment variables with expansion support
**Job Control**: `jobs`, `fg`  
**Utilities**: `echo`, `repeat`, `sleep`, `alias`, `version`, `help`, `exit`

**Operators**:
- `|` - Pipe commands together (with variable expansion)
- `>` - Redirect output (with variable expansion)
- `>>` - Redirect output append (with variable expansion)
- `<` - Input redirect from file (with variable expansion)
- `&` - Background execution (with variable expansion)

**Variable Expansion**:
- `$VAR` - Basic variable expansion
- `${VAR}` - Brace notation for clear boundaries
- Works in all commands, arguments, and file paths

## Project Structure

```
rucli/
├── src/
│   ├── main.rs         # Entry point
│   ├── commands.rs     # Command definitions & execution
│   ├── parser.rs       # Input parsing with variable expansion
│   ├── handlers.rs     # Command implementations
│   ├── environment.rs  # Environment variables & expansion engine
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
cargo test environment  # Test environment & expansion
cargo test expansion    # Test variable expansion
cargo test integration  # Test env + expansion integration
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
- [ ] Command substitution (56)
- [ ] Here documents (57)
- [ ] Scripting support (58-65)

**Phase 4: Interactive Features (66-85)**
- [ ] Command history & navigation
- [ ] Tab completion
- [ ] Syntax highlighting

**Phase 5: Extensions (86-100)**
- [ ] Plugin system
- [ ] Configuration files
- [ ] Performance optimization

---

**Next**: Command substitution (`$(command)` syntax) in PR #56! 🔄