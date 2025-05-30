use colored::*;
use console::{Term, style};
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use spinners::{Spinner, Spinners};
use std::io::{self, Write};
use std::time::Duration;
use prettytable::{Table, Row, Cell, format};

use crate::modules::types::{SpeedTestResult, ConnectionQuality, TestConfig, DetailLevel};

pub struct UI {
    term: Term,
    config: TestConfig,
    multi_progress: MultiProgress,
}

impl UI {
    pub fn new(config: TestConfig) -> Self {
        Self {
            term: Term::stdout(),
            config,
            multi_progress: MultiProgress::new(),
        }
    }

    pub fn clear_screen(&self) -> io::Result<()> {
        self.term.clear_screen()
    }

    pub fn show_welcome_banner(&self) -> io::Result<()> {
        self.term.clear_screen()?;
        
        let banner = r#"
 _   _ _____ _______  __ _   _ _   _ _   _ _____ ____  
| \ | | ____|_   _\ \/ /| \ | | \ | | \ | | ____|  _ \ 
|  \| |  _|   | |  \  / |  \| |  \| |  \| |  _| | |_) |
| |\  | |___  | |  /  \ | |\  | |\  | |\  | |___|  _ < 
|_| \_|_____| |_| /_/\_\|_| \_|_| \_|_| \_|_____|_| \_\
                                                       
        "#;

        println!("{}", banner.bright_cyan());
        println!("{}", "⚡ Internet Speed Test & Network Diagnostics ⚡".bright_green());
        println!("{}", "============================================".bright_blue());
        println!();
        println!("{}", "💻 Analyze your connection with style 💻".bright_yellow());
        println!();

