# rucli - Rust CLI Tool

🎯 **100 PR Challenge**: Building a feature-rich CLI tool in 100 PRs

## Progress: 56/100 PRs 🎉

[■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□]

## Latest Changes (PR #56)

- Added command substitution with `$(command)` syntax
- Support for nested substitutions: `$(echo $(pwd))`
- Seamless integration with variable expansion
- Automatic trimming of trailing newlines from command output
- Robust error handling - failed commands expand to empty string
- Full test coverage for all substitution patterns

## Usage

```bash
$ cargo run
> echo Today is $(date)              # Today is Sat Jul 12 15:30:00 JST 2025
> echo Working in $(pwd)              # Working in /home/user/rucli
> mkdir $(echo myproject)-$(date +%Y%m%d)  # Creates myproject-20250712
> echo Hello $(echo $(echo World))   # Hello World (nested)
> cat $(ls | grep config | head -1)  # Reads first config file
```

## Command Substitution System

**Basic Syntax**:
```bash
> echo Current directory: $(pwd)
> echo Files: $(ls | wc -l) files found
> write output.txt $(cat input.txt | grep pattern)
```

**Nested Substitutions**:
```bash
> echo $(echo Nested: $(pwd))        # Evaluates inner $(pwd) first
> cp file.txt $(echo backup)-$(date +%s).txt  # Dynamic filenames
```

**With Variable Expansion**:
```bash
> env PROJECT=myapp
> echo Building in $(pwd) for $PROJECT version $(cat VERSION)
> mkdir -p $(echo $PROJECT)/$(date +%Y)/$(git branch --show-current)
```

**Error Handling**:
```bash
> echo Result: $(nonexistent_cmd)    # Result: (empty on error)
> echo Unclosed: $(echo hello        # Unclosed: $(echo hello (preserved)
> echo Empty: $()                    # Empty: (empty substitution)
```

## Complete Expansion System

**Processing Order**:
1. **Variable Expansion** (`$VAR`, `${VAR}`) - First pass
2. **Command Substitution** (`$(command)`) - Second pass
3. **Command Parsing** - Final pass

**Combined Examples**:
```bash
# Variables defined
> env USER=alice
> env PROJECT=myapp
> env BUILD_DIR=/tmp/builds

# Complex substitutions
> echo $USER working on $PROJECT in $(pwd)
alice working on myapp in /home/alice/myapp

> cd $(echo $BUILD_DIR)/$PROJECT-$(date +%Y%m%d)
> echo Switched to $(pwd)
Switched to /tmp/builds/myapp-20250712

# Nested with pipes
> echo Found $(ls $(echo $PROJECT)*.txt | wc -l) project files
Found 5 project files

# Dynamic file operations
> cp $(find . -name "*.conf" | head -1) $(echo $PROJECT).conf.backup
> write log.txt User $USER executed $(echo $0) at $(date)
```

## Environment Variable System

**Session Variables**: Variables set within rucli using `env VAR=value`
- Stored separately from system environment
- Persist throughout rucli session
- Don't affect system environment after exit
- Take precedence over system variables with same name

**Variable Expansion Priority**: Session Variables > System Variables

## Advanced Usage Examples

**Build Automation**:
```bash
> env VERSION=$(cat version.txt)
> env BUILD_ID=$(date +%Y%m%d-%H%M%S)
> mkdir -p builds/$(echo $VERSION)-$(echo $BUILD_ID)
> echo Build directory: $(pwd)/builds/$(ls builds | tail -1)
```

**Log Analysis**:
```bash
> env LOG_DATE=$(date +%Y-%m-%d)
> grep ERROR $(find /var/log -name "*$LOG_DATE*.log") > errors-$(date +%s).txt
> echo Found $(cat errors-*.txt | wc -l) errors today
```

**Dynamic Configuration**:
```bash
> env CONFIG_FILE=$(find . -name "*.conf" | grep $(hostname))
> cat $(echo $CONFIG_FILE) | grep -v "^#" > active.conf
> echo Loaded $(wc -l < active.conf) configuration lines
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

**Expansion Features**:
- `$VAR` - Basic variable expansion
- `${VAR}` - Brace notation for clear boundaries
- `$(command)` - Command substitution with full nesting support

## Project Structure

```
rucli/
├── src/
│   ├── main.rs         # Entry point
│   ├── commands.rs     # Command definitions & execution
│   ├── parser.rs       # Input parsing with expansions
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
cargo test environment  # Test variables & substitutions
cargo test command_sub  # Test command substitution
cargo test integration  # Test combined features
cargo run -- --debug    # Run with debug logging
```

## Roadmap

**Phase 3: Advanced Features (46-65)** 🚀
- [x