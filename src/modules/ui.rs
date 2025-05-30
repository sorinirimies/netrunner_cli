use colored::*;
use console::Term;
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use std::io;
use std::time::Duration;

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
}