# Netrunner CLI 🚀

A high-performance, cyberpunk-styled network diagnostics and speed testing tool built in Rust.

[![Crates.io](https://img.shields.io/crates/v/netrunner_cli)](https://crates.io/crates/netrunner_cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Release](https://github.com/sorinirimies/netrunner_cli/actions/workflows/release.yml/badge.svg)](https://github.com/sorinirimies/netrunner_cli/actions/workflows/release.yml)

## Preview
![speed-test](https://github.com/user-attachments/assets/9e5695aa-116d-4abf-8fb1-e6a259ee1ea2)
<img width="831" height="1215" alt="image" src="https://github.com/user-attachments/assets/81972ad6-8a13-4c4b-be30-59c5e7a450b3" />
<img width="739" height="244" alt="image" src="https://github.com/user-attachments/assets/e5de4493-c0fa-4899-a612-fe839145ed4c" />

## ✨ Features

### 🎯 Core Capabilities

- **🚀 High-Speed Testing** - Optimized for gigabit+ connections (up to 10 Gbps)
  - 50 parallel connections for maximum throughput
  - Large 500MB chunk downloads to minimize overhead
  - 2-second warmup period for connection establishment
  - Progressive speed sampling with intelligent averaging
  
- **🌍 Smart Server Selection** - Robust geolocation-based discovery
  - Automatic IP-based location detection with 5 geolocation services
  - Sequential failover: ipapi.co → ip-api.com → ipinfo.io → freegeoip.app → ipwhois.app
  - Smart fallback to Kansas City, USA if all services fail
  - Fetches real servers from Speedtest.net API
  - Falls back to LibreSpeed and regional hubs
  - Haversine formula for accurate distance calculation
  - Quality score algorithm: latency + distance + geographic weight
  - Tests up to 15 servers concurrently, selects best 3
  - Clean output with silent failover (debug mode available)
  
- **📊 Comprehensive Metrics**
  - Download/Upload speeds (Mbps)
  - Ping latency (ms)
  - Jitter measurement
  - Packet loss detection
  - Connection quality assessment (Excellent → Poor)
  
- **📈 Live Animated Bandwidth Monitors**
  - Real-time animated graphs during speed tests
  - Dynamic updates every 200ms with smooth animation
  - Graph grows from left to right as test progresses
  - Shows live current speed and peak speed
  - Filled area visualization with Unicode blocks (8 lines × 80 chars)
  - Separate animated monitors for download (15s) and upload (12s)
  - Smooth in-place rendering using ANSI escape codes
  - Engaging, professional visual feedback
  
- **📈 Historical Tracking**
  - Automatic 30-day test history retention
  - Powered by sled embedded database
  - View trends and statistics over time
  - Query past results with `--history` flag
  - Automatic cleanup of old records
  
- **🔍 Network Diagnostics**
  - Public IP detection
  - ISP information
  - DNS server analysis
  - Network interface details
  - Traceroute functionality
  - Connection quality scoring
  
- **🎨 Stunning Visuals**
  - Animated intro with color-cycling borders
  - Cyberpunk-themed UI with neon colors
  - Smooth progress bars and gauges
  - Real-time speed updates during tests
  - Beautiful ASCII art logo

### 💻 Technical Excellence

- **Zero-Copy Performance** - Optimized for minimal overhead
- **Async/Await** - Powered by Tokio for efficient I/O
- **Cross-Platform** - Works on Linux, macOS, and Windows
- **Embedded Database** - No external dependencies for history storage
- **Clean Architecture** - Well-structured, maintainable codebase
- **Comprehensive Tests** - 90+ unit and integration tests
- **Robust Error Handling** - Graceful degradation with multiple fallbacks
- **Data Validation** - Comprehensive validation for all geolocation responses

## 📥 Installation

### From Crates.io (Recommended)

```bash
cargo install netrunner_cli
```

### From Source

```bash
git clone https://github.com/sorinirimies/netrunner_cli.git
cd netrunner_cli
cargo install --path .
```

### Binary Releases

Download pre-built binaries from the [Releases](https://github.com/sorinirimies/netrunner_cli/releases) page.

## 🚀 Quick Start

### Run with Animated Intro

```bash
netrunner_cli
```

Experience the full cyberpunk intro with animated borders and color-cycling effects!

### Run Speed Test

```bash
# Full speed test with automatic server selection and live bandwidth monitor
netrunner_cli speed

# Speed test with JSON output (no bandwidth monitor)
netrunner_cli speed --json

# Speed test without animations (headless mode, no bandwidth monitor)
netrunner_cli speed --no-animation
```

### Live Animated Bandwidth Monitors

During speed tests, Netrunner displays **live animated bandwidth monitors** that update in real-time:

**Download Speed Monitor (15 seconds):**
```
⟨⟨⟨ DOWNLOAD SPEED BANDWIDTH MONITOR ⟩⟩⟩

104.2 Mbps

Peak: 120.0 Mbps

│████████████████████████████████████████████████████████████████████████████
│████████████████████████████████████████████████████████████████████████████
│████████████████████████████████████████████████████████████████████████████
│██████████████████████████████████████████████████████████████████████████▌         
│█████████████████████████████████████████████████████████████▌              
│██████████████████████████████████████████████████▌                         
│████████████████████████████▌                                               
│████▌                                                                       
└────────────────────────────────────────────────────────────────────────────
```

**Upload Speed Monitor (12 seconds):**
```
⟨⟨⟨ UPLOAD SPEED BANDWIDTH MONITOR ⟩⟩⟩

45.8 Mbps

Peak: 52.3 Mbps

│████████████████████████████████████████████████████████████████████████████
│████████████████████████████████████████████████████████████████████████████
│██████████████████████████████████████████████████████████▌                 
│████████████████████████████████████████████████▌                           
│██████████████████████████████████████▌                                     
│████████████████████████████▌                                               
│████████████████▌                                                           
│████▌                                                                       
└────────────────────────────────────────────────────────────────────────────
```

**Animation Features:**
- Appears automatically at test start with empty graph
- Updates dynamically every 200ms throughout the test
- Graph fills from left to right showing speed progression
- Current speed and peak speed update in real-time
- Smooth animation using ANSI escape codes (no flicker)
- Single chart per test (no duplicates or stacking)
- Only appears when animations enabled (hidden in JSON/headless mode)

### View Test History

```bash
# View last 30 days of test results
netrunner_cli --history

# Or use the shorthand
netrunner_cli -H
```

### Network Diagnostics

```bash
# Run full network diagnostics
netrunner_cli diag

# Get detailed diagnostics output
netrunner_cli diag --detail detailed
```

### Full Network Analysis

```bash
# Run both speed test and diagnostics
netrunner_cli full
```

## 📋 Command Reference

### Commands

```bash
netrunner_cli [OPTIONS] [COMMAND]
```

#### Available Commands

- `speed` - Run a comprehensive internet speed test
- `diag` - Run network diagnostics to analyze your connection
- `full` - Run both speed test and diagnostics
- `help` - Display help information

### Options

| Flag | Long Form | Description |
|------|-----------|-------------|
| `-H` | `--history` | View test history (last 30 days) |
| `-s <URL>` | `--server <URL>` | Custom test server URL |
| `-z <MB>` | `--size <MB>` | Test file size in MB (default: 100) |
| `-t <SEC>` | `--timeout <SEC>` | Timeout in seconds (default: 30) |
| `-j` | `--json` | Output results in JSON format |
| `-n` | `--no-animation` | Disable animations (headless mode) |
| `-d <LEVEL>` | `--detail <LEVEL>` | Detail level: basic, standard, detailed, debug |
| `-m <COUNT>` | `--max-servers <N>` | Maximum servers to test (default: 3) |
| `-h` | `--help` | Display help information |
| `-V` | `--version` | Display version information |

### Examples

```bash
# Run speed test with custom server
netrunner_cli speed --server https://speed.cloudflare.com

# Run diagnostics with debug output
netrunner_cli diag --detail debug

# Get JSON output for scripting
netrunner_cli speed --json > results.json

# Run headless mode (CI/CD)
netrunner_cli speed --no-animation --json

# Test against 5 servers
netrunner_cli speed --max-servers 5

# View your connection history
netrunner_cli --history
```

## 📊 Understanding Your Results

### Connection Quality Ratings

Netrunner provides a comprehensive assessment based on multiple metrics:

| Quality | Download | Upload | Ping | Jitter | Packet Loss |
|---------|----------|--------|------|--------|-------------|
| ⭐⭐⭐⭐⭐ **Excellent** | ≥100 Mbps | ≥20 Mbps | <20 ms | <5 ms | <0.1% |
| ⭐⭐⭐⭐ **Good** | ≥50 Mbps | ≥10 Mbps | <50 ms | <15 ms | <1% |
| ⭐⭐⭐ **Average** | ≥25 Mbps | ≥5 Mbps | <100 ms | <25 ms | <2.5% |
| ⭐⭐ **Poor** | ≥10 Mbps | ≥2 Mbps | <150 ms | <40 ms | <5% |
| ⭐ **Very Poor** | <10 Mbps | <2 Mbps | ≥150 ms | ≥40 ms | ≥5% |

### Metric Explanations

- **Download Speed**: How fast you can receive data (streaming, downloading)
- **Upload Speed**: How fast you can send data (video calls, cloud backups)
- **Ping (Latency)**: Time for data to reach the server and back (gaming, real-time apps)
- **Jitter**: Variation in ping times (video calls, online gaming stability)
- **Packet Loss**: Percentage of data packets that don't arrive (connection reliability)

## 🏗️ Architecture

### High-Speed Testing Strategy

Netrunner is optimized for modern high-speed connections:

1. **Parallel Connections**: Uses 50 simultaneous connections to maximize throughput
2. **Large Chunks**: Downloads 500MB chunks to minimize protocol overhead
3. **Warmup Period**: 2-second warmup establishes connections before measurement
4. **Progressive Sampling**: Continuously samples speed and averages for accuracy
5. **Smart Exclusion**: Excludes warmup period from final calculations

### Server Selection Algorithm

1. **Geolocation**: Detects your location via IP using 5 services with sequential failover
   - Primary: ipapi.co (HTTP/HTTPS)
   - Secondary: ip-api.com (with status validation)
   - Tertiary: ipinfo.io (loc field parsing)
   - Quaternary: freegeoip.app
   - Final: ipwhois.app (with success field check)
   - Fallback: Kansas City, USA (39.0997°N, 94.5786°W)

2. **Data Validation**: Every response is validated for:
   - Valid HTTP status (2xx)
   - No API error messages
   - Non-empty country/city names
   - Valid coordinates (lat: -90 to 90, lon: -180 to 180)
   - No zero coordinates (rejects 0,0 as invalid)

3. **Server Discovery**: Dynamic multi-source approach
   - Speedtest.net API servers (up to 10 nearby)
   - Continent-based CDN servers
   - Country-specific servers
   - Global CDN fallbacks (Cloudflare, Google)

4. **Distance Calculation**: Haversine formula for accurate geographic distance
   ```
   distance = 2r × arcsin(√(sin²(Δlat/2) + cos(lat1) × cos(lat2) × sin²(Δlon/2)))
   where r = 6371 km (Earth's radius)
   ```

5. **Quality Scoring**: Intelligent server ranking
   ```
   quality_score = (10000 × geographic_weight) / (latency_penalty + distance_penalty)
   
   where:
     latency_penalty = max(latency_ms, 1.0)
     distance_penalty = max(distance_km / 100.0, 1.0)
     geographic_weight = 0.3 to 1.0 (regional > continental > global)
   ```

6. **Server Testing**: Concurrent performance evaluation
   - Tests up to 15 servers in parallel
   - Measures actual latency for each
   - Sorts by quality score (higher = better)
   - Selects top 3 servers for speed testing

7. **Output**: Clean, professional display
   - Shows only successful geolocation by default
   - Silent failover to next service on errors
   - Debug mode available: `NETRUNNER_DEBUG=1`

### History Storage

- **Database**: Embedded sled database (no external dependencies)
- **Retention**: Automatic 30-day retention with daily cleanup
- **Location**: `~/.netrunner_cli/history.db`
- **Format**: Efficient binary storage with MessagePack serialization
- **Queries**: Fast indexed lookups by timestamp

## 🎨 Visual Features

### Animated Intro

Experience the cyberpunk aesthetic with:
- **Color-cycling borders** around the logo
- **Smooth fade-in effects**
- **6-color palette** (cyan, magenta, yellow, neon green)
- **Box drawing characters** for retro terminal vibes
- **60 FPS animation** for smooth visuals

### Live Animated Bandwidth Monitors

Real-time animated bandwidth visualization:
- **Dynamic Animation** - Graph grows and updates live during test execution
- **Download Speed Monitor** - 15-second test with real-time animated graph
- **Upload Speed Monitor** - 12-second test with real-time animated graph
- **200ms Update Rate** - Smooth, responsive animation
- **Instant Feedback** - See current and peak speeds update live
- **Growing Visualization** - Chart fills left-to-right as test progresses
- **Professional Appearance** - Filled area chart with Unicode blocks
- **ANSI-Based Rendering** - Smooth in-place updates without flicker
- **Single Chart Per Test** - Clean, uncluttered output
- **Automatic Scaling** - Graph adjusts to measured speed range

### Progress Indicators

- **Real-time speed updates** during downloads/uploads
- **Animated progress bars** with percentage
- **Color-coded quality indicators**
- **Gauge widgets** for visual metric display

## 🔧 Configuration

### Environment Variables

```bash
# Set custom history database path
export NETRUNNER_HISTORY_PATH="~/custom/path/history.db"

# Disable colors (for CI/CD)
export NO_COLOR=1

# Enable debug mode (show geolocation service failures)
export NETRUNNER_DEBUG=1
```

**Debug Mode**: When `NETRUNNER_DEBUG=1` is set, the application shows trace logs for failed geolocation services. This is useful for troubleshooting network issues or API rate limits.

**Normal Output:**
```
🌍 Detecting your location...
📍 Location: Berlin, Germany (via ipinfo.io)
🔌 ISP: Deutsche Telekom
```

**Debug Output:**
```
🌍 Detecting your location...
[TRACE] ipapi.co geolocation failed: HTTP error: 429 Too Many Requests
[TRACE] ip-api.com geolocation failed: timeout
📍 Location: Berlin, Germany (via ipinfo.io)
🔌 ISP: Deutsche Telekom
```

### History Database

Location: `~/.netrunner_cli/history.db`

You can manually inspect or backup this database using sled tools.

## 📚 Examples

### Basic Speed Test

```bash
# Run a simple speed test
netrunner speed

# Output:
# 🌍 Detecting your location...
# 📍 Location: San Francisco, United States (via ipapi.co)
# 🔌 ISP: Comcast Cable Communications
#
# 🔍 Building server pool...
# ✓ 12 servers in pool
#
# ⚡ Testing server performance...
# ✓ 3 servers selected for testing
#   1. US West Coast Hub - 8.2 ms (15 km)
#   2. LibreSpeed Los Angeles - 12.5 ms (560 km)
#   3. US Central - 28.3 ms (2800 km)
#
# [Download/Upload speed tests with animated graphs]
#
# ✓ Download: 487.3 Mbps
# ✓ Upload: 125.8 Mbps
# ✓ Ping: 8.2 ms
# ✓ Quality: Excellent
```

### Speed Test with JSON Output

```bash
# Get machine-readable results
netrunner speed --json

# Output:
# {
#   "timestamp": "2024-01-15T10:30:00Z",
#   "download_mbps": 487.3,
#   "upload_mbps": 125.8,
#   "ping_ms": 8.2,
#   "jitter_ms": 1.5,
#   "packet_loss_percent": 0.0,
#   "server_location": "San Francisco, USA",
#   "quality": "Excellent",
#   "isp": "Comcast Cable Communications"
# }
```

### Debug Mode for Troubleshooting

```bash
# Enable debug mode to see geolocation service failures
NETRUNNER_DEBUG=1 netrunner speed

# Output shows trace logs:
# 🌍 Detecting your location...
# [TRACE] ipapi.co geolocation failed: HTTP error: 429 Too Many Requests
# [TRACE] ip-api.com geolocation failed: connection timeout
# 📍 Location: Berlin, Germany (via ipinfo.io)
```

### View Historical Results

```bash
# Show test history
netrunner history

# Output:
# ╔═══════════════════════════════════════════════════════════════╗
# ║                    Speed Test History                         ║
# ╚═══════════════════════════════════════════════════════════════╝
#
# Recent Tests (Last 7 Days):
# ┌────────────────────┬──────────┬──────────┬──────────┬──────────┐
# │ Date               │ Download │ Upload   │ Ping     │ Quality  │
# ├────────────────────┼──────────┼──────────┼──────────┼──────────┤
# │ 2024-01-15 10:30   │ 487.3    │ 125.8    │ 8.2 ms   │ Excellent│
# │ 2024-01-14 15:20   │ 492.1    │ 127.3    │ 7.9 ms   │ Excellent│
# │ 2024-01-13 09:45   │ 478.5    │ 123.1    │ 9.1 ms   │ Excellent│
# └────────────────────┴──────────┴──────────┴──────────┴──────────┘
```

### Network Diagnostics

```bash
# Run comprehensive network diagnostics
netrunner diagnose

# Output includes:
# - Public IP address
# - ISP information
# - DNS servers
# - Network interfaces
# - Traceroute to common destinations
# - Connection quality assessment
```

### Full Network Analysis

```bash
# Combine speed test with diagnostics
netrunner analyze

# Performs:
# 1. Network diagnostics
# 2. Speed test
# 3. Historical comparison
# 4. Quality assessment with recommendations
```

### Custom Server URL

```bash
# Test against specific server
netrunner speed --server https://custom-server.example.com

# Useful for testing:
# - Internal network speeds
# - Specific CDN endpoints
# - Custom speed test servers
```

### Disable Animations

```bash
# Run without animations (faster, CI/CD friendly)
netrunner speed --no-animation

# Or use environment variable
NO_COLOR=1 netrunner speed
```

### Compare Current vs Historical Average

```bash
# View statistics
netrunner history --stats

# Output:
# Statistics (Last 30 Days):
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# Average Download: 485.2 Mbps
# Average Upload:   125.1 Mbps
# Average Ping:     8.5 ms
# Tests Performed:  47
# Best Quality:     Excellent (89% of tests)
```

## 🧪 Development

### Building from Source

```bash
# Clone the repository
git clone https://github.com/sorinirimies/netrunner_cli.git
cd netrunner_cli

# Build in debug mode
cargo build

# Build in release mode (optimized)
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test module
cargo test speed_test

# Run with output
cargo test -- --nocapture

# Run integration tests
cargo test --test '*'
```

### Project Structure

```
netrunner_cli/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library exports
│   └── modules/
│       ├── speed_test.rs    # Speed testing implementation
│       ├── history.rs       # History storage with sled
│       ├── diagnostics.rs   # Network diagnostics
│       ├── intro.rs         # Animated intro screen
│       ├── logo.rs          # ASCII logo rendering
│       ├── ui.rs            # UI components and gauges
│       └── types.rs         # Shared types and traits
├── tests/                   # Integration tests
├── examples/                # Example usage
└── Cargo.toml              # Project dependencies
```

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'Add amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

### Development Guidelines

- Write tests for new features
- Follow Rust idioms and best practices
- Update documentation for API changes
- Run `cargo fmt` and `cargo clippy` before committing
- Ensure all tests pass with `cargo test`

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **[Ratatui](https://github.com/ratatui-org/ratatui)** - Terminal UI framework
- **[Tokio](https://tokio.rs/)** - Async runtime
- **[Sled](https://github.com/spacejam/sled)** - Embedded database
- **[Reqwest](https://github.com/seanmonstar/reqwest)** - HTTP client
- **[Colored](https://github.com/mackwic/colored)** - Terminal colors

## 📜 Changelog

See [CHANGELOG.md](CHANGELOG.md) for detailed version history.

## 🐛 Bug Reports

Found a bug? Please [open an issue](https://github.com/sorinirimies/netrunner_cli/issues) with:
- Your OS and terminal emulator
- Steps to reproduce
- Expected vs actual behavior
- Relevant logs (run with `RUST_LOG=debug`)

## 💡 Feature Requests

Have an idea? [Open an issue](https://github.com/sorinirimies/netrunner_cli/issues) with the `enhancement` label!

## 📊 Project Stats

- **Language**: Rust 🦀
- **Tests**: 45+ passing
- **Dependencies**: Carefully curated, security-focused
- **Performance**: Optimized for gigabit+ speeds
- **Maintenance**: Actively maintained

---

**Built with ❤️ and Rust by [Sorin Irimies](https://github.com/sorinirimies)**

*Jack in, trace the net, and measure your connection with style.* 🚀
