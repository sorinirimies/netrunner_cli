use clap::{Arg, Command};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[derive(Debug, Serialize, Deserialize)]
struct SpeedTestResult {
    download_mbps: f64,
    upload_mbps: f64,
    ping_ms: f64,
    test_duration_seconds: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("Netrunner Speed Test")
        .version("0.1.0")
        .about("A CLI tool to test internet speed")
        .author("Greenspand")
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
                .default_value("10"),
        )
        .arg(
            Arg::new("timeout")
                .short('t')
                .long("timeout")
                .value_name("SECONDS")
                .help("Timeout for each test in seconds")
                .default_value("30"),
        )
        .arg(
            Arg::new("json")
                .short('j')
                .long("json")
                .help("Output results in JSON format")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let server_url = matches.get_one::<String>("server").unwrap();
    let test_size_mb: u64 = matches.get_one::<String>("size").unwrap().parse()?;
    let timeout_seconds: u64 = matches.get_one::<String>("timeout").unwrap().parse()?;
    let json_output = matches.get_flag("json");

    if !json_output {
        println!("🌐 NetSpeed - Internet Speed Test Tool");
        println!("=====================================");
        println!("Server: {}", server_url);
        println!("Test size: {} MB", test_size_mb);
        println!();
    }

    let client = Client::builder()
        .timeout(Duration::from_secs(timeout_seconds))
        .build()?;

    // Test ping
    let ping_ms = test_ping(&client, server_url, json_output).await?;

    // Test download speed
    let download_mbps = test_download_speed(&client, server_url, test_size_mb, json_output).await?;

    // Test upload speed
    let upload_mbps = test_upload_speed(&client, server_url, test_size_mb, json_output).await?;

    let result = SpeedTestResult {
        download_mbps,
        upload_mbps,
        ping_ms,
        test_duration_seconds: 0.0, // This would be calculated in a real implementation
    };

    if json_output {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("\n📊 Results:");
        println!("===========");
        println!("Ping: {:.2} ms", result.ping_ms);
        println!("Download: {:.2} Mbps", result.download_mbps);
        println!("Upload: {:.2} Mbps", result.upload_mbps);
    }

    Ok(())
}

async fn test_ping(
    client: &Client,
    server_url: &str,
    json_output: bool,
) -> Result<f64, Box<dyn std::error::Error>> {
    if !json_output {
        println!("🏓 Testing ping...");
    }

    let mut total_time = 0.0;
    let ping_count = 3;

    let pb = if !json_output {
        let pb = ProgressBar::new(ping_count);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}",
                )
                .unwrap()
                .progress_chars("#>-"),
        );
        Some(pb)
    } else {
        None
    };

    for i in 0..ping_count {
        let start = Instant::now();
        let response = client
            .get(&format!("{}/status/200", server_url))
            .send()
            .await;
        let duration = start.elapsed();

        match response {
            Ok(_) => {
                total_time += duration.as_millis() as f64;
                if let Some(ref pb) = pb {
                    pb.set_message(format!("Ping {}: {:.2}ms", i + 1, duration.as_millis()));
                    pb.inc(1);
                }
            }
            Err(e) => {
                if !json_output {
                    eprintln!("Ping failed: {}", e);
                }
                total_time += 1000.0; // Add 1 second penalty for failed ping
            }
        }

        if i < ping_count - 1 {
            sleep(Duration::from_millis(100)).await;
        }
    }

    if let Some(pb) = pb {
        pb.finish_with_message("Ping test completed");
    }

    Ok(total_time / ping_count as f64)
}

async fn test_download_speed(
    client: &Client,
    server_url: &str,
    size_mb: u64,
    json_output: bool,
) -> Result<f64, Box<dyn std::error::Error>> {
    if !json_output {
        println!("⬇️  Testing download speed...");
    }

    // For demo purposes, we'll download from httpbin's base64 endpoint
    // In a real implementation, you'd want a dedicated speed test server
    let test_url = format!(
        "{}/base64/{}",
        server_url,
        "a".repeat((size_mb * 1024) as usize)
    );

    let pb = if !json_output {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap(),
        );
        pb.enable_steady_tick(Duration::from_millis(100));
        Some(pb)
    } else {
        None
    };

    let start = Instant::now();

    match client.get(&test_url).send().await {
        Ok(response) => {
            let bytes = response.bytes().await?;
            let duration = start.elapsed();
            let duration_seconds = duration.as_secs_f64();
            let bytes_downloaded = bytes.len() as f64;
            let mbps = (bytes_downloaded * 8.0) / (duration_seconds * 1_000_000.0);

            if let Some(pb) = pb {
                pb.finish_with_message(format!(
                    "Downloaded {} bytes in {:.2}s",
                    bytes_downloaded, duration_seconds
                ));
            }

            Ok(mbps)
        }
        Err(e) => {
            if let Some(pb) = pb {
                pb.finish_with_message("Download test failed");
            }
            if !json_output {
                eprintln!("Download test failed: {}", e);
            }
            Ok(0.0)
        }
    }
}

async fn test_upload_speed(
    client: &Client,
    server_url: &str,
    size_mb: u64,
    json_output: bool,
) -> Result<f64, Box<dyn std::error::Error>> {
    if !json_output {
        println!("⬆️  Testing upload speed...");
    }

    // Create test data
    let test_data = "A".repeat((size_mb * 1024 * 100) as usize); // Smaller test for demo
    let test_url = format!("{}/post", server_url);

    let pb = if !json_output {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap(),
        );
        pb.enable_steady_tick(Duration::from_millis(100));
        Some(pb)
    } else {
        None
    };

    let start = Instant::now();

    match client.post(&test_url).body(test_data.clone()).send().await {
        Ok(_) => {
            let duration = start.elapsed();
            let duration_seconds = duration.as_secs_f64();
            let bytes_uploaded = test_data.len() as f64;
            let mbps = (bytes_uploaded * 8.0) / (duration_seconds * 1_000_000.0);

            if let Some(pb) = pb {
                pb.finish_with_message(format!(
                    "Uploaded {} bytes in {:.2}s",
                    bytes_uploaded, duration_seconds
                ));
            }

            Ok(mbps)
        }
        Err(e) => {
            if let Some(pb) = pb {
                pb.finish_with_message("Upload test failed");
            }
            if !json_output {
                eprintln!("Upload test failed: {}", e);
            }
            Ok(0.0)
        }
    }
}
