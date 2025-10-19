# Quick Reference: Geolocation & Server Selection

## What Was Fixed

### Before
```
🌍 Detecting your location...
Location: Unknown, Unknown
Selected: Cloudflare (Anycast) (Global, 999999 km)
```

### After
```
🌍 Detecting your location...
📍 Location: San Francisco, United States (via ipapi.co)
🔌 ISP: Comcast Cable Communications

⚡ Testing server performance...
✓ 3 servers selected for testing
  1. US West Coast Hub - 8.2 ms (15 km)
  2. LibreSpeed Los Angeles - 12.5 ms (560 km)
  3. US Central - 28.3 ms (2800 km)
```

**Note:** Failed geolocation services are silent by default. Use `NETRUNNER_DEBUG=1` to see trace logs.

## Key Improvements

### 1. Multiple Geolocation Services (5 total)
1. **ipapi.co** - Primary
2. **ip-api.com** - Secondary  
3. **ipinfo.io** - Tertiary
4. **freegeoip.app** - Quaternary
5. **ipwhois.app** - Final fallback

### 2. Smart Fallback
- If all services fail → Uses Kansas City, USA (geographic center)
- Never shows "Unknown, Unknown"
- Clear error messages for each failed service

### 3. Better Server Selection
- **Quality Score** = `(10000 × weight) / (latency + distance/100)`
- Tests 15 servers concurrently
- Selects best 3 based on:
  - Latency (lower is better)
  - Distance (closer is better)
  - Geographic weight (regional > global)

### 4. Realistic Distances
- Regional servers: Actual calculated distance
- Global CDN: 5000 km (not 999999 km)
- Backup servers: Lower priority but usable

## Test Coverage

```
✓ 88 tests total (0 failed, 0 ignored)
  - 20 geolocation tests
  - 26 server selection tests
  - 42 other tests
```

## Validation

### Coordinate Validation
- Latitude: -90° to +90°
- Longitude: -180° to +180°
- Rejects (0, 0) as invalid
- Rejects empty/Unknown values

### Data Validation
- HTTP status must be 2xx
- API error fields checked
- Country/city must not be empty
- ISP is optional

## Server Quality Formula

```rust
latency_penalty = max(latency_ms, 1.0)
distance_penalty = max(distance_km / 100.0, 1.0)
quality_score = (10000.0 × geographic_weight) / (latency_penalty + distance_penalty)
```

**Higher score = Better server**

## Geographic Weights

| Server Type | Weight | Priority |
|-------------|--------|----------|
| Regional    | 1.0    | Highest  |
| Continent   | 0.9    | High     |
| Global CDN  | 0.5    | Medium   |
| Backup      | 0.3    | Lowest   |

## Error Handling

### Silent Failover (Default)
Failed services are silent - only successful connection is shown:
```
🌍 Detecting your location...
📍 Location: Berlin, Germany (via ipinfo.io)
```

### Debug Mode
Enable trace logging with `NETRUNNER_DEBUG=1`:
```bash
NETRUNNER_DEBUG=1 netrunner speed
```

Output:
```
🌍 Detecting your location...
[TRACE] ipapi.co geolocation failed: HTTP error: 429 Too Many Requests
[TRACE] ip-api.com geolocation failed: timeout
📍 Location: Berlin, Germany (via ipinfo.io)
```

### All Services Failed
```
⚠ Using default location (USA Central) - all geolocation services failed
```

## Files Changed

1. **src/modules/speed_test.rs**
   - Added 5 geolocation API methods
   - Enhanced `detect_location()` with validation
   - Improved `select_best_servers()` algorithm
   - Better error messages

2. **tests/geolocation_tests.rs** (NEW)
   - 20 comprehensive tests
   - Real API integration test
   - Validation tests

3. **tests/speed_test_tests.rs** (ENHANCED)
   - Added 9 new tests
   - Server selection tests
   - Quality score tests

## Usage

No changes required! The improvements are automatic:

```bash
netrunner speed          # Uses new geolocation automatically
netrunner speed --json   # JSON output (no UI messages)
```

## Performance

- **Timeout per service**: 5 seconds
- **Total max time**: ~25 seconds for all 5 services (if all fail)
- **Typical time**: 1-3 seconds (first service succeeds)
- **Server testing**: Concurrent (15 servers tested in parallel)

## Debugging

### Geolocation Trace Logs
To see why geolocation services failed:
```bash
NETRUNNER_DEBUG=1 netrunner speed
```

### Full Debug Output
For complete debug information:
```bash
RUST_LOG=debug NETRUNNER_DEBUG=1 netrunner speed
```

## Documentation

See `GEOLOCATION_SERVER_FIX.md` for complete technical details.

---

**Status**: ✅ Production Ready  
**Tests**: ✅ 88/88 Passing  
**Build**: ✅ No Errors/Warnings