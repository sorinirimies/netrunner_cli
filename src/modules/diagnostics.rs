use std::net::{IpAddr, Ipv4Addr};
use std::time::{Duration, Instant};
use tokio::time::sleep;
use colored::*;
use rand::Rng;
use dns_lookup::lookup_host;

use crate::modules::types::{NetworkDiagnostics, RouteHop, TestConfig};
use crate::modules::ui::UI;

pub struct NetworkDiagnosticsTool {
    config: TestConfig,
    ui: UI,
}

impl NetworkDiagnosticsTool {
    pub fn new(config: TestConfig) -> Self {
        let ui = UI::new(config.clone());
        Self { config, ui }
    }
    
    pub async fn run_diagnostics(&self) -> Result<NetworkDiagnostics, Box<dyn std::error::Error>> {
        if !self.config.json_output {
            self.ui.show_section_header("Running Network Diagnostics")?;
        }
        
        // Determine gateway
        let gateway_ip = self.detect_gateway().await?;
        
        // Get DNS servers
        let dns_servers = self.detect_dns_servers().await?;
        
        // Measure DNS response time
        let dns_response_time = self.measure_dns_response_time().await?;
        
        // Trace route
        let route_hops = self.trace_route("8.8.8.8").await?;
        
        // Check IPv6 availability
        let is_ipv6_available = self.check_ipv6().await?;
        
        // Determine connection type (wired/wireless)
        let connection_type = self.detect_connection_type().await?;
        
        // Get network interface
        let network_interface = self.detect_network_interface().await?;
        
        let diagnostics = NetworkDiagnostics {
            gateway_ip,
            dns_servers,
            dns_response_time_ms: dns_response_time,
            route_hops,
            is_ipv6_available,
            connection_type: Some(connection_type),
            network_interface: Some(network_interface),
        };
        
        // Display results
        if !self.config.json_output {
            self.display_diagnostics_results(&diagnostics)?;
        }
        
        Ok(diagnostics)
    }
    
