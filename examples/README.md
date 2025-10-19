# NetRunner CLI - Examples & Demos

This directory contains examples and demo recordings for NetRunner CLI.

## 📹 VHS Tapes (Terminal Recordings)

We use [VHS](https://github.com/charmbracelet/vhs) to create automated terminal recordings. These demonstrate various features and can be used to generate GIF/video demos.

### Available Demos

#### 1. **speed-test.tape** - Basic Speed Test
Demonstrates the core speed test functionality:
- Automatic IP-based geolocation
- Server discovery and selection
- Download and upload speed testing
- Real-time animated graphs
- Results display

**Run:** `vhs examples/vhs/speed-test.tape` (outputs to `target/speed-test.gif`)

#### 2. **debug-mode.tape** - Debug Mode
Shows the difference between normal and debug output:
- Clean output by default (silent failover)
- Debug mode with trace logs (`NETRUNNER_DEBUG=1`)
- Troubleshooting geolocation service failures

**Run:** `vhs examples/vhs/debug-mode.tape` (outputs to `target/debug-mode.gif`)

#### 3. **geolocation.tape** - Geolocation Features
Demonstrates the robust geolocation system:
- 5 geolocation services with sequential failover
- Data validation process
- ISP detection
- Smart fallback to Kansas City, USA

**Run:** `vhs examples/vhs/geolocation.tape` (outputs to `target/geolocation.gif`)

#### 4. **history.tape** - History Management
Shows historical test tracking:
- Building test history
- Viewing recent tests
- Displaying statistics
- 30-day retention

**Run:** `vhs examples/vhs/history.tape` (outputs to `target/history.gif`)

#### 5. **json-output.tape** - JSON Output
Demonstrates machine-readable output:
- JSON format for CI/CD
- Parsing with `jq`
- Saving results to file
- Integration examples

**Run:** `vhs examples/vhs/json-output.tape` (outputs to `target/json-output.gif`)

## 🎬 Generating Recordings

### Prerequisites

Install VHS:
```bash
# macOS
brew install vhs

# Linux (with Go installed)
go install github.com/charmbracelet/vhs@latest

# Or download from: https://github.com/charmbracelet/vhs/releases
```

### Generate All Recordings

```bash
# From project root
cd examples/vhs

# Generate individual recordings
vhs speed-test.tape      # Creates speed-test.gif
vhs debug-mode.tape      # Creates debug-mode.gif
vhs geolocation.tape     # Creates geolocation.gif
vhs history.tape         # Creates history.gif
vhs json-output.tape     # Creates json-output.gif

# Or generate all at once
for tape in *.tape; do
    echo "Recording $tape..."
    vhs "$tape"
done
```

### Output Files

Generated GIFs will be saved in the `target/` directory (gitignored):
- `target/speed-test.gif` - Basic speed test demo
- `target/debug-mode.gif` - Debug mode comparison
- `target/geolocation.gif` - Geolocation features
- `target/history.gif` - History management
- `target/json-output.gif` - JSON output examples

**Note:** The `target/` directory is gitignored, so generated GIFs won't be committed to version control.

## 🎨 Customizing Recordings

### VHS Configuration

Each `.tape` file can be customized:

```vhs
# Appearance
Set Theme "Dracula"          # Color scheme
Set FontSize 14              # Terminal font size
Set Width 1200               # Terminal width
Set Height 800               # Terminal height

# Behavior
Set PlaybackSpeed 0.5        # Slower = 0.5, Faster = 2.0
Set TypingSpeed 100ms        # Speed of typing animation

# Output
Output target/demo.gif           # Output file path (gitignored)
```

### Available Themes

- Dracula (default in examples)
- Monokai
- Nord
- Solarized Dark/Light
- And many more...

See: https://github.com/charmbracelet/vhs#themes

## 🚀 Code Examples

### Rust Examples

NetRunner CLI provides several comprehensive Rust examples demonstrating different features:

#### 1. **logo_demo.rs** - Logo Widget Demo
Simple example showing the NetRunner logo with cyberpunk aesthetics.

```bash
cargo run --example logo_demo
cargo run --example logo_demo small
cargo run --example logo_demo tiny
```

#### 2. **basic_speed_test.rs** - Basic Speed Test
Complete example of running a speed test programmatically.

```bash
cargo run --example basic_speed_test
```

Features:
- Running a speed test
- Accessing test results
- Displaying metrics
- Performance analysis
- Error handling

#### 3. **json_output.rs** - JSON Output Integration
Demonstrates JSON output for automation and integration.

```bash
cargo run --example json_output
```

Features:
- JSON serialization
- Saving results to files
- Parsing with external tools
- Conditional logic based on results
- CI/CD integration patterns

#### 4. **history_management.rs** - History Storage
Working with NetRunner's embedded database for test history.

```bash
cargo run --example history_management
```

Features:
- Storing test results
- Retrieving historical data
- Calculating statistics
- Analyzing trends
- Comparing current vs historical performance
- Database maintenance
- Exporting to JSON

#### 5. **custom_configuration.rs** - Custom Configurations
Different configuration profiles for various use cases.

```bash
cargo run --example custom_configuration
```

Configurations demonstrated:
- Quick Test (fast checks)
- Accurate Test (detailed analysis)
- CI/CD Pipeline (automation)
- Slow Connection (mobile/DSL)
- Gigabit Test (fiber/high-speed)

#### 6. **continuous_monitoring.rs** - Network Monitoring
Continuous network monitoring with alerts and logging.

```bash
cargo run --example continuous_monitoring
```

Features:
- Periodic speed testing
- Real-time monitoring
- Performance alerts
- Data logging to CSV
- Uptime tracking
- Statistics display
- Alert system

### Example Output Comparison

**basic_speed_test.rs:**
```
╔═══════════════════════════════════════════════════════════╗
║          NetRunner CLI - Basic Speed Test Example        ║
╚═══════════════════════════════════════════════════════════╝

📊 Speed Metrics:
   ↓ Download: 487.30 Mbps
   ↑ Upload:   125.80 Mbps
   📡 Ping:    8.20 ms
   ⚡ Quality:  Excellent
```

**json_output.rs:**
```json
{
  "timestamp": "2024-01-15T10:30:00Z",
  "download_mbps": 487.3,
  "upload_mbps": 125.8,
  "ping_ms": 8.2,
  "quality": "Excellent"
}
```

**continuous_monitoring.rs:**
```
📊 Test #1 - 2024-01-15 10:30:00
───────────────────────────────────────────────────────────
   ↓ Download: 487.30 Mbps
   ↑ Upload:   125.80 Mbps
   📡 Ping:    8.20 ms
   ⚡ Quality:  Excellent
   ✓ Test completed successfully
```

## 📝 Usage Examples

### Basic Speed Test

```bash
# Simple speed test
netrunner speed

# With JSON output
netrunner speed --json

# Without animations (faster)
netrunner speed --no-animation
```

### Debug Mode

```bash
# Show trace logs for failed geolocation services
NETRUNNER_DEBUG=1 netrunner speed

# Useful for troubleshooting:
# - API rate limits (429 errors)
# - Network timeouts
# - Service outages
```

### History Management

```bash
# View recent tests
netrunner history

# Show detailed statistics
netrunner history --stats

# View specific time range
netrunner history --days 7
```

### JSON Integration

```bash
# Save to file
netrunner speed --json > results.json

# Parse with jq
netrunner speed --json | jq '.download_mbps'

# Monitor continuously
watch -n 300 'netrunner speed --json | jq -r ".download_mbps"'

# CI/CD integration
netrunner speed --json | jq -e '.download_mbps > 100'
```

### Network Diagnostics

```bash
# Full diagnostics
netrunner diagnose

# Combined analysis
netrunner analyze
```

## 🔧 Advanced Examples

### Custom Server Testing

```bash
# Test against specific server
netrunner speed --server https://custom-server.example.com

# Useful for:
# - Internal network testing
# - CDN endpoint validation
# - Custom speed test servers
```

### Automated Monitoring

```bash
#!/bin/bash
# monitor.sh - Check speed every 5 minutes

while true; do
    RESULT=$(netrunner speed --json)
    DOWNLOAD=$(echo "$RESULT" | jq -r '.download_mbps')
    QUALITY=$(echo "$RESULT" | jq -r '.quality')
    
    echo "$(date): $DOWNLOAD Mbps - $QUALITY" >> speed_log.txt
    
    # Alert if speed drops below threshold
    if (( $(echo "$DOWNLOAD < 100" | bc -l) )); then
        echo "Warning: Speed below 100 Mbps!"
    fi
    
    sleep 300
done
```

### Integration with Prometheus

```bash
#!/bin/bash
# Export metrics for Prometheus

RESULT=$(netrunner speed --json)

echo "# HELP netrunner_download_mbps Download speed in Mbps"
echo "# TYPE netrunner_download_mbps gauge"
echo "netrunner_download_mbps $(echo "$RESULT" | jq -r '.download_mbps')"

echo "# HELP netrunner_upload_mbps Upload speed in Mbps"
echo "# TYPE netrunner_upload_mbps gauge"
echo "netrunner_upload_mbps $(echo "$RESULT" | jq -r '.upload_mbps')"

echo "# HELP netrunner_ping_ms Ping latency in milliseconds"
echo "# TYPE netrunner_ping_ms gauge"
echo "netrunner_ping_ms $(echo "$RESULT" | jq -r '.ping_ms')"
```

## 📚 Documentation

For more information, see:
- [Main README](../README.md) - Project overview
- [QUICK_REFERENCE.md](../QUICK_REFERENCE.md) - Quick reference guide
- [GEOLOCATION_SERVER_FIX.md](../GEOLOCATION_SERVER_FIX.md) - Technical details

## 🎯 Contributing Examples

Want to add a new example? Great!

1. Create a `.tape` file following the existing format
2. Test it with `vhs your-example.tape`
3. Add documentation to this README
4. Submit a pull request

## 🎓 Learning Path

Recommended order for exploring the examples:

1. **Start here:** `basic_speed_test.rs` - Learn the fundamentals
2. **Next:** `json_output.rs` - Understand data output formats
3. **Then:** `custom_configuration.rs` - Customize for your needs
4. **After:** `history_management.rs` - Track results over time
5. **Finally:** `continuous_monitoring.rs` - Build production monitors

## 🔧 Development Tips

### Using Examples as Templates

All examples are well-documented and can be used as templates:

```rust
// Copy an example
cp examples/basic_speed_test.rs examples/my_custom_test.rs

// Modify for your needs
// Add to Cargo.toml if needed

// Run your custom example
cargo run --example my_custom_test
```

### Integration Patterns

**As a Library:**
```rust
use netrunner_cli::modules::{
    speed_test::SpeedTest,
    types::TestConfig,
};

// Use in your own application
let config = TestConfig::default();
let test = SpeedTest::new(config)?;
let result = test.run_full_test().await?;
```

**In Scripts:**
```bash
#!/bin/bash
# Run speed test and check threshold
RESULT=$(netrunner speed --json)
SPEED=$(echo "$RESULT" | jq -r '.download_mbps')

if (( $(echo "$SPEED < 100" | bc -l) )); then
    echo "Speed below threshold!"
    exit 1
fi
```

## 📄 License

All examples are licensed under MIT, same as the main project.