# VHS documentation: https://github.com/charmbracelet/vhs

Output "examples/vhs/target/history.gif"

Set Shell "bash"
Set FontSize 14
Set Width 1200
Set Height 700
Set PlaybackSpeed 0.5
Set Theme "Dracula"
Set TypingSpeed 80ms

Type "# NetRunner CLI - History Feature Demo"
Sleep 1s
Enter
Enter

Type "# First, let's run a quick speed test"
Sleep 500ms
Enter

Type "cargo run --release"
Sleep 1s
Enter

# Wait for compilation
Sleep 5s

# Wait for speed test to complete
Sleep 30s

Type "clear"
Enter
Enter

Type "# Now let's view the test history"
Sleep 500ms
Enter

Type "cargo run --release -- --history"
Sleep 1s
Enter

# Wait for compilation
Sleep 3s

# Wait for history to display
Sleep 5s

Type "# The history shows:"
Enter
Type "#  ✓ Recent test results (30-day retention)"
Enter
Type "#  ✓ Download/Upload speeds"
Enter
Type "#  ✓ Ping latency and jitter"
Enter
Type "#  ✓ Connection quality ratings"
Enter
Type "#  ✓ Server locations"
Enter
Sleep 2s
Enter
Enter

Type "# History stored in: ~/.netrunner_cli/history.db"
Enter
Type "# Automatic cleanup of old records"
Enter
Sleep 3s
