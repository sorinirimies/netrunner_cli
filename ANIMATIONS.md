# NetRunner CLI - Enhanced Animations & Server Selection

## 🎮 Animation System Overview

NetRunner CLI now features a complete cyberpunk-themed animation system that replaces traditional progress bars with engaging, dynamic spinners and visual effects.

## 🎯 New Animation Types

### Core Spinners

#### 1. Cyberpunk Scanner
```
⟨⟨⟨ ▰▰▰▰▰▰▰ SCANNING NETWORK NODES ⟩⟩⟩
```
- **Use**: Primary scanning operations
- **Pattern**: Flowing blocks with cyberpunk brackets
- **Speed**: 150ms intervals for dramatic effect

#### 2. Pacman Data Hunter
```
🎮 ᗤ    ••••••• CONSUMING DATA PACKETS
```
- **Use**: Data processing operations
- **Pattern**: Pacman eating dots across the line
- **Speed**: 150ms intervals for retro gaming feel

#### 3. Download Arrows
```
📥 ⬇️ ⬇️ ⬇️ ⬇️ DOWNLOADING DATA STREAMS
```
- **Use**: Download speed testing
- **Pattern**: Cascading arrows with completion indicator
- **Speed**: 200ms intervals with final "💾 Data Captured"

#### 4. Upload Rockets
```
📤 ⬆️ ⬆️ ⬆️ ⬆️ TRANSMITTING DATA PACKETS
```
- **Use**: Upload speed testing
- **Pattern**: Rising arrows with rocket completion
- **Speed**: 180ms intervals with final "🚀 Sent Complete"

#### 5. Ping Pong Animation
```
🏓 🏓    📍 MEASURING NEURAL RESPONSE
```
- **Use**: Latency testing
- **Pattern**: Ball bouncing between endpoints
- **Speed**: 100ms intervals for rapid response feel

### Special Effects Spinners

#### 6. DNA Helix
```
🧬 ╱   ╲ ANALYZING DNS INFRASTRUCTURE
```
- **Use**: DNS analysis operations
- **Pattern**: Rotating double helix structure
- **Speed**: 200ms intervals for scientific feel

#### 7. Rocket Boost
```
🚀 🚀   🌟 TESTING DNS QUANTUM RESPONSE
```
- **Use**: High-speed operations
- **Pattern**: Rocket leaving star trail
- **Speed**: 120ms intervals for dynamic motion

#### 8. Wave Frequency
```
🌊 ▁▂▃▄▅▆▇██▇▆▅▄▃▂▁ SCANNING FREQUENCIES
```
- **Use**: Signal analysis
- **Pattern**: Audio wave visualization
- **Speed**: 100ms intervals for smooth wave motion

#### 9. Speed Test Flow
```
🌐 ▓▓▓▓▓▓▓▓▓▓ DATA FLOW ANALYSIS
```
- **Use**: General speed testing
- **Pattern**: Flowing gradient blocks
- **Speed**: 120ms intervals

## 🌟 Enhanced Visual Effects

### Matrix Rain Effects
```
█0⠈⠄⠂⡀⠠⠠⠈1⠈⠄⡀⠠⡀⠐⠁⠁1⠠0⠂⠁⠂⠠⠠⠄⠄⠐⢀0
█⠐⠠010⡀⢀⡀0⢀⠐⢀⠁⢀⠠⠄⠁⢀⠄⢀⠐⠁⢀⠠⠄⠁⠁⠐⠁⠐
█⠄1⠠⠈⠁⢀⠠⠈1⡀⠁0⢀⠄1⢀⠂1⠂⠄⠠⠄⠁⠂⠈⠂⠂1⠂⠐
```
- **Use**: Background atmosphere during operations
- **Effect**: Cascading green digital rain
- **Duration**: 2-3 lines for subtle enhancement

### Typing Effects
```
⟨⟨⟨ NEURAL INTERFACE ACTIVATED ⟩⟩⟩
```
- **Use**: System status messages
- **Effect**: Character-by-character typing
- **Speed**: 50ms per character

### Pulse Animations
```
⟨⟨⟨ SCANNING QUANTUM NETWORKS ⟩⟩⟩ (bright cyan)
⟨⟨⟨ SCANNING QUANTUM NETWORKS ⟩⟩⟩ (blue)
```
- **Use**: Important status updates
- **Effect**: Color pulsing between bright and dim
- **Cycles**: 2-3 pulses per message

### Connection Establishment Sequence
```
⟨⟨⟨ INITIALIZING NEURAL INTERFACE ⟩⟩⟩
⟨⟨⟨ SCANNING NETWORK TOPOLOGY ⟩⟩⟩
⟨⟨⟨ ESTABLISHING QUANTUM TUNNEL ⟩⟩⟩
⟨⟨⟨ CALIBRATING DATA STREAMS ⟩⟩⟩
⟨⟨⟨ CONNECTION ESTABLISHED ⟩⟩⟩
```
- **Use**: System initialization
- **Effect**: Sequential status messages with delays
- **Timing**: 800ms between messages

## 🌐 Advanced Server Selection System

### Multi-Provider Server Pool

#### Primary Servers
1. **Cloudflare Global** - `https://speed.cloudflare.com`
   - Reliability: ⭐⭐⭐⭐⭐ 
   - Coverage: Global CDN
   - Capabilities: Download ✅ Upload ✅ Latency ✅

2. **Cloudflare Regional**
   - US: `https://cloudflare.com`
   - EU: `https://1.1.1.1` 
   - Asia: `https://1.0.0.1`

