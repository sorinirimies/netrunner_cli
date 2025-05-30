mod modules;

use clap::{Arg, Command, ArgAction, value_parser};
use colored::*;
use dialoguer::{Select, theme::ColorfulTheme};
use std::io::{self, Write};
use tokio::signal;

use modules::{
    types::{TestConfig, DetailLevel, TestServer},
    ui::UI,
    speed_test::SpeedTest,
    diagnostics::NetworkDiagnosticsTool,
    history::HistoryStorage,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Handle Ctrl+C gracefully
    let ctrl_c = signal::ctrl_c();
    tokio::select! {
        _ = ctrl_c => {
            println!("\n{}", "Test cancelled by user".bright_red());
            return Ok(());
        },
        result = run_app() => {
            return result;
        }
    }
}

async fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("Netrunner Speed Test")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A feature-rich internet speed test & network diagnostics tool")
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            Arg::new("server")
                .short('s')
                .long("server")
                .value_name("URL")
                .help("Custom test server URL")
                .default_value("https://httpbin.org"),
        )
        .arg(
            Arg::new("size")
                .short('z')
                .long("size")
                .value_name("MB")
                .help("Test file size in MB")
                .value_parser(value_parser!(u64))
                .default_value("10"),
        )
        .arg(
            Arg::new("timeout")
                .short('t')
                .long("timeout")
                .value_name("SECONDS")
                .help("Timeout for each test in seconds")
                .value_parser(value_parser!(u64))
                .default_value("30"),
        )
        .arg(
            Arg::new("json")
                .short('j')
                .long("json")
                .help("Output results in JSON format")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no-animation")
                .short('n')
                .long("no-animation")
                .help("Disable animations")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("detail")
                .short('d')
                .long("detail")
                .value_name("LEVEL")
                .help("Detail level (basic, standard, detailed, debug)")
                .default_value("standard"),
        )
        .arg(
            Arg::new("mode")
                .short('m')
                .long("mode")
                .value_name("MODE")
                .help("Test mode (speed, diag, history, full)")
                .default_value("speed"),
        )
        .get_matches();

    let server_url = matches.get_one::<String>("server").unwrap().clone();
    let test_size_mb = *matches.get_one::<u64>("size").unwrap();
    let timeout_seconds = *matches.get_one::<u64>("timeout").unwrap();
    let json_output = matches.get_flag("json");
    let animation_enabled = !matches.get_flag("no-animation");
    
    let detail_level = match matches.get_one::<String>("detail").unwrap().as_str() {
        "basic" => DetailLevel::Basic,
        "detailed" => DetailLevel::Detailed,
        "debug" => DetailLevel::Debug,
        _ => DetailLevel::Standard,
    };
    
    let config = TestConfig {
        server_url,
        test_size_mb,
        timeout_seconds,
        json_output,
        animation_enabled,
        detail_level,
        max_servers: 3,
    };
    
    // If JSON output is requested, skip the interactive menu
    if json_output {
        return run_speed_test(&config).await;
    }
    
    // Initialize UI
    let ui = UI::new(config.clone());
    ui.clear_screen()?;
    ui.show_welcome_banner()?;
    
    // Parse command line mode or show interactive menu
    let mode = matches.get_one::<String>("mode").unwrap();
    match mode.as_str() {
        "speed" => run_speed_test(&config).await?,
        "diag" => run_diagnostics(&config).await?,
        "history" => show_history(&config).await?,
        "full" => run_full_test(&config).await?,
        _ => show_interactive_menu(&config).await?,
    }
    
    Ok(())
}

async fn show_interactive_menu(config: &TestConfig) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let options = vec![
            "🚀 Run Speed Test",
            "🔍 Run Network Diagnostics",
            "📈 View Test History",
            "🌐 Full Network Analysis",
            "❌ Exit",
        ];
        
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option")
            .default(0)
            .items(&options)
            .interact()?;
        
        match selection {
            0 => run_speed_test(config).await?,
            1 => run_diagnostics(config).await?,
            2 => show_history(config).await?,
            3 => run_full_test(config).await?,
            _ => {
                println!("{}", "Goodbye!".bright_blue());
                return Ok(());
            }
        }
        
        // Ask if the user wants to return to the menu
        println!();
        print!("{} ", "Return to main menu? [Y/n]:".bright_blue());
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        if input.trim().to_lowercase() == "n" {
            println!("{}", "Goodbye!".bright_blue());
            break;
        }
    }
    
    Ok(())
}

async fn run_speed_test(config: &TestConfig) -> Result<(), Box<dyn std::error::Error>> {
    // Create speed test
    let speed_test = SpeedTest::new(config.clone())?;
    
    // Run the test
    let result = speed_test.run_full_test().await?;
    
    // Save result to history if not in JSON mode
    if !config.json_output {
        match HistoryStorage::new() {
            Ok(storage) => {
                if let Err(e) = storage.save_result(&result) {
                    eprintln!("Failed to save test result: {}", e);
                }
            },
            Err(e) => {
                eprintln!("Failed to initialize history storage: {}", e);
            }
        }
    } else {
        // If JSON output is requested, print the result
        println!("{}", serde_json::to_string_pretty(&result)?);
    }
    
    Ok(())
}

async fn run_diagnostics(config: &TestConfig) -> Result<(), Box<dyn std::error::Error>> {
    // Create diagnostics tool
    let diagnostics_tool = NetworkDiagnosticsTool::new(config.clone());
    
    // Run diagnostics
    let result = diagnostics_tool.run_diagnostics().await?;
    
    // Output JSON if requested
    if config.json_output {
        println!("{}", serde_json::to_string_pretty(&result)?);
    }
    
    Ok(())
}

