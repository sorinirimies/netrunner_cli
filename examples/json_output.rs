//! # JSON Output Example
//!
//! This example demonstrates how to run a speed test and get JSON output
//! for integration with other tools, CI/CD pipelines, or data analysis.
//!
//! Usage:
//!   cargo run --example json_output
//!
//! Features demonstrated:
//! - Running speed test with JSON output
//! - Serializing results to JSON
//! - Parsing JSON output
//! - Saving results to file
//! - Integration patterns

use netrunner_cli::modules::{
    speed_test::SpeedTest,
    types::{DetailLevel, TestConfig},
};
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║          NetRunner CLI - JSON Output Example             ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!();

    // Create test configuration with JSON output enabled
    let config = TestConfig {
        server_url: "https://speed.cloudflare.com".to_string(),
        test_size_mb: 50, // Smaller test for faster results
        timeout_seconds: 60,
        json_output: true, // Enable JSON output mode (suppresses UI)
        animation_enabled: false,
        detail_level: DetailLevel::Standard,
        max_servers: 2,
    };

    println!("📋 Running speed test in JSON mode...");
    println!("   (This suppresses UI output for clean JSON results)");
    println!();

    // Create and run speed test
    let speed_test = SpeedTest::new(config)?;

    match speed_test.run_full_test().await {
        Ok(result) => {
            println!("✅ Speed test completed!");
            println!();

            // Example 1: Serialize to pretty JSON
            println!("📄 Example 1: Pretty JSON Output");
            println!("─────────────────────────────────────────────────────────");
            let json_pretty = serde_json::to_string_pretty(&result)?;
            println!("{}", json_pretty);
            println!();

            // Example 2: Serialize to compact JSON
            println!("📄 Example 2: Compact JSON (for piping)");
            println!("─────────────────────────────────────────────────────────");
            let json_compact = serde_json::to_string(&result)?;
            println!("{}", json_compact);
            println!();

            // Example 3: Save to file
            println!("💾 Example 3: Save to File");
            println!("─────────────────────────────────────────────────────────");
            let filename = "speed_test_result.json";
            fs::write(filename, &json_pretty)?;
            println!("✓ Saved to: {}", filename);
            println!();

            // Example 4: Extract specific fields
            println!("🔍 Example 4: Extract Specific Fields");
            println!("─────────────────────────────────────────────────────────");
            println!("Download Speed: {:.2} Mbps", result.download_mbps);
            println!("Upload Speed:   {:.2} Mbps", result.upload_mbps);
            println!("Ping:          {:.2} ms", result.ping_ms);
            println!("Quality:       {:?}", result.quality);
            println!();

            // Example 5: Conditional logic based on results
            println!("⚙️  Example 5: Conditional Logic");
            println!("─────────────────────────────────────────────────────────");

            // Check if speed meets requirements
            let min_download_required = 100.0;
            let min_upload_required = 20.0;
            let max_ping_allowed = 50.0;

            let speed_ok = result.download_mbps >= min_download_required;
            let upload_ok = result.upload_mbps >= min_upload_required;
            let latency_ok = result.ping_ms <= max_ping_allowed;

            println!("Requirements Check:");
            println!(
                "  • Download ≥ {} Mbps: {}",
                min_download_required,
                if speed_ok { "✓ PASS" } else { "✗ FAIL" }
            );
            println!(
                "  • Upload ≥ {} Mbps: {}",
                min_upload_required,
                if upload_ok { "✓ PASS" } else { "✗ FAIL" }
            );
            println!(
                "  • Ping ≤ {} ms: {}",
                max_ping_allowed,
                if latency_ok { "✓ PASS" } else { "✗ FAIL" }
            );
            println!();

            if speed_ok && upload_ok && latency_ok {
                println!("✅ All requirements met!");
                std::process::exit(0);
            } else {
                println!("❌ Some requirements not met");
                std::process::exit(1);
            }
        }
        Err(e) => {
            // Even errors can be structured as JSON
            let error_json = serde_json::json!({
                "success": false,
                "error": e.to_string(),
                "timestamp": chrono::Utc::now().to_rfc3339()
            });

            eprintln!("❌ Speed test failed");
            eprintln!("{}", serde_json::to_string_pretty(&error_json)?);
            std::process::exit(1);
        }
    }
}

// Example integration patterns in comments:
//
// Bash/Shell:
// ```bash
// # Get download speed
// cargo run --example json_output 2>/dev/null | jq -r '.download_mbps'
//
// # Check if speed is acceptable
// SPEED=$(cargo run --example json_output 2>/dev/null | jq -r '.download_mbps')
// if (( $(echo "$SPEED > 100" | bc -l) )); then
//     echo "Speed OK"
// else
//     echo "Speed too slow"
//     exit 1
// fi
// ```
//
// Python:
// ```python
// import json
// import subprocess
//
// result = subprocess.run(
//     ['cargo', 'run', '--example', 'json_output'],
//     capture_output=True,
//     text=True
// )
//
// data = json.loads(result.stdout)
// print(f"Download: {data['download_mbps']} Mbps")
// print(f"Quality: {data['quality']}")
// ```
//
// CI/CD (GitHub Actions):
// ```yaml
// - name: Check Network Speed
//   run: |
//     RESULT=$(netrunner speed --json)
//     SPEED=$(echo "$RESULT" | jq -r '.download_mbps')
//     if (( $(echo "$SPEED < 50" | bc -l) )); then
//       echo "::warning::Network speed below threshold: $SPEED Mbps"
//     fi
// ```
