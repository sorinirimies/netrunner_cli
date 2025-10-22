# VHS documentation: https://github.com/charmbracelet/vhs

Output target/debug-mode.gif

Set Shell "bash"
Set FontSize 14
Set Width 1200
Set Height 600
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

Type "# NetRunner CLI - Debug Mode Demo"
Sleep 1s
Enter
Enter

Type "# Normal mode (clean output - only successful connections shown)"
Sleep 500ms
Enter

Type "../../target/release/netrunner_cli"
Sleep 1s
Enter

# Wait for geolocation (should be quick and clean)
Sleep 4s

Type "clear"
Enter
Enter

Type "# Debug mode (shows trace logs for failed services)"
Sleep 500ms
Enter

Type "NETRUNNER_DEBUG=1 ../../target/release/netrunner_cli"
Sleep 1s
Enter

# Wait for geolocation with trace logs
Sleep 5s

Type "# Notice the [TRACE] logs showing which services failed"
Sleep 2s
Enter
Sleep 2s

Type "# Useful for troubleshooting:"
Enter
Type "#  • API rate limits"
Enter
Type "#  • Network connectivity issues"
Enter
Type "#  • Service outages"
Enter
Sleep 3s
