#!/usr/bin/env rucli
# This script tests comment and empty line handling

echo First line

# This is a comment and should be ignored

echo Second line after empty line and comment

    # Indented comment
    echo Third line with indentation

# Multiple consecutive comments
# should all be ignored
# including this one

echo Final line