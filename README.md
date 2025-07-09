# rucli - Rust CLI Tool

🎯 **100 PR Challenge**: Building a feature-rich CLI tool in 100 PRs

## Progress: 52/100 PRs 🎉

[■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□]

## Latest Changes (PR #52)

- Implemented background execution with `&` operator
- Commands run in separate threads without blocking the prompt
- Added job management system with job IDs
- Added `sleep` command for testing
- Format: `[job_id] ThreadId(thread_id)`

## Usage

```bash
$ cargo run
> sleep 5 &
[1] ThreadId(2)
> echo "Can run other commands!"
Can run other commands!

# Multiple background jobs
> find . *.txt &
[1] ThreadId(2)  
> grep pattern large_file.txt &
[2] ThreadId(3)
> cat data.txt | grep ERROR > errors.txt &
[3] ThreadId(4)
```

## Commands

**File Operations**: `cat`, `write`, `cp`, `mv`, `rm`  
**Directory Operations**: `ls`, `cd`, `pwd`, `mkdir`  
**Search Operations**: `find`, `grep`  
**Utilities**: `echo`, `repeat`, `sleep`, `alias`, `version`, `help`, `exit`

**Operators**:
- `|` - Pipe
- `>` - Redirect (overwrite)
- `>>` - Redirect (append)
- `<` - Input redirect
- `&` - Background execution

## Project Structure

```
rucli/
├── src/
│   ├── main.rs      # Entry point
│   ├── commands.rs  # Command definitions
│   ├── parser.rs    # Input parsing
│   ├── handlers.rs  # Command implementations
│   ├── pipeline.rs  # Pipeline logic
│   ├── redirect.rs  # Redirection handling
│   ├── job.rs       # Background job management
│   ├── alias.rs     # Alias system
│   └── error.rs     # Error types
└── tests/
    ├── cli_tests.rs
    └── integration_tests.rs
```

## Testing

```bash
cargo test              # Run all tests
cargo test background   # Test background execution
cargo run -- --debug    # Run with debug logging
```

## Roadmap

**Phase 3: Advanced Features (46-65)** 🚀
- [x] Pipelines (46-48)
- [x] Redirections (49-51)
- [x] Background execution (52)
- [ ] Job management (53)
- [ ] Environment variables (54-55)
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

**Next**: Job management (jobs, fg, bg) in PR #53! 🚀