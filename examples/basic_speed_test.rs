//! # Basic Speed Test Example
//!
//! This example demonstrates how to run a basic speed test programmatically.
//!
//! Usage:
//!   cargo run --example basic_speed_test
//!
//! Features demonstrated:
//! - Running a speed test
//! - Accessing test results
//! - Displaying download/upload speeds
//! - Showing connection quality

use netrunner_cli::modules::{
    speed_test::SpeedTest,
    types::{DetailLevel, TestConfig},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║          NetRunner CLI - Basic Speed Test Example        ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!();

    // Create test configuration
    let config = TestConfig {
        server_url: "https://speed.cloudflare.com".to_string(),
        test_size_mb: 100, // 100 MB test
        timeout_seconds: 60,
        json_output: false,
        animation_enabled: true,
        detail_level: DetailLevel::Standard,
        max_servers: 3,
    };

    println!("📋 Configuration:");
    println!("   • Test Size: {} MB", config.test_size_mb);
    println!("   • Timeout: {} seconds", config.timeout_seconds);
    println!("   • Max Servers: {}", config.max_servers);
    println!();

    // Create speed test instance
    println!("🔧 Initializing speed test...");
    let speed_test = SpeedTest::new(config)?;
    println!("✓ Speed test initialized");
    println!();

    // Run the test
    println!("🚀 Running speed test...");
    println!("   This will take approximately 30-40 seconds");
    println!();

    match speed_test.run_full_test().await {
        Ok(result) => {
            println!();
            println!("╔═══════════════════════════════════════════════════════════╗");
            println!("║                     TEST RESULTS                          ║");
            println!("╚═══════════════════════════════════════════════════════════╝");
            println!();
            println!("📊 Speed Metrics:");
            println!("   ↓ Download: {:.2} Mbps", result.download_mbps);
            println!("   ↑ Upload:   {:.2} Mbps", result.upload_mbps);
            println!("   📡 Ping:    {:.2} ms", result.ping_ms);
            println!("   📊 Jitter:  {:.2} ms", result.jitter_ms);
            println!();

            println!("🌍 Network Information:");
            println!("   • Server:   {}", result.server_location);
            if let Some(ip) = &result.server_ip {
                println!("   • Server IP: {}", ip);
            }
            if let Some(client_ip) = &result.client_ip {
                println!("   • Your IP:  {}", client_ip);
            }
            if let Some(isp) = &result.isp {
                println!("   • ISP:      {}", isp);
            }
            println!();

            println!("⚡ Connection Quality:");
            println!("   • Rating:    {:?}", result.quality);
            println!("   • Packet Loss: {:.2}%", result.packet_loss_percent);
            println!("   • Duration: {:.1}s", result.test_duration_seconds);
            println!();

            // Performance analysis
            println!("📈 Performance Analysis:");
            if result.download_mbps >= 100.0 {
                println!("   ✓ Excellent download speed for HD streaming and gaming");
            } else if result.download_mbps >= 50.0 {
                println!("   ✓ Good download speed for most online activities");
            } else {
                println!("   ⚠ Download speed may be slow for HD content");
            }

            if result.upload_mbps >= 20.0 {
                println!("   ✓ Excellent upload speed for video calls and cloud uploads");
            } else if result.upload_mbps >= 10.0 {
                println!("   ✓ Good upload speed for video conferencing");
            } else {
                println!("   ⚠ Upload speed may be slow for video calls");
            }

            if result.ping_ms <= 20.0 {
                println!("   ✓ Excellent latency for online gaming");
            } else if result.ping_ms <= 50.0 {
                println!("   ✓ Good latency for most activities");
            } else {
                println!("   ⚠ High latency may affect real-time applications");
            }
            println!();

            println!("✅ Speed test completed successfully!");
        }
        Err(e) => {
            eprintln!("❌ Speed test failed: {}", e);
            eprintln!();
            eprintln!("Possible causes:");
            eprintln!("  • Network connectivity issues");
            eprintln!("  • Firewall blocking connections");
            eprintln!("  • Server temporarily unavailable");
            eprintln!("  • Timeout too short for your connection");
            return Err(e);
        }
    }

    Ok(())
}
