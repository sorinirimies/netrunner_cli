# VHS documentation: https://github.com/charmbracelet/vhs

Output "examples/vhs/target/speed-test.gif"

Set Shell "bash"
Set FontSize 14
Set Width 1200
Set Height 800
Set PlaybackSpeed 0.5
Set Theme "One Dark Pro"
Set TypingSpeed 100ms

Hide
Type "cargo build --release"
Enter
Sleep 3s
Type "clear"
Enter
Show

Type "# NetRunner CLI - Speed Test Demo"
Sleep 1s
Enter
Enter

Type "# Run a speed test with automatic server selection"
Sleep 500ms
Enter

Type "../../target/release/netrunner_cli"
Sleep 1s
Enter

# Wait for geolocation
Sleep 3s

# Wait for server discovery
Sleep 2s

# Wait for server selection
Sleep 2s

# Wait for download test (15 seconds)
Sleep 16s

# Wait for upload test (12 seconds)
Sleep 13s

# Show final results
Sleep 5s

Type "# Speed test complete!"
Sleep 2s
Enter
Enter

Type "# The test automatically:"
Enter
Type "#  ✓ Detected location via IP"
Enter
Type "#  ✓ Found nearby servers"
Enter
Type "#  ✓ Tested 15 servers concurrently"
Enter
Type "#  ✓ Selected best 3 servers"
Enter
Type "#  ✓ Measured download & upload speeds"
Enter
Sleep 3s
