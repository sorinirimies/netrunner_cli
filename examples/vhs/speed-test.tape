# VHS documentation: https://github.com/charmbracelet/vhs

Output "examples/vhs/target/speed-test.gif"

Set Shell "bash"
Set FontSize 14
Set Width 1200
Set Height 800
Set PlaybackSpeed 0.5
Set Theme "Dracula"
Set TypingSpeed 100ms

Type "# NetRunner CLI - Speed Test Demo"
Sleep 1s
Enter
Enter

Type "# Run a speed test with automatic server selection"
Sleep 500ms
Enter

Type "cargo run --release"
Sleep 1s
Enter

# Wait for compilation
Sleep 5s

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
Type "#  ✓ Tested servers for best performance"
Enter
Type "#  ✓ Selected optimal servers"
Enter
Type "#  ✓ Measured download & upload speeds"
Enter
Sleep 3s