async fn show_history(config: &TestConfig) -> Result<(), Box<dyn std::error::Error>> {
    let ui = UI::new(config.clone());
    
    if !config.json_output {
        ui.show_section_header("Test History")?;
    }
    
    match HistoryStorage::new() {
        Ok(storage) => {
            // Get recent results and statistics
            let results = storage.get_recent_results(10)?;
            let stats = storage.get_statistics()?;
            
            if config.json_output {
                // Output JSON if requested
                let output = serde_json::json!({
                    "results": results,
                    "statistics": {
                        "avg_download_mbps": stats.avg_download_mbps,
                        "max_download_mbps": stats.max_download_mbps,
                        "min_download_mbps": stats.min_download_mbps,
                        "avg_upload_mbps": stats.avg_upload_mbps,
                        "max_upload_mbps": stats.max_upload_mbps,
                        "min_upload_mbps": stats.min_upload_mbps,
                        "avg_ping_ms": stats.avg_ping_ms,
                        "min_ping_ms": stats.min_ping_ms,
                        "max_ping_ms": stats.max_ping_ms,
                        "test_count": stats.test_count,
                    }
                });
                println!("{}", serde_json::to_string_pretty(&output)?);
            } else {
                // Display history table
                if results.is_empty() {
                    println!("{}", "No test results found in history.".yellow());
                } else {
                    // Display recent results
                    let mut table = prettytable::Table::new();
                    table.set_format(*prettytable::format::consts::FORMAT_BORDERS_ONLY);
                    
                    // Add header
                    table.add_row(prettytable::row![bF=> 
                        "Date", "Download (Mbps)", "Upload (Mbps)", "Ping (ms)", "Quality"
                    ]);
                    
                    // Add rows
                    for result in &results {
                        let quality_str = format!("{}", result.quality);
                        table.add_row(prettytable::row![
                            result.timestamp.format("%Y-%m-%d %H:%M").to_string(),
                            format!("{:.2}", result.download_mbps),
                            format!("{:.2}", result.upload_mbps),
                            format!("{:.2}", result.ping_ms),
                            quality_str
                        ]);
                    }
                    
                    table.printstd();
                    
                    // Display statistics
                    println!("\n{}", " 📊 STATISTICS 📊 ".on_bright_blue().white().bold());
                    println!("{}", "═════════════════════════".bright_blue());
                    println!("{}: {}", "Tests Recorded".bold(), stats.test_count);
                    println!("{}: {:.2} Mbps (Max: {:.2}, Min: {:.2})", 
                        "Average Download".bold(), 
                        stats.avg_download_mbps,
                        stats.max_download_mbps,
                        stats.min_download_mbps
                    );
                    println!("{}: {:.2} Mbps (Max: {:.2}, Min: {:.2})", 
                        "Average Upload".bold(), 
                        stats.avg_upload_mbps,
                        stats.max_upload_mbps,
                        stats.min_upload_mbps
                    );
                    println!("{}: {:.2} ms (Min: {:.2}, Max: {:.2})", 
                        "Average Ping".bold(), 
                        stats.avg_ping_ms,
                        stats.min_ping_ms,
                        stats.max_ping_ms
                    );
                }
            }
        },
        Err(e) => {
            if config.json_output {
                let error = serde_json::json!({ "error": e.to_string() });
                println!("{}", serde_json::to_string_pretty(&error)?);
            } else {
                ui.show_error(&format!("Failed to access history: {}", e))?;
            }
        }
    }
    
    Ok(())
}

async fn run_full_test(config: &TestConfig) -> Result<(), Box<dyn std::error::Error>> {
    let ui = UI::new(config.clone());
    
    if !config.json_output {
        ui.show_section_header("Running Full Network Analysis")?;
        println!("This will perform a complete network test, including speed test and diagnostics.");
        println!();
    }
    
    // Run speed test
    let speed_test = SpeedTest::new(config.clone())?;
    let speed_result = speed_test.run_full_test().await?;
    
    // Run diagnostics
    let diagnostics_tool = NetworkDiagnosticsTool::new(config.clone());
    let diag_result = diagnostics_tool.run_diagnostics().await?;
    
    // Save result to history
    if !config.json_output {
        match HistoryStorage::new() {
            Ok(storage) => {
                if let Err(e) = storage.save_result(&speed_result) {
                    eprintln!("Failed to save test result: {}", e);
                }
            },
            Err(e) => {
                eprintln!("Failed to initialize history storage: {}", e);
            }
        }
    } else {
        // If JSON output is requested, print combined results
        let combined_result = serde_json::json!({
            "speed_test": speed_result,
            "diagnostics": diag_result
        });
        println!("{}", serde_json::to_string_pretty(&combined_result)?);
    }
    
    Ok(())
}

// Helper function to simulate random ping with realistic values
fn thread_rng() -> ThreadRngWrapper {
    ThreadRngWrapper {}
}

struct ThreadRngWrapper {}

impl ThreadRngWrapper {
    fn gen_range_u64(&self, range: std::ops::Range<u64>) -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();
        
        range.start + (seed as u64 % (range.end - range.start))
    }
    
    fn gen_range_f64(&self, range: std::ops::Range<f64>) -> f64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos() as f64;
        
        range.start + (seed % 1000.0) / 1000.0 * (range.end - range.start)
    }
    
    fn gen_bool(&self, probability: f64) -> bool {
        use std::time::{SystemTime, UNIX_EPOCH};
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos() as f64;
        
        (seed % 1000.0) / 1000.0 < probability
    }
}
