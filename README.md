# rucli - Rust CLI Tool

A simple command-line interface tool written in Rust.

## Features

### Implemented ✅
- [x] Basic REPL loop with prompt
- [x] Help command - Show available commands
- [x] Echo command - Display messages (multi-word support)
- [x] Repeat command - Repeat messages with count validation
- [x] Cat command - Display file contents
- [x] Write command - Write content to files
- [x] Ls command - List directory contents
- [x] Exit/Quit command - Exit the program

### TODO 📝
- [ ] Command aliases (e.g., `q` for `quit`)
- [ ] Command history (↑/↓ arrows)
- [ ] Tab completion
- [ ] Append mode for write command
- [ ] Mkdir command - Create directories
- [ ] Rm command - Remove files
- [ ] Cd command - Change directory
- [ ] Pwd command - Print working directory

## Usage Examples

```bash
$ cargo run
Hello, rucli!

> help
help - show help message
echo - display message
cat - show texts in file
repeat <count> <message> - repeat message count times
write <filename> <content> - write content to file
ls - list directory contents
exit - exit the program
quit - exit the program

> ls
Cargo.toml
Cargo.lock
README.md
src
target
test.txt

> write hello.txt Hello, World!
File written successfully: hello.txt

> cat hello.txt
Hello, World!

> exit
good bye