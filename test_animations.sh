#!/bin/bash

echo "Testing NetRunner CLI Animations..."
echo

echo "1. Testing Animation Showcase:"
echo "5" | ./target/release/netrunner_cli
echo

echo "2. Testing Speed Test with New Animations:"
./target/release/netrunner_cli --mode speed --no-animation=false
echo

echo "3. Testing Server Analysis:"
./target/release/netrunner_cli --mode servers
echo

echo "Animation tests complete!"