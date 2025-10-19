//! # History Management Example
//!
//! This example demonstrates how to work with NetRunner's test history storage.
//!
//! Usage:
//!   cargo run --example history_management
//!
//! Features demonstrated:
//! - Running multiple speed tests
//! - Storing results in history database
//! - Retrieving historical results
//! - Calculating statistics
//! - Comparing current vs historical performance

use netrunner_cli::modules::{
    history::HistoryStorage,
    speed_test::SpeedTest,
    types::{DetailLevel, TestConfig},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║       NetRunner CLI - History Management Example         ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!();

    // Initialize history storage
    println!("📁 Initializing history storage...");
    let history = HistoryStorage::new()?;
    println!("✓ History storage initialized");
    println!();

    // Example 1: Run a speed test and save to history
    println!("═══════════════════════════════════════════════════════════");
    println!("Example 1: Run Speed Test & Save to History");
    println!("═══════════════════════════════════════════════════════════");
    println!();

    let config = TestConfig {
        server_url: "https://speed.cloudflare.com".to_string(),
        test_size_mb: 50,
        timeout_seconds: 60,
        json_output: true, // Suppress UI output
        animation_enabled: false,
        detail_level: DetailLevel::Standard,
        max_servers: 2,
    };

    println!("🚀 Running speed test...");
    let speed_test = SpeedTest::new(config)?;

    match speed_test.run_full_test().await {
        Ok(result) => {
            println!("✓ Speed test completed");
            println!("   • Download: {:.2} Mbps", result.download_mbps);
            println!("   • Upload: {:.2} Mbps", result.upload_mbps);
            println!("   • Ping: {:.2} ms", result.ping_ms);
            println!();

            // Save to history
            println!("💾 Saving result to history...");
            history.save_result(&result)?;
            println!("✓ Result saved");
            println!();
        }
        Err(e) => {
            println!("⚠ Speed test failed: {}", e);
            println!("   Continuing with existing history data...");
            println!();
        }
    }

    // Example 2: Retrieve recent tests
    println!("═══════════════════════════════════════════════════════════");
    println!("Example 2: Retrieve Recent Tests");
    println!("═══════════════════════════════════════════════════════════");
    println!();

    let limit = 10;
    match history.get_recent_results(limit) {
        Ok(results) => {
            println!("📊 Recent tests - {} tests found", results.len());
            println!();

            if results.is_empty() {
                println!("   No tests found. Run some speed tests first!");
            } else {
                println!("┌─────────────────────┬───────────┬───────────┬──────────┬──────────┐");
                println!("│ Timestamp           │ Download  │ Upload    │ Ping     │ Quality  │");
                println!("├─────────────────────┼───────────┼───────────┼──────────┼──────────┤");

                for result in results.iter().take(10) {
                    let timestamp = result.timestamp.format("%Y-%m-%d %H:%M:%S");
                    println!(
                        "│ {} │ {:>7.1} M │ {:>7.1} M │ {:>6.1} ms │ {:8?} │",
                        timestamp,
                        result.download_mbps,
                        result.upload_mbps,
                        result.ping_ms,
                        result.quality
                    );
                }

                println!("└─────────────────────┴───────────┴───────────┴──────────┴──────────┘");
            }
        }
        Err(e) => {
            println!("❌ Failed to retrieve history: {}", e);
        }
    }
    println!();

    // Example 3: Calculate statistics
    println!("═══════════════════════════════════════════════════════════");
    println!("Example 3: Calculate Statistics");
    println!("═══════════════════════════════════════════════════════════");
    println!();

    match history.get_statistics() {
        Ok(stats) => {
            println!("📈 Statistics (All time):");
            println!("   Total Tests: {}", stats.test_count);
            println!();

            println!("   Download Speed:");
            println!("     • Average: {:.2} Mbps", stats.avg_download_mbps);
            println!("     • Max:     {:.2} Mbps", stats.max_download_mbps);
            println!("     • Min:     {:.2} Mbps", stats.min_download_mbps);
            println!();

            println!("   Upload Speed:");
            println!("     • Average: {:.2} Mbps", stats.avg_upload_mbps);
            println!("     • Max:     {:.2} Mbps", stats.max_upload_mbps);
            println!("     • Min:     {:.2} Mbps", stats.min_upload_mbps);
            println!();

            println!("   Latency:");
            println!("     • Average: {:.2} ms", stats.avg_ping_ms);
            println!("     • Best:    {:.2} ms", stats.min_ping_ms);
            println!("     • Worst:   {:.2} ms", stats.max_ping_ms);
            println!();
        }
        Err(e) => {
            println!("❌ Failed to calculate statistics: {}", e);
        }
    }
    println!();

    // Example 4: Get fastest recorded speeds
    println!("═══════════════════════════════════════════════════════════");
    println!("Example 4: Record Speeds");
    println!("═══════════════════════════════════════════════════════════");
    println!();

    match history.get_fastest_download() {
        Ok(Some(result)) => {
            println!("🏆 Fastest Download:");
            println!("   • Speed: {:.2} Mbps", result.download_mbps);
            println!(
                "   • Date: {}",
                result.timestamp.format("%Y-%m-%d %H:%M:%S")
            );
            println!("   • Server: {}", result.server_location);
        }
        Ok(None) => {
            println!("   No download records found");
        }
        Err(e) => {
            println!("❌ Failed to get fastest download: {}", e);
        }
    }
    println!();

    match history.get_fastest_upload() {
        Ok(Some(result)) => {
            println!("🏆 Fastest Upload:");
            println!("   • Speed: {:.2} Mbps", result.upload_mbps);
            println!(
                "   • Date: {}",
                result.timestamp.format("%Y-%m-%d %H:%M:%S")
            );
            println!("   • Server: {}", result.server_location);
        }
        Ok(None) => {
            println!("   No upload records found");
        }
        Err(e) => {
            println!("❌ Failed to get fastest upload: {}", e);
        }
    }
    println!();

    // Example 5: Compare current vs historical average
    println!("═══════════════════════════════════════════════════════════");
    println!("Example 5: Performance Comparison");
    println!("═══════════════════════════════════════════════════════════");
    println!();

    match (history.get_recent_results(1), history.get_statistics()) {
        (Ok(results), Ok(stats)) if !results.is_empty() => {
            let latest = &results[0]; // Most recent test

            println!("🔍 Current vs Historical Average:");
            println!();

            let download_diff = ((latest.download_mbps - stats.avg_download_mbps)
                / stats.avg_download_mbps)
                * 100.0;
            println!("   Download: {:.2} Mbps", latest.download_mbps);
            if download_diff > 5.0 {
                println!("     ✓ {:.1}% faster than average", download_diff);
            } else if download_diff < -5.0 {
                println!("     ⚠ {:.1}% slower than average", download_diff.abs());
            } else {
                println!("     → Similar to average");
            }
            println!();

            let upload_diff =
                ((latest.upload_mbps - stats.avg_upload_mbps) / stats.avg_upload_mbps) * 100.0;
            println!("   Upload: {:.2} Mbps", latest.upload_mbps);
            if upload_diff > 5.0 {
                println!("     ✓ {:.1}% faster than average", upload_diff);
            } else if upload_diff < -5.0 {
                println!("     ⚠ {:.1}% slower than average", upload_diff.abs());
            } else {
                println!("     → Similar to average");
            }
            println!();

            let ping_diff = ((latest.ping_ms - stats.avg_ping_ms) / stats.avg_ping_ms) * 100.0;
            println!("   Latency: {:.2} ms", latest.ping_ms);
            if ping_diff < -5.0 {
                println!("     ✓ {:.1}% lower than average", ping_diff.abs());
            } else if ping_diff > 5.0 {
                println!("     ⚠ {:.1}% higher than average", ping_diff);
            } else {
                println!("     → Similar to average");
            }
        }
        _ => {
            println!("   No historical data for comparison");
        }
    }
    println!();

    // Example 6: Database Statistics
    println!("═══════════════════════════════════════════════════════════");
    println!("Example 6: Database Information");
    println!("═══════════════════════════════════════════════════════════");
    println!();

    match history.get_db_stats() {
        Ok(db_stats) => {
            println!("💾 Database Statistics:");
            println!("   • Size on Disk: {} bytes", db_stats.size_on_disk);
            println!("   • Result Count: {}", db_stats.results_count);
        }
        Err(e) => {
            println!("❌ Failed to get database stats: {}", e);
        }
    }
    println!();

    // Example 7: Export history to JSON
    println!("═══════════════════════════════════════════════════════════");
    println!("Example 7: Export History");
    println!("═══════════════════════════════════════════════════════════");
    println!();

    let filename = "speed_test_history.json";
    match history.export_to_json(filename) {
        Ok(()) => match history.count() {
            Ok(count) => {
                println!("💾 Exported {} test results to: {}", count, filename);
                println!("   You can analyze this data with external tools");
            }
            Err(_) => {
                println!("💾 Exported history to: {}", filename);
            }
        },
        Err(e) => {
            println!("❌ Export failed: {}", e);
        }
    }
    println!();

    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║                    Example Complete!                      ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!();
    println!("💡 Tips:");
    println!("   • History is stored in: ~/.netrunner_cli/history.db");
    println!("   • Automatic 30-day retention");
    println!("   • Use 'netrunner history' command to view in terminal");
    println!("   • Run this example multiple times to build history");

    Ok(())
}
