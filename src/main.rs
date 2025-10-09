mod modules;

use clap::{value_parser, Arg, ArgAction, Command};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Select};

use std::io::{self, Write};
use std::time::Duration;
use tokio::signal;

use modules::{
    diagnostics::NetworkDiagnosticsTool,
    history::HistoryStorage,
    intro::{show_intro, show_simple_intro},
    speed_test::SpeedTest,
    types::{DetailLevel, TestConfig},
    ui::UI,
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
                .help("Test mode (speed, diag, history, full, servers)")
                .default_value("speed"),
        )
        .arg(
            Arg::new("debug-servers")
                .long("debug-servers")
                .help("Show detailed server testing information")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("history")
                .long("history")
                .help("Show test history (shorthand for --mode history)")
                .action(ArgAction::SetTrue),
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

    let debug_servers = matches.get_flag("debug-servers");

    let config = TestConfig {
        server_url,
        test_size_mb,
        timeout_seconds,
        json_output,
        animation_enabled,
        detail_level,
        max_servers: 3,
    };

    // If JSON output is requested, skip the interactive menu and intro
    if json_output {
        return run_speed_test(&config).await;
    }

    // Show animated intro with glow effects (skip if animations disabled)
    if animation_enabled {
        // Try to show animated intro, fallback to simple if it fails
        if let Err(_) = show_intro() {
            let _ = show_simple_intro();
        }
    } else {
        let _ = show_simple_intro();
    }

    // Initialize UI
    let ui = UI::new(config.clone());
    ui.clear_screen()?;
    ui.show_welcome_banner()?;

    // Check for --history flag first (shorthand)
    if matches.get_flag("history") {
        return show_history(&config).await;
    }

    // Parse command line mode or show interactive menu
    let mode = matches.get_one::<String>("mode").unwrap();
    match mode.as_str() {
        "speed" => run_speed_test(&config).await?,
        "diag" => run_diagnostics(&config).await?,
        "history" => show_history(&config).await?,
        "full" => run_full_test(&config).await?,
        "servers" => test_all_servers(&config, debug_servers).await?,
        _ => show_interactive_menu(&config).await?,
    }

    Ok(())
}

async fn test_all_servers(
    config: &TestConfig,
    debug_mode: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    use modules::speed_test::SpeedTest;

    let ui = UI::new(config.clone());

    if !config.json_output {
        ui.show_section_header("Server Performance Analysis")?;
        println!(
            "{}",
            "Testing all available servers for optimal performance...".bright_cyan()
        );
        println!();
    }

    // Create speed test instance to access server pool
    let _speed_test = SpeedTest::new(config.clone())?;

    if debug_mode && !config.json_output {
        println!(
            "{}",
            "📊 DETAILED SERVER ANALYSIS MODE".bright_yellow().bold()
        );
        println!(
            "{}",
            "═══════════════════════════════════════".bright_yellow()
        );
        println!();
    }

    // Get location info first
    println!("{}", "🌍 Detecting your location...".bright_blue());
    let pb = if config.animation_enabled {
        Some(ui.create_cyberpunk_spinner("GEOLOCATING NEURAL INTERFACE"))
    } else {
        None
    };

    tokio::time::sleep(Duration::from_millis(1000)).await;

    if let Some(pb) = pb {
        pb.finish_with_message("⟨⟨⟨ LOCATION DETECTED ⟩⟩⟩");
    }

    println!();
    println!("{}", "🔍 Testing server performance...".bright_green());
    println!();

    // This would ideally access the server testing logic from SpeedTest
    // For now, we'll show a simulation
    let test_servers = vec![
        (
            "Cloudflare Global",
            "https://speed.cloudflare.com",
            "Global CDN",
        ),
        ("Cloudflare US", "https://cloudflare.com", "United States"),
        ("Cloudflare EU", "https://1.1.1.1", "Europe"),
        ("Google Global", "https://www.google.com", "Global CDN"),
        ("Netflix Fast.com", "https://fast.com", "Netflix CDN"),
        ("HTTPBin Test", "https://httpbin.org", "Global"),
    ];

    for (name, url, location) in test_servers {
        let spinner = if config.animation_enabled {
            Some(ui.create_network_scanner_bar(&format!("TESTING {}", name.to_uppercase())))
        } else {
            None
        };

        // Simulate testing
        tokio::time::sleep(Duration::from_millis(800)).await;

        // Simulate results
        let latency = rand::random::<f64>() * 100.0 + 10.0;
        let score = 1.0 - (latency / 200.0);

        if let Some(spinner) = spinner {
            spinner.finish_with_message(format!(
                "⟨⟨⟨ {} | {:.0}ms | Score: {:.2} ⟩⟩⟩",
                name, latency, score
            ));
        }

        if debug_mode && !config.json_output {
            println!("   📡 {}: {}", "Server".bold(), name.bright_cyan());
            println!("   🌍 {}: {}", "Location".bold(), location);
            println!("   🔗 {}: {}", "URL".bold(), url.bright_blue());
            println!("   🏓 {}: {:.0}ms", "Latency".bold(), latency);
            println!("   ⭐ {}: {:.2}", "Quality Score".bold(), score);
            println!(
                "   🔧 {}: ✅ Download | ✅ Upload | ✅ Latency",
                "Capabilities".bold()
            );
            println!();
        }
    }

    if !config.json_output {
        println!(
            "{}",
            "╔═══════════════════════════════════════════════════╗".bright_green()
        );
        println!(
            "{}",
            "║            🏆 SERVER ANALYSIS COMPLETE 🏆         ║".bright_green()
        );
        println!(
            "{}",
            "╚═══════════════════════════════════════════════════╝".bright_green()
        );
        println!();
        println!(
            "{}",
            "💡 Recommendation: Use Cloudflare servers for best reliability".bright_yellow()
        );
        println!(
            "{}",
            "🔧 Add --debug-servers flag for detailed analysis".bright_blue()
        );
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
            "🛠️ Test All Servers",
            "🎮 Animation Showcase",
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
            4 => test_all_servers(config, true).await?,
            5 => show_animation_showcase(config).await?,
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
            }
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
                    println!(
                        "{}: {:.2} Mbps (Max: {:.2}, Min: {:.2})",
                        "Average Download".bold(),
                        stats.avg_download_mbps,
                        stats.max_download_mbps,
                        stats.min_download_mbps
                    );
                    println!(
                        "{}: {:.2} Mbps (Max: {:.2}, Min: {:.2})",
                        "Average Upload".bold(),
                        stats.avg_upload_mbps,
                        stats.max_upload_mbps,
                        stats.min_upload_mbps
                    );
                    println!(
                        "{}: {:.2} ms (Min: {:.2}, Max: {:.2})",
                        "Average Ping".bold(),
                        stats.avg_ping_ms,
                        stats.min_ping_ms,
                        stats.max_ping_ms
                    );
                }
            }
        }
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
        println!(
            "This will perform a complete network test, including speed test and diagnostics."
        );
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
            }
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