3. **Google Global** - `https://www.google.com`
   - Reliability: ⭐⭐⭐⭐
   - Capabilities: Download ✅ Upload ❌ Latency ✅

4. **Netflix Fast.com** - `https://fast.com`
   - Reliability: ⭐⭐⭐⭐
   - Specialization: Download testing
   - Max Test Size: 200MB

#### Backup Servers
5. **HTTPBin Test** - `https://httpbin.org`
   - Purpose: Development and fallback testing
   - Full capability support

### Intelligent Server Scoring Algorithm

```rust
Server Score = (Latency Score × 0.4) + 
               (Download Capability × 0.3) + 
               (Geographic Weight × 0.2) + 
               (Provider Reliability × 0.1) +
               (Proximity Bonus × 0.05)
```

#### Scoring Factors:
- **Latency Performance** (40%): <20ms=40pts, <50ms=35pts, <100ms=25pts
- **Download Capability** (30%): Real connection test results
- **Geographic Weight** (20%): Based on server location optimization
- **Provider Reliability** (10%): Cloudflare=10pts, Google=8pts, Netflix=7pts
- **Proximity Bonus** (5%): Country/region matching

### Fallback Strategy

1. **Primary Selection**: Best score >0.7 + latency <200ms
2. **Cloudflare Fallback**: Best Cloudflare server if available
3. **Any Working Server**: Any server with latency <1000ms + score >0.2

### Geographic Optimization

#### Location Detection Services
1. **ipapi.co** - Primary geolocation service
2. **ipinfo.io** - Secondary with coordinate data
3. **api.ipify.org** - Fallback for basic info

#### Server Selection Criteria
- **Same Country**: +5 point bonus
- **Regional CDN**: Prefer regional endpoints
- **ISP Optimization**: Consider ISP-specific routing

## 🎯 Animation Integration Points

### Speed Test Flow
1. **Server Discovery**: Cyberpunk scanner
2. **Latency Testing**: Ping pong animation
3. **Download Test**: Download arrows with data capture
4. **Upload Test**: Upload rockets with transmission
5. **Results**: Enhanced visual bars and quality indicators

### Diagnostics Flow
1. **Initialization**: Connection establishment sequence + matrix rain
2. **Gateway Detection**: Cyberpunk scanner
3. **DNS Analysis**: DNA helix spinner
4. **DNS Response**: Rocket boost spinner
5. **Route Tracing**: Progress bar with neural pathway mapping
6. **IPv6 Check**: Speed test flow spinner
7. **Results**: Comprehensive cybernetic analysis display

### Server Testing Flow
1. **Location Detection**: Cyberpunk scanner
2. **Multi-Server Test**: Network scanner for each server
3. **Performance Analysis**: Detailed metrics with quality scores
4. **Recommendation**: Final server selection with reasoning

## 🎮 Animation Showcase Mode

Access via interactive menu option 6 or:
```bash
echo "5" | ./target/release/netrunner_cli
```

Demonstrates all animation types:
1. Typing effects
2. Pulse animations
3. Matrix rain
4. Connection sequences
5. All spinner variations
6. Special effects combinations

## 🛠️ Technical Implementation

### Spinner Creation
```rust
pub fn create_pacman_spinner(&self, message: &str) -> ProgressBar {
    let pb = self.multi_progress.add(ProgressBar::new_spinner());
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("🎮 {spinner:.bright_yellow} {msg}")
            .tick_strings(&["ᗧ ••••••••••", "ᗤ  •••••••••", ...])
    );
    pb.set_message(message.to_string());
    pb.enable_steady_tick(Duration::from_millis(150));
    pb
}
```

### Dynamic Message Updates
```rust
// Dynamic progress messages during download
let download_messages = [
    "ACCESSING DATA NODES",
    "ESTABLISHING QUANTUM TUNNEL", 
    "SYNCHRONIZING NEURAL PACKETS",
    "DOWNLOADING CYBER STREAMS",
    "OPTIMIZING DATA FLOW",
    "CAPTURING DIGITAL ESSENCE",
];
```

## 🎨 Color Schemes

- **Cyberpunk Scanner**: Bright cyan with magenta accents
- **Data Operations**: Blue/green for downloads, magenta for uploads  
- **Status Messages**: Green for success, red for errors, yellow for warnings
- **Special Effects**: Bright colors with dimmer alternates for pulsing

## 📊 Performance Impact

- **Animation Overhead**: <1% CPU usage
- **Memory Impact**: Minimal (spinner state only)
- **Network Impact**: None (animations are purely visual)
- **Disable Option**: `--no-animation` flag available

## 🚀 Usage Examples

### Basic Speed Test with Animations
```bash
./target/release/netrunner_cli --mode speed
```

### Diagnostics with Full Effects
```bash  
./target/release/netrunner_cli --mode diag
```

### Server Analysis with Debug Info
```bash
./target/release/netrunner_cli --mode servers --debug-servers
```

### Animation Showcase
```bash
./target/release/netrunner_cli  # Select option 6
```

### Disable Animations
```bash
./target/release/netrunner_cli --no-animation --mode speed
```

## 🎯 Future Enhancements

- **Audio Effects**: Sound effects for completion events
- **Custom Themes**: User-selectable animation themes
- **Interactive Elements**: Click-to-skip long animations
- **Performance Scaling**: Adaptive animation complexity based on terminal capability
- **Server Health Monitoring**: Real-time server status tracking with visual indicators