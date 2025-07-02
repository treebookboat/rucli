# rucli - Rust CLI Tool

A simple command-line interface tool written in Rust.

## Features

### Implemented ✅
- [x] Basic REPL loop with prompt
- [x] Help command - Show available commands
- [x] Echo command - Display messages (multi-word support)
- [x] Repeat command - Repeat messages with count validation
- [x] Cat command - Display file contents
- [x] Exit/Quit command - Exit the program

### TODO 📝
- [ ] Write command - Write content to files
- [ ] Ls command - List directory contents
- [ ] Command aliases (e.g., `q` for `quit`)
- [ ] Command history
- [ ] Tab completion

## Usage

```bash
$ cargo run
Hello, rucli!
> help
help - show help message
echo - display message
cat - show texts in file
repeat <count> <message> - repeat message count times
exit - exit the program
quit - exit the program

> cat README.md
[File contents displayed here]

> cat src/
Error: 'src/' is a directory

> exit
good bye