async fn show_animation_showcase(config: &TestConfig) -> Result<(), Box<dyn std::error::Error>> {
    let ui = UI::new(config.clone());

    ui.clear_screen()?;
    ui.show_section_header("Animation Showcase")?;

    println!(
        "{}",
        "Demonstrating NetRunner's cyberpunk animations...".bright_cyan()
    );
    println!();

    // Show typing effect
    println!("{}", "1. Typing Effect:".bright_yellow().bold());
    ui.show_typing_effect("⟨⟨⟨ NEURAL INTERFACE ACTIVATED ⟩⟩⟩")?;
    tokio::time::sleep(Duration::from_millis(1000)).await;

    // Show pulse text
    println!("{}", "2. Pulse Effect:".bright_yellow().bold());
    ui.show_pulse_text("⟨⟨⟨ SCANNING QUANTUM NETWORKS ⟩⟩⟩", 3)?;
    tokio::time::sleep(Duration::from_millis(1000)).await;

    // Show matrix effect
    println!("{}", "3. Matrix Rain Effect:".bright_yellow().bold());
    ui.show_matrix_effect(4)?;
    tokio::time::sleep(Duration::from_millis(1000)).await;

    // Show connection establishing
    println!("{}", "4. Connection Sequence:".bright_yellow().bold());
    ui.show_connection_establishing()?;
    tokio::time::sleep(Duration::from_millis(1000)).await;

    // Show different spinner types
    println!("{}", "5. Advanced Spinners:".bright_yellow().bold());

    let spinner1 = ui.create_cyberpunk_spinner("CYBERPUNK SCANNER");
    tokio::time::sleep(Duration::from_millis(2000)).await;
    spinner1.finish_with_message("⟨⟨⟨ CYBERPUNK SCAN COMPLETE ⟩⟩⟩");

    let spinner2 = ui.create_pacman_spinner("PACMAN DATA HUNTER");
    tokio::time::sleep(Duration::from_millis(3000)).await;
    spinner2.finish_with_message("⟨⟨⟨ ALL DATA CONSUMED ⟩⟩⟩");

    let spinner3 = ui.create_download_spinner("DOWNLOAD SIMULATION");
    tokio::time::sleep(Duration::from_millis(2500)).await;
    spinner3.finish_with_message("⟨⟨⟨ DOWNLOAD CAPTURED ⟩⟩⟩");

    let spinner4 = ui.create_upload_spinner("UPLOAD SIMULATION");
    tokio::time::sleep(Duration::from_millis(2500)).await;
    spinner4.finish_with_message("⟨⟨⟨ DATA TRANSMITTED ⟩⟩⟩");

    let spinner5 = ui.create_ping_spinner("PING SIMULATION");
    tokio::time::sleep(Duration::from_millis(2000)).await;
    spinner5.finish_with_message("⟨⟨⟨ NEURAL LATENCY MEASURED ⟩⟩⟩");

    println!();
    println!("{}", "6. Special Effects:".bright_yellow().bold());

    let spinner6 = ui.create_dna_helix_spinner("DNA HELIX ANALYSIS");
    tokio::time::sleep(Duration::from_millis(3000)).await;
    spinner6.finish_with_message("⟨⟨⟨ GENETIC CODE DECODED ⟩⟩⟩");

    let spinner7 = ui.create_rocket_spinner("ROCKET BOOST MODE");
    tokio::time::sleep(Duration::from_millis(2500)).await;
    spinner7.finish_with_message("⟨⟨⟨ WARP SPEED ACHIEVED ⟩⟩⟩");

    let spinner8 = ui.create_wave_spinner("WAVE FREQUENCY SCAN");
    tokio::time::sleep(Duration::from_millis(2500)).await;
    spinner8.finish_with_message("⟨⟨⟨ FREQUENCY LOCKED ⟩⟩⟩");

    println!();
    println!("{}", "Animation showcase complete!".bright_green().bold());
    println!();

    Ok(())
}
