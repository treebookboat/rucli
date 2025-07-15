rucli - Rust CLI Tool

🎯 100 Commit Challenge: Building a feature-rich CLI tool in 100 commits
Progress: 60/100 Commits 🎉

[■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□]
Latest Changes (Commit #60)

    Added while loops for condition-based repetition
    Real-time command execution with immediate output
    Loop iteration limit (1000) to prevent infinite loops
    Single-line while statement support
    Clone trait added to Command for loop iteration

Usage
While Loops (New!)

Basic while loop syntax:
bash

> while condition; do command; done

File processing example:
bash

> write data.txt "Processing..."
> while cat data.txt; do echo "File exists"; rm data.txt; done
Processing...
File exists
# Loop ends when file is deleted

Counter simulation:
bash

> write counter.txt "3"
> while cat counter.txt; do echo "Count"; rm counter.txt; done
3
Count
# Ends after one iteration

With variables:
bash

> env FILE=test.txt
> write $FILE "content"
> while cat $FILE; do rm $FILE; done
content
# File removed, loop ends

Current limitations:

    ✅ Single-line while statements
    ✅ Real-time output during execution
    ✅ Infinite loop protection (1000 iterations max)
    ❌ Multiple commands in loop body - not yet supported
    ❌ Break/continue statements - not yet supported
    ❌ Nested loops - not yet supported

Control Flow Features

Conditionals:
bash

if condition; then action; else alternative; fi

Loops:
bash

while condition; do action; done

Interactive Mode
bash

$ cargo run
Hello, rucli!
> while echo "Loop"; do echo "Running"; done
Loop
Running
Loop
Running
... (continues until MAX_ITERATIONS)
> exit
good bye

Complete Feature Set

Control Flow:

    If-then-else conditionals
    While loops (NEW!)
    Background execution with &
    Pipeline chaining with |

File Operations: cat, write, cp, mv, rm
Directory Operations: ls, cd, pwd, mkdir
Search Operations: find, grep
Environment: env - manage environment variables
Job Control: jobs, fg - background job management
Utilities: echo, repeat, sleep, alias, version, help, exit

Operators:

    | - Pipe commands together
    > - Redirect output to file
    >> - Append output to file
    < - Input from file
    & - Background execution
    << - Here document
    <<- - Here document with tab stripping
    ; - Command separator
    if-then-fi - Conditional execution
    while-do-done - Loop execution

Examples
Loop Scripts

cleanup_loop.rsh:
bash

#!/usr/bin/env rucli
# Clean temporary files until none exist

echo "Starting cleanup..."
while find . *.tmp; do
    rm *.tmp
    echo "Removed temporary files"
done
echo "Cleanup complete"

monitor.rsh:
bash

#!/usr/bin/env rucli
# Monitor file existence

env TARGET=important.txt
while cat $TARGET; do
    echo "File still exists"
    sleep 2
done
echo "File has been removed!"

process_files.rsh:
bash

#!/usr/bin/env rucli
# Process files one by one

write file1.txt "Data 1"
write file2.txt "Data 2"
write file3.txt "Data 3"

while ls *.txt; do
    echo "Processing files..."
    # In real implementation, would process each file
    rm file1.txt
done
echo "All files processed"

Safety Features

Infinite Loop Protection:
bash

# This will stop after 1000 iterations
> while echo "infinite"; do echo "loop"; done

Maximum iterations: 1000 (prevents system hang)
Project Structure

rucli/
├── src/
│   ├── main.rs         # Entry point
│   ├── commands.rs     # Command definitions + while execution
│   ├── parser.rs       # Input parsing + while parser
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
    ├── loops.rsh        # While loop examples
    ├── conditionals.rsh # If-then-else examples
    └── scripts.rsh      # General scripts

Testing
bash

cargo test              # Run all tests
cargo test while        # Test while loops
cargo run -- test.rsh   # Run a test script

Roadmap

Phase 3: Advanced Features (46-65) 🚀

    Pipelines (46-48)
    Redirections (49-51)
    Background execution (52)
    Job management (53)
    Environment variables (54)
    Variable expansion (55)
    Command substitution (56)
    Here documents (57)
    Script file execution (58)
    If conditions (59)
    While loops (60)
    For loops (61)
    Functions (62)
    Error handling in scripts (63)
    Script debugging (64)
    Phase 3 integration tests (65)

Phase 4: Interactive Features (66-85)

    Command history & navigation
    Tab completion
    Syntax highlighting

Phase 5: Extensions (86-100)

    Plugin system
    Configuration files
    Performance optimization

Next: For loops (for var in items; do commands; done) in Commit #61! 🔁
