use colored::*;
use console::Term;
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use std::io::{self, Write};
use std::time::Duration;
use std::thread;
use rand::Rng;

use crate::modules::types::TestConfig;

pub struct UI {
    term: Term,
    multi_progress: MultiProgress,
}

impl UI {
    pub fn new(_config: TestConfig) -> Self {
        Self {
            term: Term::stdout(),
            multi_progress: MultiProgress::new(),
        }
    }

    pub fn clear_screen(&self) -> io::Result<()> {
        self.term.clear_screen()
    }

    pub fn show_welcome_banner(&self) -> io::Result<()> {
        self.term.clear_screen()?;
        
        let banner = r#"
 _   _ ______ _______ _____  _    _ _   _ _   _ ______ _____  
| \ | |  ____|__   __|  __ \| |  | | \ | | \ | |  ____|  __ \ 
|  \| | |__     | |  | |__) | |  | |  \| |  \| | |__  | |__) |
| . ` |  __|    | |  |  _  /| |  | | . ` | . ` |  __| |  _  / 
| |\  | |____   | |  | | \ \| |__| | |\  | |\  | |____| | \ \ 
|_| \_|______|  |_|  |_|  \_\\____/|_| \_|_| \_|______|_|  \_\
                                                              
        "#;

        println!("{}", banner.bright_cyan());
        
        // Cyberpunk-style status messages with glitch effects
        println!("{}", "┌─ SYSTEM STATUS ─────────────────────────────────────────┐".bright_magenta());
        println!("{}", "│ ⟨⟨⟨ NEURAL INTERFACE: ONLINE ⟩⟩⟩                        │".bright_green());
        println!("{}", "│ ⟨⟨⟨ NETWORK SCANNER: INITIALIZED ⟩⟩⟩                   │".bright_green());
        println!("{}", "│ ⟨⟨⟨ QUANTUM DIAGNOSTICS: READY ⟩⟩⟩                     │".bright_green());
        println!("{}", "└─────────────────────────────────────────────────────────┘".bright_magenta());
        println!();
        println!("{}", ">>> JACK IN AND ANALYZE YOUR DIGITAL HIGHWAY <<<".bright_yellow().bold());
        println!("{}", ">>> DATA FLOWS | PACKET STREAMS | NEURAL PATHS <<<".bright_blue());
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

    pub fn create_speed_test_spinner(&self, message: &str) -> ProgressBar {
        let pb = self.multi_progress.add(ProgressBar::new_spinner());
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("🌐 {spinner:.bright_cyan} {msg}")
                .unwrap()
                .tick_strings(&[
                    "▓▓▓▓▓▓▓▓▓▓",
                    "▒▓▓▓▓▓▓▓▓▓",
                    "░▒▓▓▓▓▓▓▓▓",
                    " ░▒▓▓▓▓▓▓▓",
                    "  ░▒▓▓▓▓▓▓",
                    "   ░▒▓▓▓▓▓",
                    "    ░▒▓▓▓▓",
                    "     ░▒▓▓▓",
                    "      ░▒▓▓",
                    "       ░▒▓",
                    "        ░▒",
                    "         ░",
                    "          ",
                    " ░        ",
                    " ▒░       ",
                    " ▓▒░      ",
                    " █▓▒░     ",
                    " █▓▒░    ",
                    " █▓▒░   ",
                    " █▓▒░  ",
                    " █▓▒░ ",
                    " █▓▒░",
                ])
        );
        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(120));
        pb
    }

    pub fn create_pacman_spinner(&self, message: &str) -> ProgressBar {
        let pb = self.multi_progress.add(ProgressBar::new_spinner());
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("🎮 {spinner:.bright_yellow} {msg}")
                .unwrap()
                .tick_strings(&[
                    "ᗧ ••••••••••",
                    "ᗤ  •••••••••",
                    "ᗤ   ••••••••",
                    "ᗤ    •••••••",
                    "ᗤ     ••••••",
                    "ᗤ      •••••",
                    "ᗤ       ••••",
                    "ᗤ        •••",
                    "ᗤ         ••",
                    "ᗤ          •",
                    "ᗤ           ",
                    "ᗧ           ",
                    " ᗧ          ",
                    "  ᗧ         ",
                    "   ᗧ        ",
                    "    ᗧ       ",
                    "     ᗧ      ",
                    "      ᗧ     ",
                    "       ᗧ    ",
                    "        ᗧ   ",
                    "         ᗧ  ",
                    "          ᗧ ",
                    "           ᗧ",
                ])
        );
        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(150));
        pb
    }

    pub fn create_download_spinner(&self, message: &str) -> ProgressBar {
        let pb = self.multi_progress.add(ProgressBar::new_spinner());
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("📥 {spinner:.bright_blue} {msg}")
                .unwrap()
                .tick_strings(&[
                    "⬇️       ",
                    " ⬇️      ",
                    "  ⬇️     ",
                    "   ⬇️    ",
                    "    ⬇️   ",
                    "     ⬇️  ",
                    "      ⬇️ ",
                    "       ⬇️",
                    "        ⬇️",
                    "⬇️ ⬇️ ⬇️ ⬇️",
                    "💾 Data  ",
                    "💾 Captured",
                ])
        );
        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(200));
        pb
    }

    pub fn create_upload_spinner(&self, message: &str) -> ProgressBar {
        let pb = self.multi_progress.add(ProgressBar::new_spinner());
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("📤 {spinner:.bright_magenta} {msg}")
                .unwrap()
                .tick_strings(&[
                    "⬆️       ",
                    " ⬆️      ",
                    "  ⬆️     ",
                    "   ⬆️    ",
                    "    ⬆️   ",
                    "     ⬆️  ",
                    "      ⬆️ ",
                    "       ⬆️",
                    "        ⬆️",
                    "⬆️ ⬆️ ⬆️ ⬆️",
                    "🚀 Sent  ",
                    "🚀 Complete",
                ])
        );
        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(180));
        pb
    }

    pub fn create_ping_spinner(&self, message: &str) -> ProgressBar {
        let pb = self.multi_progress.add(ProgressBar::new_spinner());
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("🏓 {spinner:.bright_green} {msg}")
                .unwrap()
                .tick_strings(&[
                    "🏓      📍",
                    " 🏓     📍",
                    "  🏓    📍",
                    "   🏓   📍",
                    "    🏓  📍",
                    "     🏓 📍",
                    "      🏓📍",
                    "       ⚡",
                    "      📍🏓",
                    "     📍 🏓",
                    "    📍  🏓",
                    "   📍   🏓",
                    "  📍    🏓",
                    " 📍     🏓",
                    "📍      🏓",
                ])
        );
        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(100));
        pb
    }

    pub fn create_dna_helix_spinner(&self, message: &str) -> ProgressBar {
        let pb = self.multi_progress.add(ProgressBar::new_spinner());
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("🧬 {spinner:.bright_green} {msg}")
                .unwrap()
                .tick_strings(&[
                    "  ╭─╮  ",
                    "  │ │  ",
                    "  ╰─╯  ",
                    " ╱   ╲ ",
                    "╱     ╲",
                    "╲     ╱",
                    " ╲   ╱ ",
                    "  ╲ ╱  ",
                    "   ╳   ",
                    "  ╱ ╲  ",
                    " ╱   ╲ ",
                    "╱     ╲",
                ])
        );
        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(200));
        pb
    }

    pub fn create_rocket_spinner(&self, message: &str) -> ProgressBar {
        let pb = self.multi_progress.add(ProgressBar::new_spinner());
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("🚀 {spinner:.bright_yellow} {msg}")
                .unwrap()
                .tick_strings(&[
                    "🚀      ",
                    " 🚀     ",
                    "  🚀    ",
                    "   🚀   ",
                    "    🚀  ",
                    "     🚀 ",
                    "      🚀",
                    "       🌟",
                    "      🌟 ",
                    "     🌟  ",
                    "    🌟   ",
                    "   🌟    ",
                    "  🌟     ",
                    " 🌟      ",
                    "🌟       ",
                ])
        );
        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(120));
        pb
    }

    pub fn create_wave_spinner(&self, message: &str) -> ProgressBar {
        let pb = self.multi_progress.add(ProgressBar::new_spinner());
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("🌊 {spinner:.bright_cyan} {msg}")
                .unwrap()
                .tick_strings(&[
                    "▁▁▁▁▁▁▁▁",
                    "▁▁▁▂▁▁▁▁",
                    "▁▁▂▃▂▁▁▁",
                    "▁▂▃▄▃▂▁▁",
                    "▂▃▄▅▄▃▂▁",
                    "▃▄▅▆▅▄▃▂",
                    "▄▅▆▇▆▅▄▃",
                    "▅▆▇██▇▆▅",
                    "▆▇██▇▆▅▄",
                    "▇██▇▆▅▄▃",
                    "██▇▆▅▄▃▂",
                    "█▇▆▅▄▃▂▁",
                    "▇▆▅▄▃▂▁▁",
                    "▆▅▄▃▂▁▁▁",
                    "▅▄▃▂▁▁▁▁",
                    "▄▃▂▁▁▁▁▁",
                ])
        );
        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(100));
        pb
    }

    pub fn create_network_scanner_bar(&self, message: &str) -> ProgressBar {
        let pb = self.multi_progress.add(ProgressBar::new_spinner());
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("🔍 {spinner:.bright_yellow} {msg}")
                .unwrap()
                .tick_strings(&[
                    "⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏",
                    "⢎⡰", "⢎⡡", "⢎⡑", "⢎⠱", "⠎⡱", "⢊⡱", "⢌⡱", "⢆⡱"
                ])
        );
        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(120));
        pb
    }

    pub fn create_cyberpunk_spinner(&self, message: &str) -> ProgressBar {
        let pb = self.multi_progress.add(ProgressBar::new_spinner());
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("⟨⟨⟨ {spinner:.bright_cyan} {msg} ⟩⟩⟩")
                .unwrap()
                .tick_strings(&[
                    "▰▱▱▱▱▱▱",
                    "▰▰▱▱▱▱▱",
                    "▰▰▰▱▱▱▱",
                    "▰▰▰▰▱▱▱",
                    "▰▰▰▰▰▱▱",
                    "▰▰▰▰▰▰▱",
                    "▰▰▰▰▰▰▰",
                    "▱▰▰▰▰▰▰",
                    "▱▱▰▰▰▰▰",
                    "▱▱▱▰▰▰▰",
                    "▱▱▱▱▰▰▰",
                    "▱▱▱▱▱▰▰",
                    "▱▱▱▱▱▱▰",
                    "▱▱▱▱▱▱▱"
                ])
        );
        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(150));
        pb
    }

    pub fn create_spinner(&self, message: &str) -> ProgressBar {
        let pb = self.multi_progress.add(ProgressBar::new_spinner());
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("🔄 {spinner:.bright_green} {msg}")
                .unwrap()
                .tick_strings(&[
                    "⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"
                ])
        );
        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(100));
        pb
    }



    pub fn show_section_header(&self, title: &str) -> io::Result<()> {
        println!();
        println!("{}", format!("▓▓▓ {} ▓▓▓", title.to_uppercase()).bright_magenta().bold());
        println!("{}", "╔══════════════════════════════════════════════════════════╗".bright_cyan());
        println!("{}", format!("║ >>> {} INITIATED <<<", title.to_uppercase()).bright_green());
        println!("{}", "╚══════════════════════════════════════════════════════════╝".bright_cyan());
        Ok(())
    }

    pub fn show_error(&self, message: &str) -> io::Result<()> {
        println!("{} {}", "ERROR:".bright_red().bold(), message.bright_red());
        Ok(())
    }
    
    pub fn show_info(&self, message: &str) -> io::Result<()> {
        println!("{} {}", "INFO:".bright_blue().bold(), message.bright_blue());
        Ok(())
    }

    pub fn show_animated_text(&self, text: &str, delay_ms: u64) -> io::Result<()> {
        for char in text.chars() {
            print!("{}", char.to_string().bright_cyan());
            std::io::stdout().flush()?;
            thread::sleep(Duration::from_millis(delay_ms));
        }
        println!();
        Ok(())
    }

    pub fn show_typing_effect(&self, text: &str) -> io::Result<()> {
        for char in text.chars() {
            print!("{}", char.to_string().bright_green());
            std::io::stdout().flush()?;
            thread::sleep(Duration::from_millis(50));
        }
        println!();
        Ok(())
    }

    pub fn show_matrix_effect(&self, lines: usize) -> io::Result<()> {
        let matrix_chars = ["0", "1", "⠁", "⠂", "⠄", "⡀", "⢀", "⠠", "⠐", "⠈"];
        
        for _ in 0..lines {
            print!("{}", "█".bright_green());
            for _ in 0..60 {
                let idx = rand::random::<usize>() % matrix_chars.len();
                print!("{}", matrix_chars[idx].bright_green());
                thread::sleep(Duration::from_millis(20));
            }
            println!();
        }
        Ok(())
    }

    pub fn show_pulse_text(&self, text: &str, pulses: usize) -> io::Result<()> {
        for _ in 0..pulses {
            print!("\r{}", text.bright_cyan().bold());
            std::io::stdout().flush()?;
            thread::sleep(Duration::from_millis(500));
            
            print!("\r{}", text.bright_blue());
            std::io::stdout().flush()?;
            thread::sleep(Duration::from_millis(500));
        }
        println!();
        Ok(())
    }

    pub fn show_connection_establishing(&self) -> io::Result<()> {
        let steps = [
            "⟨⟨⟨ INITIALIZING NEURAL INTERFACE ⟩⟩⟩",
            "⟨⟨⟨ SCANNING NETWORK TOPOLOGY ⟩⟩⟩",
            "⟨⟨⟨ ESTABLISHING QUANTUM TUNNEL ⟩⟩⟩",
            "⟨⟨⟨ CALIBRATING DATA STREAMS ⟩⟩⟩",
            "⟨⟨⟨ CONNECTION ESTABLISHED ⟩⟩⟩"
        ];

        for step in steps.iter() {
            println!("{}", step.bright_magenta());
            thread::sleep(Duration::from_millis(800));
        }
        println!();
        Ok(())
    }
}