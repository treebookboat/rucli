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
- [x] Exit/Quit command - Exit the program

### TODO 📝
- [ ] Ls command - List directory contents
- [ ] Command aliases (e.g., `q` for `quit`)
- [ ] Command history (↑/↓ arrows)
- [ ] Tab completion
- [ ] Append mode for write command
- [ ] Mkdir command - Create directories

## Usage

```bash
$ cargo run
Hello, rucli!
> help
help - show help message
echo - display message
cat - show texts in file
repeat <count> <message> - repeat message count times
write <filename> <content> - write content to file
exit - exit the program
quit - exit the program

> write test.txt Hello, World!
File written successfully: test.txt

> cat test.txt
Hello, World!

> write test.txt Overwritten content
File written successfully: test.txt

> cat test.txt
Overwritten content