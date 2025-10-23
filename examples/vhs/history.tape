# VHS documentation: https://github.com/charmbracelet/vhs

Output target/history.gif

Set Shell "bash"
Set FontSize 14
Set Width 1200
Set Height 700
Set PlaybackSpeed 0.5
Set Theme "Dracula"
Set TypingSpeed 80ms

Hide
Type "cargo build --release"
Enter
Sleep 3s
Type "clear"
Enter
Show

Type "# NetRunner CLI - History Feature Demo"
Sleep 1s
Enter
Enter

Type "# First, let's run a few speed tests to build history"
Sleep 500ms
Enter

Type "../../target/release/netrunner_cli --json > /dev/null"
Sleep 1s
Enter
Sleep 25s

Type "../../target/release/netrunner_cli --json > /dev/null"
Enter
Sleep 25s

Type "clear"
Enter
Enter

Type "# Now let's view the test history"
Sleep 500ms
Enter

Type "../../target/release/netrunner_cli --history"
Sleep 1s
Enter

# Wait for history to display
Sleep 5s

Type "# The history shows:"
Enter
Type "#  ✓ Recent test results not older than 30 days"
Enter
Type "#  ✓ Download/Upload speeds"
Enter
Type "#  ✓ Ping latency"
Enter
Type "#  ✓ Connection quality"
Enter
Type "#  ✓ Test timestamps"
Enter
Sleep 2s
Enter
Enter



Type "# History is stored in: ~/.netrunner_cli/history.db"
Enter
Type "# Automatic 30-day retention with daily cleanup"
Enter
Sleep 3s
