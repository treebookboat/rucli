# rucli - Rust CLI Tool

🎯 **100 PR Challenge**: Building a feature-rich CLI tool in 100 PRs

## Progress: 53/100 PRs 🎉

[■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□]

## Latest Changes (PR #53)

- Enhanced job management with comprehensive testing
- Improved job status tracking and cleanup verification
- Added thorough test coverage for background execution
- Verified `jobs` command marker functionality (+/-)
- Tested completed job auto-cleanup behavior
- Enhanced `fg` command with better error handling

## Usage

```bash
$ cargo run
> sleep 5 &
[1] ThreadId(2)
> echo "Can run other commands!"
Can run other commands!
> jobs
[1]+  Running    sleep 5
> fg 1
Job [1] (sleep 5) is still running
> jobs
No jobs   # Job completed and auto-cleaned
```

## Job Management

**Background Execution**: Add `&` to run commands in background
```bash
> find . -name "*.txt" &
[1] ThreadId(2)
> grep "error" log.txt &  
[2] ThreadId(3)
```

**Job Control**:
- `jobs` - List running background jobs
- `fg [job_id]` - Show job status (defaults to latest job)
- Completed jobs are automatically cleaned up

**Job Display Format**:
```
[1]+  Running    sleep 10    # Latest job (+ marker)
[2]-  Running    grep pattern # Previous job (- marker)  
[3]   Running    find /usr    # Older jobs
```

## Commands

**File Operations**: `cat`, `write`, `cp`, `mv`, `rm`  
**Directory Operations**: `ls`, `cd`, `pwd`, `mkdir`  
**Search Operations**: `find`, `grep`  
**Job Control**: `jobs`, `fg`  
**Utilities**: `echo`, `repeat`, `sleep`, `alias`, `version`, `help`, `exit`

**Operators**:
- `|` - Pipe commands together
- `>` - Redirect output (overwrite)
- `>>` - Redirect output (append)
- `<` - Input redirect from file
- `&` - Background execution

## Examples

```bash
# Pipeline with background execution
> cat large.txt | grep "ERROR" | wc -l &
[1] ThreadId(2)

# Multiple jobs
> sleep 10 &
[1] ThreadId(2)
> find /usr -name "*.conf" > configs.txt &
[2] ThreadId(3)
> jobs
[1]-  Running    sleep 10
[2]+  Running    find /usr -name *.conf > configs.txt

# Job status checking
> fg
Job [2] (find /usr -name *.conf > configs.txt) is still running
> fg 1  
Job [1] (sleep 10) is still running
```

## Project Structure

```
rucli/
├── src/
│   ├── main.rs      # Entry point
│   ├── commands.rs  # Command definitions & execution
│   ├── parser.rs    # Input parsing & command recognition
│   ├── handlers.rs  # Command implementations
│   ├── pipeline.rs  # Pipeline execution logic
│   ├── redirect.rs  # I/O redirection handling
│   ├── job.rs       # Background job management
│   ├── alias.rs     # Command alias system
│   └── error.rs     # Error types & handling
└── tests/
    ├── cli_tests.rs
    └── integration_tests.rs
```

## Testing

```bash
cargo test              # Run all tests
cargo test job          # Test job management
cargo test background   # Test background execution
cargo run -- --debug    # Run with debug logging
```

## Roadmap

**Phase 3: Advanced Features (46-65)** 🚀
- [x] Pipelines (46-48)
- [x] Redirections (49-51)
- [x] Background execution (52)
- [x] Job management (53)
- [ ] Environment variables (54-55)
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

**Next**: Environment variables (`env` command) in PR #54! 🌍