rucli - Rust CLI Tool

🎯 100 Commit Challenge: Building a feature-rich CLI tool in 100 commits
Progress: 61/100 Commits 🎉

[■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□□]
Latest Changes (Commit #61)

    Added for loops for list iteration
    Runtime variable expansion with expand_variables method
    Loop variables are set/unset as environment variables
    Fixed all environment tests for new expansion behavior
    Single-line for statement support

Usage
For Loops (New!)

Basic for loop syntax:
bash

> for var in items; do command; done

Number iteration:
bash

> for i in 1 2 3; do echo Number: $i; done
Number: 1
Number: 2
Number: 3

String iteration:
bash

> for name in Alice Bob Charlie; do echo Hello, $name!; done
Hello, Alice!
Hello, Bob!
Hello, Charlie!

File processing:
bash

> for file in file1.txt file2.txt; do cat $file; done
# Contents of file1.txt
# Contents of file2.txt

With existing variables:
bash

> env PREFIX=test
> for num in 1 2 3; do echo $PREFIX-$num; done
test-1
test-2
test-3

Current limitations:

    ✅ Single-line for statements
    ✅ Variable expansion in loop body
    ✅ Loop variable cleanup after execution
    ❌ Multiple commands in loop body - not yet supported
    ❌ Wildcard expansion (*.txt) - not yet supported
    ❌ Command substitution in list - not yet supported
    ❌ Nested loops - not yet supported

Control Flow Features

Conditionals:
bash

if condition; then action; else alternative; fi

While loops:
bash

while condition; do action; done

For loops:
bash

for var in list; do action; done

Variable Expansion Changes

Starting from this commit, variables are expanded at runtime rather than parse time:
bash

> env X=1
> alias show="echo $X"
> env X=2
> show
2  # Uses current value, not value when alias was created

This enables proper support for loop variables and dynamic values.
Complete Feature Set

Control Flow:

    If-then-else conditionals
    While loops
    For loops (NEW!)
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
    for-in-do-done - List iteration

Examples
For Loop Scripts

batch_rename.rsh:
bash

#!/usr/bin/env rucli
# Rename files with prefix

env PREFIX=backup
for file in data1.txt data2.txt data3.txt; do
    mv $file ${PREFIX}_$file
    echo Renamed $file to ${PREFIX}_$file
done
echo All files renamed

process_list.rsh:
bash

#!/usr/bin/env rucli
# Process a list of items

for item in apple banana cherry; do
    echo Processing $item
    write ${item}.txt "Data for $item"
done
echo Created files for all items

directory_tour.rsh:
bash

#!/usr/bin/env rucli
# Visit each directory

mkdir -p project/src project/docs project/tests
for dir in src docs tests; do
    cd project/$dir
    echo Now in: $(pwd)
    cd ../..
done
echo Tour complete

Safety Features

Loop Variable Isolation:
bash

> env var=original
> for var in temp; do echo Inside: $var; done
Inside: temp
> echo Outside: $var
Outside: original  # Original value preserved

Environment Variable Safety:

    Uses unsafe blocks for env::set_var/remove_var (required in Rust 1.71+)
    Variables are properly cleaned up after loop execution
    No interference with system environment

Project Structure

rucli/
├── src/
│   ├── main.rs         # Entry point
│   ├── commands.rs     # Command definitions + expand_variables
│   ├── parser.rs       # Input parsing + for parser
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
    ├── loops.rsh        # For/while loop examples
    ├── conditionals.rsh # If-then-else examples
    └── scripts.rsh      # General scripts

Testing
bash

cargo test              # Run all tests
cargo test for_loop     # Test for loops
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

Next: Functions (function name() { commands; }) in Commit #62! 🔧
