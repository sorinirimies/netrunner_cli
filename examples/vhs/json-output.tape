# VHS documentation: https://github.com/charmbracelet/vhs

Output "examples/vhs/target/json-output.gif"

Set Shell "bash"
Set FontSize 14
Set Width 1200
Set Height 700
Set PlaybackSpeed 0.5
Set Theme "One Dark Pro"
Set TypingSpeed 80ms

Hide
Type "cargo build --release"
Enter
Sleep 3s
Type "clear"
Enter
Show

Type "# NetRunner CLI - JSON Output Demo"
Sleep 1s
Enter
Enter

Type "# JSON output is perfect for:"
Enter
Type "#  • CI/CD pipelines"
Enter
Type "#  • Automated monitoring"
Enter
Type "#  • Data analysis"
Enter
Type "#  • Integration with other tools"
Enter
Sleep 2s
Enter
Enter

Type "# Run speed test with JSON output"
Sleep 500ms
Enter

Type "../../target/release/netrunner_cli --json"
Sleep 1s
Enter

# Wait for test to complete
Sleep 30s

Type "# The output is valid JSON that can be piped or parsed"
Enter
Sleep 2s
Enter
Enter

Type "# Example: Save results to file"
Sleep 500ms
Enter

Type "../../target/release/netrunner_cli --json > results.json"
Sleep 1s
Enter
Sleep 30s

Type "cat results.json"
Enter
Sleep 3s
Enter
Enter

Type "# Example: Parse with jq"
Sleep 500ms
Enter

Type "cat results.json | jq '.download_mbps'"
Enter
Sleep 2s

Type "cat results.json | jq '.quality'"
Enter
Sleep 2s

Type "cat results.json | jq '.server_location'"
Enter
Sleep 3s
Enter
Enter

Type "# Example: Monitor speed over time"
Sleep 500ms
Enter

Type "while true; do"
Enter
Type "  ../../target/release/netrunner_cli --json | jq -r '.download_mbps'"
Enter
Type "  sleep 300  # Every 5 minutes"
Enter
Type "done"
Enter
Sleep 3s
Enter
Enter

Type "# Clean up"
Enter
Type "rm results.json"
Enter
Sleep 1s