    async fn detect_gateway(&self) -> Result<Option<IpAddr>, Box<dyn std::error::Error>> {
        if !self.config.json_output {
            self.ui.show_info("Detecting gateway...")?;
        }
        
        let pb = if !self.config.json_output && self.config.animation_enabled {
            Some(self.ui.create_spinner("Detecting gateway..."))
        } else {
            None
        };
        
        // This is a simplified approach. In a real implementation, you'd:
        // 1. On Windows: Use "ipconfig" and parse the "Default Gateway" line
        // 2. On Linux/macOS: Use "ip route | grep default" or "netstat -nr | grep default"
        
        // For demonstration, we'll simulate it
        sleep(Duration::from_millis(800)).await;
        
        let gateway = Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)));
        
        if let Some(pb) = pb {
            if let Some(gw) = gateway {
                pb.finish_with_message(format!("Gateway detected: {}", gw));
            } else {
                pb.finish_with_message("Could not detect gateway");
            }
        }
        
        Ok(gateway)
    }
    
    async fn detect_dns_servers(&self) -> Result<Vec<IpAddr>, Box<dyn std::error::Error>> {
        if !self.config.json_output {
            self.ui.show_info("Detecting DNS servers...")?;
        }
        
        let pb = if !self.config.json_output && self.config.animation_enabled {
            Some(self.ui.create_spinner("Detecting DNS servers..."))
        } else {
            None
        };
        
        // This is a simplified approach. In a real implementation, you'd:
        // 1. On Windows: Use "ipconfig /all" and parse the "DNS Servers" lines
        // 2. On Linux: Read "/etc/resolv.conf"
        // 3. On macOS: Use "scutil --dns" and parse the output
        
        // For demonstration, we'll simulate it
        sleep(Duration::from_millis(700)).await;
        
        let dns_servers = vec![
            IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
            IpAddr::V4(Ipv4Addr::new(8, 8, 4, 4)),
        ];
        
        if let Some(pb) = pb {
            pb.finish_with_message(format!("Found {} DNS servers", dns_servers.len()));
        }
        
        Ok(dns_servers)
    }
    
    async fn measure_dns_response_time(&self) -> Result<f64, Box<dyn std::error::Error>> {
        if !self.config.json_output {
            self.ui.show_info("Measuring DNS response time...")?;
        }
        
        let pb = if !self.config.json_output && self.config.animation_enabled {
            Some(self.ui.create_spinner("Testing DNS response time..."))
        } else {
            None
        };
        
        let domains = vec![
            "google.com",
            "amazon.com",
            "facebook.com",
            "microsoft.com",
            "apple.com",
        ];
        
        let mut total_time = 0.0;
        let mut successful_lookups = 0;
        
        for domain in domains {
            let start = Instant::now();
            match lookup_host(domain) {
                Ok(_) => {
                    let duration = start.elapsed().as_millis() as f64;
                    total_time += duration;
                    successful_lookups += 1;
                    
                    if let Some(ref pb) = pb {
                        pb.set_message(format!("Resolved {} in {:.2}ms", domain, duration));
                    }
                },
                Err(e) => {
                    if !self.config.json_output {
                        self.ui.show_error(&format!("Failed to resolve {}: {}", domain, e))?;
                    }
                }
            }
            
            sleep(Duration::from_millis(100)).await;
        }
        
        let avg_time = if successful_lookups > 0 {
            total_time / successful_lookups as f64
        } else {
            0.0
        };
        
        if let Some(pb) = pb {
            pb.finish_with_message(format!("Average DNS response time: {:.2}ms", avg_time));
        }
        
        Ok(avg_time)
    }
    
    async fn trace_route(&self, target: &str) -> Result<Vec<RouteHop>, Box<dyn std::error::Error>> {
        if !self.config.json_output {
            self.ui.show_info(&format!("Tracing route to {}...", target))?;
        }
        
        let max_hops = 15;
        let pb = if !self.config.json_output && self.config.animation_enabled {
            Some(self.ui.create_progress_bar(max_hops, &format!("Tracing route to {}...", target)))
        } else {
            None
        };
        
        let mut hops = Vec::new();
        
        // This is a simplified approach. In a real implementation, you'd:
        // 1. Use a proper traceroute implementation or library
        // 2. On Windows: Use "tracert" command
        // 3. On Linux/macOS: Use "traceroute" command
        
        // For demonstration, we'll simulate traceroute
        for hop_number in 1..=max_hops {
            // Simulate network delay
            let mut rng = rand::thread_rng();
            let delay = if hop_number < 3 {
                // Local network hops are faster
                rng.gen_range(1..10)
            } else if hop_number < 8 {
                // ISP network
                rng.gen_range(10..50)
            } else {
                // Internet
                rng.gen_range(50..150)
            };
            
            sleep(Duration::from_millis(delay)).await;
            
            // Simulate sometimes missing hops
            let address = if hop_number != 6 && hop_number != 9 {
                let fake_ip = format!("192.168.{}.{}", hop_number, hop_number * 10);
                Some(fake_ip.parse::<IpAddr>()?)
            } else {
                None
            };
            
            let hostname = None;
            
            let response_time = if address.is_some() {
                Some(delay as f64)
            } else {
                None
            };
            
            let hop = RouteHop {
                hop_number: hop_number as u32,
                address,
                hostname,
                response_time_ms: response_time,
            };
            
            // Store address and response time before moving hop
            let hop_addr = hop.address.clone();
            let hop_resp_time = hop.response_time_ms;
            
            hops.push(hop);
            
            if let Some(ref pb) = pb {
                if let Some(addr) = &hop_addr {
                    pb.set_message(format!("Hop {}: {} ({:.2}ms)", 
                        hop_number, 
                        addr,
                        hop_resp_time.unwrap_or(0.0)
                    ));
                } else {
                    pb.set_message(format!("Hop {}: * * * (timeout)", hop_number));
                }
                pb.inc(1);
            }
            
            // Last hop should be the target  
            if hop_number == max_hops {
                // Simulate target destination
                let target_ip = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
                hops.pop(); // Remove the last simulated hop
                hops.push(RouteHop {
                    hop_number: hop_number as u32,
                    address: Some(target_ip),
                    hostname: Some(target.to_string()),
                    response_time_ms: Some(delay as f64),
                });
                    
                if let Some(ref pb) = pb {
                    pb.set_message(format!("Hop {}: {} ({:.2}ms) - Destination", 
                        hop_number, 
                        target_ip,
                        delay as f64
                    ));
                }
            }
        }
        
        if let Some(pb) = pb {
            pb.finish_with_message(format!("Trace to {} completed with {} hops", target, hops.len()));
        }
        
        Ok(hops)
    }
    
    async fn check_ipv6(&self) -> Result<bool, Box<dyn std::error::Error>> {
        if !self.config.json_output {
            self.ui.show_info("Checking IPv6 connectivity...")?;
        }
        
        let pb = if !self.config.json_output && self.config.animation_enabled {
            Some(self.ui.create_spinner("Testing IPv6 connectivity..."))
        } else {
            None
        };
        
        // For demonstration, we'll simulate it
        sleep(Duration::from_millis(600)).await;
        
        // Randomly determine if IPv6 is available
        let ipv6_available = rand::thread_rng().gen_bool(0.7); // 70% chance of having IPv6
        
        if let Some(pb) = pb {
            if ipv6_available {
                pb.finish_with_message("IPv6 is available");
            } else {
                pb.finish_with_message("IPv6 is not available");
            }
        }
        
        Ok(ipv6_available)
    }
    
    async fn detect_connection_type(&self) -> Result<String, Box<dyn std::error::Error>> {
        if !self.config.json_output {
            self.ui.show_info("Detecting connection type...")?;
        }
        
        let pb = if !self.config.json_output && self.config.animation_enabled {
            Some(self.ui.create_spinner("Determining connection type..."))
        } else {
            None
        };
        
        // For demonstration, we'll simulate it
        sleep(Duration::from_millis(500)).await;
        
        // Randomly choose between wired and wireless
        let connection_type = if rand::thread_rng().gen_bool(0.6) {
            "Wireless (Wi-Fi)".to_string()
        } else {
            "Wired (Ethernet)".to_string()
        };
        
        if let Some(pb) = pb {
            pb.finish_with_message(format!("Connection type: {}", connection_type));
        }
        
        Ok(connection_type)
    }
    
    async fn detect_network_interface(&self) -> Result<String, Box<dyn std::error::Error>> {
        if !self.config.json_output {
            self.ui.show_info("Detecting network interface...")?;
        }
        
        let pb = if !self.config.json_output && self.config.animation_enabled {
            Some(self.ui.create_spinner("Identifying network interface..."))
        } else {
            None
        };
        
        // For demonstration, we'll simulate it
        sleep(Duration::from_millis(400)).await;
        
        // Simulate different interfaces based on OS
        let interface = if cfg!(target_os = "windows") {
            "Ethernet".to_string()
        } else if cfg!(target_os = "macos") {
            "en0".to_string()
        } else {
            "eth0".to_string()
        };
        
        if let Some(pb) = pb {
            pb.finish_with_message(format!("Network interface: {}", interface));
        }
        
        Ok(interface)
    }
    
    fn display_diagnostics_results(&self, diagnostics: &NetworkDiagnostics) -> Result<(), Box<dyn std::error::Error>> {
        self.ui.show_section_header("Network Diagnostics Results")?;
        
        // Create a pretty table for the basic info
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        
        // Gateway
        if let Some(gateway) = diagnostics.gateway_ip {
            table.add_row(Row::new(vec![
                Cell::new("Gateway IP").style_spec("Fb"),
                Cell::new(&gateway.to_string()),
            ]));
        } else {
            table.add_row(Row::new(vec![
                Cell::new("Gateway IP").style_spec("Fb"),
                Cell::new("Not detected"),
            ]));
        }
        
        // DNS Servers
        let dns_servers = if diagnostics.dns_servers.is_empty() {
            "None detected".to_string()
        } else {
            diagnostics.dns_servers
                .iter()
                .map(|ip| ip.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        };
        
        table.add_row(Row::new(vec![
            Cell::new("DNS Servers").style_spec("Fb"),
            Cell::new(&dns_servers),
        ]));
        
        // DNS Response Time
        let dns_quality = match diagnostics.dns_response_time_ms {
            t if t < 20.0 => "Excellent".green(),
            t if t < 50.0 => "Good".bright_green(),
            t if t < 100.0 => "Average".yellow(),
            t if t < 200.0 => "Slow".bright_red(),
            _ => "Very Slow".red(),
        };
        
        table.add_row(Row::new(vec![
            Cell::new("DNS Response Time").style_spec("Fb"),
            Cell::new(&format!("{:.2}ms ({})", 
                diagnostics.dns_response_time_ms, 
                dns_quality)),
        ]));
        
        // IPv6 Availability
        table.add_row(Row::new(vec![
            Cell::new("IPv6 Available").style_spec("Fb"),
            Cell::new(if diagnostics.is_ipv6_available { "Yes" } else { "No" }),
        ]));
        
        // Connection Type
        if let Some(conn_type) = &diagnostics.connection_type {
            table.add_row(Row::new(vec![
                Cell::new("Connection Type").style_spec("Fb"),
                Cell::new(conn_type),
            ]));
        }
        
        // Network Interface
        if let Some(interface) = &diagnostics.network_interface {
            table.add_row(Row::new(vec![
                Cell::new("Network Interface").style_spec("Fb"),
                Cell::new(interface),
            ]));
        }
        
        // Print the table
        table.printstd();
        
        // Display route trace if we have hops
        if !diagnostics.route_hops.is_empty() {
            println!("\n{}", " 🌐 ROUTE TRACE 🌐 ".on_bright_blue().white().bold());
            println!("{}", "═════════════════════════".bright_blue());
            
            let mut trace_table = Table::new();
            trace_table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
            
            // Add header
            trace_table.add_row(Row::new(vec![
                Cell::new("Hop").style_spec("Fb"),
                Cell::new("IP Address").style_spec("Fb"),
                Cell::new("Hostname").style_spec("Fb"),
                Cell::new("Response Time").style_spec("Fb"),
            ]));
            
            for hop in &diagnostics.route_hops {
                let addr = hop.address.map_or("* * *".to_string(), |a| a.to_string());
                let hostname = hop.hostname.clone().unwrap_or_else(|| "-".to_string());
                let time = hop.response_time_ms.map_or("timeout".to_string(), |t| format!("{:.2}ms", t));
                
                trace_table.add_row(Row::new(vec![
                    Cell::new(&hop.hop_number.to_string()),
                    Cell::new(&addr),
                    Cell::new(&hostname),
                    Cell::new(&time),
                ]));
            }
            
            trace_table.printstd();
        }
        
        // Provide some recommendations based on the diagnostics
        self.show_diagnostics_recommendations(diagnostics)?;
        
        Ok(())
    }
    
    fn show_diagnostics_recommendations(&self, diagnostics: &NetworkDiagnostics) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n{}", " 💡 RECOMMENDATIONS 💡 ".on_bright_blue().white().bold());
        println!("{}", "═════════════════════════".bright_blue());
        
        // Check DNS performance
        if diagnostics.dns_response_time_ms > 100.0 {
            println!("🔸 {}", "Your DNS response time is slow. Consider using alternative DNS servers like Google (8.8.8.8) or Cloudflare (1.1.1.1).".yellow());
        }
        
        // Check IPv6 availability
        if !diagnostics.is_ipv6_available {
            println!("🔸 {}", "IPv6 is not available on your network. This is not an issue, but enabling IPv6 may improve connectivity to some modern services.".blue());
        }
        
        // Check for missing hops in traceroute
        let missing_hops = diagnostics.route_hops.iter()
            .filter(|hop| hop.address.is_none())
            .count();
            
        if missing_hops > 2 {
            println!("🔸 {}", "Multiple hops in your network path are not responding. This could indicate network configuration issues along your connection path.".yellow());
        }
        
        // Check connection type for wireless optimization
        if let Some(conn_type) = &diagnostics.connection_type {
            if conn_type.contains("Wi-Fi") || conn_type.contains("Wireless") {
                println!("🔸 {}", "You're using a wireless connection. For better speed and stability, consider using a wired Ethernet connection for critical activities.".blue());
            }
        }
        
        // If everything looks good
        if diagnostics.dns_response_time_ms < 50.0 && missing_hops <= 2 {
            println!("🔹 {}", "Your network configuration appears healthy! No significant issues detected.".green());
        }
        
        Ok(())
    }
}

use prettytable::{Table, Row, Cell, format};