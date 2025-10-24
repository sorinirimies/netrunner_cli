# VHS documentation: https://github.com/charmbracelet/vhs

Output "examples/vhs/target/geolocation.gif"

Set Shell "bash"
Set FontSize 14
Set Width 1200
Set Height 800
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

Type "# NetRunner CLI - Geolocation Demo"
Sleep 1s
Enter
Enter

Type "# NetRunner uses 5 geolocation services for robust location detection"
Sleep 1s
Enter
Sleep 2s
Enter

Type "# Services (tried in order):"
Enter
Type "#  1. ipapi.co"
Enter
Type "#  2. ip-api.com"
Enter
Type "#  3. ipinfo.io"
Enter
Type "#  4. freegeoip.app"
Enter
Type "#  5. ipwhois.app"
Enter
Sleep 2s
Enter
Enter

Type "# If all fail, falls back to Kansas City, USA (geographic center)"
Sleep 1s
Enter
Sleep 2s
Enter
Enter

Type "# Let's see it in action:"
Sleep 500ms
Enter

Type "../../target/release/netrunner_cli"
Sleep 1s
Enter

# Wait for geolocation detection
Sleep 5s

Type "# Notice how it shows:"
Enter
Type "#  📍 Your detected location"
Enter
Type "#  🔌 Your ISP"
Enter
Type "#  ✓ Which service provided the data"
Enter
Sleep 3s
Enter
Enter

# Let's try with debug mode to see the process
Type "clear"
Enter
Enter

Type "# With debug mode, you can see the failover process:"
Sleep 500ms
Enter

Type "NETRUNNER_DEBUG=1 ../../target/release/netrunner_cli"
Sleep 1s
Enter

# Wait for geolocation with debug output
Sleep 6s

Type "# Debug mode shows:"
Enter
Type "#  • Which services were tried"
Enter
Type "#  • Why services failed (timeout, rate limit, etc.)"
Enter
Type "#  • Which service ultimately succeeded"
Enter
Sleep 3s
Enter
Enter

Type "# Each response is validated for:"
Enter
Type "#  ✓ Valid HTTP status (2xx)"
Enter
Type "#  ✓ No API errors"
Enter
Type "#  ✓ Non-empty country/city"
Enter
Type "#  ✓ Valid coordinates (-90 to 90 lat, -180 to 180 lon)"
Enter
Type "#  ✓ No zero coordinates (rejects 0,0)"
Enter
Sleep 4s
Enter
Enter

Type "# This ensures accurate location-based server selection!"
Sleep 2s
Enter
Sleep 2s
