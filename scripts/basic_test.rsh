#!/usr/bin/env rucli
# Basic test script for rucli

echo Starting basic test...
pwd
echo Current directory shown above

# Create a test file
write test_output.txt Hello from script!
cat test_output.txt

# Clean up
rm test_output.txt
echo Test completed successfully