        Ok(())
    }

    pub fn create_progress_bar(&self, len: u64, message: &str) -> ProgressBar {
        let pb = self.multi_progress.add(ProgressBar::new(len));
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                .unwrap()
                .progress_chars("█▓▒░  ")
        );
        pb.set_message(message.to_string());
        pb
    }

    pub fn create_spinner(&self, message: &str) -> ProgressBar {
        let pb = self.multi_progress.add(ProgressBar::new_spinner());
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap()
        );
        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(80));
        pb
    }

    pub fn animated_text(&self, text: &str, delay_ms: u64) -> io::Result<()> {
        for c in text.chars() {
            print!("{}", c);
            io::stdout().flush()?;
            std::thread::sleep(Duration::from_millis(delay_ms));
        }
        println!();
        Ok(())
    }

    pub fn show_section_header(&self, title: &str) -> io::Result<()> {
        println!();
        println!("{}", format!("🔹 {} 🔹", title).bright_blue().bold());
        println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_blue());
        Ok(())
    }

    pub fn show_results_dashboard(&self, result: &SpeedTestResult) -> io::Result<()> {
        self.term.clear_screen()?;
        
        // Create a fancy header
        println!("\n{}", " 📊 SPEED TEST RESULTS 📊 ".on_bright_blue().white().bold());
        println!("{}", "═════════════════════════".bright_blue());

        // Create a table for the results
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_BORDERS_ONLY);
        
        // Add header row
        table.add_row(Row::new(vec![
            Cell::new("Metric").style_spec("Fb"),
            Cell::new("Value").style_spec("Fb"),
            Cell::new("Rating").style_spec("Fb"),
        ]));
        
        // Add download speed
        let download_rating = match result.download_mbps {
            d if d >= 100.0 => "🚀 Excellent".green(),
            d if d >= 50.0 => "✅ Good".bright_green(),
            d if d >= 25.0 => "👍 Average".yellow(),
            d if d >= 10.0 => "⚠️ Poor".bright_red(),
            d if d > 0.0 => "❌ Very Poor".red(),
            _ => "💀 Failed".bright_black(),
        };
        
        table.add_row(Row::new(vec![
            Cell::new("Download").style_spec("Fb"),
            Cell::new(&format!("{:.2} Mbps", result.download_mbps)).style_spec("Fr"),
            Cell::new(&format!("{}", download_rating)),
        ]));
        
        // Add upload speed
        let upload_rating = match result.upload_mbps {
            u if u >= 20.0 => "🚀 Excellent".green(),
            u if u >= 10.0 => "✅ Good".bright_green(),
            u if u >= 5.0 => "👍 Average".yellow(),
            u if u >= 2.0 => "⚠️ Poor".bright_red(),
            u if u > 0.0 => "❌ Very Poor".red(),
            _ => "💀 Failed".bright_black(),
        };
        
        table.add_row(Row::new(vec![
            Cell::new("Upload").style_spec("Fb"),
            Cell::new(&format!("{:.2} Mbps", result.upload_mbps)).style_spec("Fr"),
            Cell::new(&format!("{}", upload_rating)),
        ]));
        
        // Add ping
        let ping_rating = match result.ping_ms {
            p if p < 20.0 => "🚀 Excellent".green(),
            p if p < 50.0 => "✅ Good".bright_green(),
            p if p < 100.0 => "👍 Average".yellow(),
            p if p < 150.0 => "⚠️ Poor".bright_red(),
            p if p > 0.0 => "❌ Very Poor".red(),
            _ => "💀 Failed".bright_black(),
        };
        
        table.add_row(Row::new(vec![
            Cell::new("Ping").style_spec("Fb"),
            Cell::new(&format!("{:.2} ms", result.ping_ms)).style_spec("Fr"),
            Cell::new(&format!("{}", ping_rating)),
        ]));
        
        // Add jitter if available
        if result.jitter_ms > 0.0 {
            let jitter_rating = match result.jitter_ms {
                j if j < 5.0 => "🚀 Excellent".green(),
                j if j < 15.0 => "✅ Good".bright_green(),
                j if j < 25.0 => "👍 Average".yellow(),
                j if j < 40.0 => "⚠️ Poor".bright_red(),
                _ => "❌ Very Poor".red(),
            };
            
            table.add_row(Row::new(vec![
                Cell::new("Jitter").style_spec("Fb"),
                Cell::new(&format!("{:.2} ms", result.jitter_ms)).style_spec("Fr"),
                Cell::new(&format!("{}", jitter_rating)),
            ]));
        }
        
        // Add packet loss if available
        if self.config.detail_level >= DetailLevel::Standard && result.packet_loss_percent >= 0.0 {
            let packet_loss_rating = match result.packet_loss_percent {
                p if p < 0.1 => "🚀 Excellent".green(),
                p if p < 1.0 => "✅ Good".bright_green(),
                p if p < 2.5 => "👍 Average".yellow(),
                p if p < 5.0 => "⚠️ Poor".bright_red(),
                _ => "❌ Very Poor".red(),
            };
            
            table.add_row(Row::new(vec![
                Cell::new("Packet Loss").style_spec("Fb"),
                Cell::new(&format!("{:.2}%", result.packet_loss_percent)).style_spec("Fr"),
                Cell::new(&format!("{}", packet_loss_rating)),
            ]));
        }
        
        // Print the table
        table.printstd();
        
        // Overall quality rating
        let quality_color = match result.quality {
            ConnectionQuality::Excellent => "bright green",
            ConnectionQuality::Good => "green",
            ConnectionQuality::Average => "yellow",
            ConnectionQuality::Poor => "bright red",
            ConnectionQuality::VeryPoor => "red",
            ConnectionQuality::Failed => "bright black",
        };
        
        println!("\n{} {}", "Overall Quality:".bold(), format!("{}", result.quality).color(quality_color).bold());
        
        // Server information
        if self.config.detail_level >= DetailLevel::Standard {
            println!("\n{}", " 🖥️  SERVER INFORMATION 🖥️  ".on_bright_blue().white().bold());
            println!("{}", "═════════════════════════".bright_blue());
            println!("{} {}", "Server Location:".bold(), result.server_location);
            
            if let Some(server_ip) = result.server_ip {
                println!("{} {}", "Server IP:".bold(), server_ip);
            }
            
            if let Some(client_ip) = result.client_ip {
                println!("{} {}", "Your IP:".bold(), client_ip);
            }
            
            if let Some(isp) = &result.isp {
                println!("{} {}", "ISP:".bold(), isp);
            }
        }
        
        // Test information
        println!("\n{}", " ℹ️  TEST INFORMATION ℹ️  ".on_bright_blue().white().bold());
        println!("{}", "═════════════════════════".bright_blue());
        println!("{} {}", "Test Duration:".bold(), format!("{:.2} seconds", result.test_duration_seconds));
        println!("{} {}", "Timestamp:".bold(), result.timestamp.format("%Y-%m-%d %H:%M:%S UTC"));
        
        // Recommendations based on the results
        self.show_recommendations(result)?;
        
        Ok(())
    }
    
    fn show_recommendations(&self, result: &SpeedTestResult) -> io::Result<()> {
        println!("\n{}", " 💡 RECOMMENDATIONS 💡 ".on_bright_blue().white().bold());
        println!("{}", "═════════════════════════".bright_blue());
        
        match result.quality {
            ConnectionQuality::Excellent => {
                println!("🎮 {}", "Your connection is excellent for gaming, 4K streaming, and large file transfers.".green());
            },
            ConnectionQuality::Good => {
                println!("📺 {}", "Your connection is good for HD streaming, video calls, and most online activities.".bright_green());
            },
            ConnectionQuality::Average => {
                println!("📱 {}", "Your connection is average and suitable for standard definition streaming and general browsing.".yellow());
                println!("💡 {}", "Consider closing background applications during important video calls or downloads.".yellow());
            },
            ConnectionQuality::Poor => {
                println!("⚠️ {}", "Your connection is poor and may struggle with video streaming or large downloads.".bright_red());
                println!("💡 {}", "Try moving closer to your router or using a wired connection.".bright_red());
                println!("💡 {}", "Consider contacting your ISP if this performance is below your plan's advertised speeds.".bright_red());
            },
            ConnectionQuality::VeryPoor | ConnectionQuality::Failed => {
                println!("❌ {}", "Your connection is very poor or failed the test.".red());
                println!("💡 {}", "Check if your router is functioning properly and restart it if necessary.".red());
                println!("💡 {}", "Try using a wired connection instead of Wi-Fi.".red());
                println!("💡 {}", "Contact your ISP to report the issue and check for outages in your area.".red());
            },
        }
        
        Ok(())
    }
    
    pub fn show_progress(&self, progress: f64, total: f64, message: &str) -> io::Result<()> {
        let percentage = (progress / total * 100.0) as u64;
        let width = 50;
        let filled = (width as f64 * (percentage as f64 / 100.0)) as usize;
        
        print!("\r{} [", message);
        for i in 0..width {
            if i < filled {
                print!("█");
            } else {
                print!(" ");
            }
        }
        print!("] {}%", percentage);
        io::stdout().flush()?;
        
        if percentage >= 100 {
            println!();
        }
        
        Ok(())
    }
    
    pub fn show_loading_animation(&self, message: &str, duration_ms: u64) -> io::Result<()> {
        let mut spinner = Spinner::new(Spinners::Dots12, message.to_string());
        std::thread::sleep(Duration::from_millis(duration_ms));
        spinner.stop();
        println!();
        Ok(())
    }
    
    pub fn show_error(&self, message: &str) -> io::Result<()> {
        println!("{} {}", "ERROR:".bright_red().bold(), message.bright_red());
        Ok(())
    }
    
    pub fn show_success(&self, message: &str) -> io::Result<()> {
        println!("{} {}", "SUCCESS:".bright_green().bold(), message.bright_green());
        Ok(())
    }
    
    pub fn show_warning(&self, message: &str) -> io::Result<()> {
        println!("{} {}", "WARNING:".yellow().bold(), message.yellow());
        Ok(())
    }
    
    pub fn show_info(&self, message: &str) -> io::Result<()> {
        println!("{} {}", "INFO:".bright_blue().bold(), message.bright_blue());
        Ok(())
    }
}