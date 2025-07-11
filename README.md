# rucli - Rust CLI Tool

🎯 **100 PR Challenge**: Building a feature-rich CLI tool in 100 PRs

## Progress: 54/100 PRs 🎉

[■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□]

## Latest Changes (PR #54)

- Added comprehensive environment variable management system
- Implemented `env` command for listing, showing, and setting variables
- Created session-specific variable storage (separate from system environment)
- Added support for system environment variable access with session override
- Environment variables persist within rucli session but don't affect system

## Usage

```bash
$ cargo run
> env                    # List all environment variables
PATH=/usr/bin:/bin
HOME=/home/user
...

> env TEST_VAR=hello     # Set session variable
> env TEST_VAR           # Show variable value
hello

> env PATH               # Show system variable (unchanged)
/usr/bin:/bin
```

## Environment Variable System

**Session Variables**: Variables set within rucli using `env VAR=value`
- Stored separately from system environment
- Persist throughout rucli session
- Don't affect system environment after exit
- Take precedence over system variables with same name

**System Variables**: Original environment variables from the system
- Accessed read-only from within rucli
- Include PATH, HOME, USER, etc.
- Remain unchanged by rucli operations

**Priority**: Session Variables > System Variables

```bash
# Example: Safe PATH customization
> env PATH               # Show system PATH
/usr/bin:/bin

> env PATH=/custom/path  # Set session PATH
> env PATH               # Show session PATH  
/custom/path

> exit                   # Exit rucli
$ echo $PATH             # System PATH unchanged
/usr/bin:/bin
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
- `>` - Redirect output (overwrite)
- `>>` - Redirect output (append)
- `<` - Input redirect from file
- `&` - Background execution

## Environment Commands

```bash
# List all variables (system + session)
> env
PATH=/usr/bin:/bin
HOME=/home/user
TEST_VAR=hello

# Show specific variable
> env HOME
/home/user

# Set session variable
> env CUSTOM_VAR=my_value
> env CUSTOM_VAR
my_value

# Override system variable safely
> env USER=custom_user
> env USER
custom_user
```

## Examples

```bash
# Environment variable management
> env DEBUG=true
> env LOG_LEVEL=info
> env                    # Shows all variables including new ones

# Safe system variable override  
> env PATH=/custom/bin:$PATH  # (Variable expansion coming in PR #55)
> echo Custom environment setup complete

# Background jobs with environment
> env WORKER_ID=1
> long_running_task &    # Will use WORKER_ID=1
[1] ThreadId(2)
```

## Project Structure

```
rucli/
├── src/
│   ├── main.rs         # Entry point
│   ├── commands.rs     # Command definitions & execution
│   ├── parser.rs       # Input parsing & command recognition
│   ├── handlers.rs     # Command implementations
│   ├── environment.rs  # Environment variable management
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
cargo test environment  # Test environment variables
cargo test env          # Test env command
cargo run -- --debug    # Run with debug logging
```

## Roadmap

**Phase 3: Advanced Features (46-65)** 🚀
- [x] Pipelines (46-48)
- [x] Redirections (49-51)
- [x] Background execution (52)
- [x] Job management (53)
- [x] Environment variables (54)
- [ ] Variable expansion ($VAR) (55)
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

**Next**: Variable expansion (`$VAR` syntax) in PR #55! 🔄