# NetRunner CLI

A feature-rich Rust-based CLI to test and analyze your internet connection with style.

[![Crates.io](https://img.shields.io/crates/v/netrunner_cli)](https://crates.io/crates/netrunner_cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/sorinirimies/netrunner_cli/actions/workflows/ci.yml/badge.svg)](https://github.com/sorinirimies/netrunner_cli/actions/workflows/ci.yml)

## ✨ Features

- 🚀 **Beautiful Animations** - Watch your tests progress with stylish visuals
- 📊 **Comprehensive Metrics** - Download, upload, ping, jitter, packet loss, and more
- 🔍 **Network Diagnostics** - Detailed analysis of your network configuration
- 📈 **Test History** - Track your connection over time with historical data
- 🌡️ **Quality Ratings** - Get a clear assessment of your connection quality
- 🎯 **Multi-Server Testing** - Test against multiple servers for accurate results
- 🎨 **Colorful Output** - Easy-to-read, color-coded results
- 💻 **Cross-Platform** - Works on Windows, macOS, and Linux

## 📥 Installation

### From Crates.io

```bash
cargo install netrunner_cli
```

### From Source

```bash
git clone https://github.com/sorinirimies/netrunner_cli.git
cd netrunner_cli
cargo install --path .
```

## 🚀 Usage

```bash
# Run an interactive menu
netrunner_cli

# Run a speed test with animations
netrunner_cli -m speed

# Run a network diagnostics test
netrunner_cli -m diag

# View your test history
netrunner_cli -m history

# Run a full network analysis
netrunner_cli -m full

# Output results in JSON format
netrunner_cli -j

# Run with detailed output
netrunner_cli -d detailed

# Get help with all options
netrunner_cli --help
```

## 🎮 Interactive Mode

NetRunner CLI features a beautiful interactive menu. Just run:

```bash
netrunner_cli
```

And select from options including:
- 🚀 Run Speed Test
- 🔍 Run Network Diagnostics
- 📈 View Test History
- 🌐 Full Network Analysis

## 📋 Command Reference

### Test Modes

- `speed` - Run a comprehensive internet speed test
- `diag` - Run network diagnostics to analyze your connection
- `history` - View and analyze your previous test results
- `full` - Run both speed test and diagnostics for complete analysis

### Options

- `-s, --server <URL>` - Set custom test server URL
- `-z, --size <MB>` - Set test file size in MB
- `-t, --timeout <SECONDS>` - Set timeout for each test
- `-j, --json` - Output results in JSON format
- `-n, --no-animation` - Disable animations
- `-d, --detail <LEVEL>` - Set detail level (basic, standard, detailed, debug)
- `-m, --mode <MODE>` - Set test mode (speed, diag, history, full)
- `-h, --help` - Display help information
- `-V, --version` - Display version information

## 📈 Understanding Your Results

NetRunner provides a comprehensive assessment of your internet connection:

| Metric | Excellent | Good | Average | Poor | Very Poor |
|--------|-----------|------|---------|------|-----------|
| Download | ≥100 Mbps | ≥50 Mbps | ≥25 Mbps | ≥10 Mbps | <10 Mbps |
| Upload | ≥20 Mbps | ≥10 Mbps | ≥5 Mbps | ≥2 Mbps | <2 Mbps |
| Ping | <20 ms | <50 ms | <100 ms | <150 ms | ≥150 ms |
| Jitter | <5 ms | <15 ms | <25 ms | <40 ms | ≥40 ms |
| Packet Loss | <0.1% | <1% | <2.5% | <5% | ≥5% |

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository at https://github.com/sorinirimies/netrunner_cli
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 📜 Changelog

See the [CHANGELOG.md](CHANGELOG.md) file for details on all changes.

> Note: The changelog is automatically generated on each release using git-cliff. All commit messages are categorized and included in the release notes.
