#!/usr/bin/env rucli
# Test script with multi-line structures

echo "Starting test..."

for i in 1 2 3
do
    echo "Number: $i"
done

if pwd
then
    echo "Current directory exists"
else
    echo "Error"
fi

function greet()
{
    echo "Hello from function"
}

greet
echo "Test complete!"