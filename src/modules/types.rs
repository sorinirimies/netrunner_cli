use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use strum::EnumString;
use strum_macros::Display;

/// Represents the quality rating of a network connection
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Display, EnumString)]
pub enum ConnectionQuality {
    #[strum(to_string = "Excellent")]
    Excellent,
    #[strum(to_string = "Good")]
    Good,
    #[strum(to_string = "Average")]
    Average,
    #[strum(to_string = "Poor")]
    Poor,
    #[strum(to_string = "Very Poor")]
    VeryPoor,
    #[strum(to_string = "Failed")]
    Failed,
}

impl ConnectionQuality {
    pub fn from_speed_and_ping(download_mbps: f64, upload_mbps: f64, ping_ms: f64) -> Self {
        // Simplified rating logic
        if download_mbps >= 100.0 && upload_mbps >= 20.0 && ping_ms < 20.0 {
            ConnectionQuality::Excellent
        } else if download_mbps >= 50.0 && upload_mbps >= 10.0 && ping_ms < 50.0 {
            ConnectionQuality::Good
        } else if download_mbps >= 25.0 && upload_mbps >= 5.0 && ping_ms < 100.0 {
            ConnectionQuality::Average
        } else if download_mbps >= 10.0 && upload_mbps >= 2.0 && ping_ms < 150.0 {
            ConnectionQuality::Poor
        } else if download_mbps > 0.0 && upload_mbps > 0.0 {
            ConnectionQuality::VeryPoor
        } else {
            ConnectionQuality::Failed
        }
    }

    pub fn color_code(&self) -> &str {
        match self {
            ConnectionQuality::Excellent => "bright green",
            ConnectionQuality::Good => "green",
            ConnectionQuality::Average => "yellow",
            ConnectionQuality::Poor => "bright red",
            ConnectionQuality::VeryPoor => "red",
            ConnectionQuality::Failed => "bright black",
        }
    }
}

/// Represents a single network speed test result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedTestResult {
    pub timestamp: DateTime<Utc>,
    pub download_mbps: f64,
    pub upload_mbps: f64,
    pub ping_ms: f64,
    pub jitter_ms: f64,
    pub packet_loss_percent: f64,
    pub server_location: String,
    pub server_ip: Option<IpAddr>,
    pub client_ip: Option<IpAddr>,
    pub quality: ConnectionQuality,
    pub test_duration_seconds: f64,
    pub isp: Option<String>,
}

impl Default for SpeedTestResult {
    fn default() -> Self {
        Self {
            timestamp: Utc::now(),
            download_mbps: 0.0,
            upload_mbps: 0.0,
            ping_ms: 0.0,
            jitter_ms: 0.0,
            packet_loss_percent: 0.0,
            server_location: "Unknown".to_string(),
            server_ip: None,
            client_ip: None,
            quality: ConnectionQuality::Failed,
            test_duration_seconds: 0.0,
            isp: None,
        }
    }
}

/// Represents a test server for speed testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestServer {
    pub name: String,
    pub url: String,
    pub location: String,
    pub distance_km: Option<f64>,
    pub latency_ms: Option<f64>,
}

/// Represents detailed network diagnostics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkDiagnostics {
    pub gateway_ip: Option<IpAddr>,
    pub dns_servers: Vec<IpAddr>,
    pub dns_response_time_ms: f64,
    pub route_hops: Vec<RouteHop>,
    pub is_ipv6_available: bool,
    pub connection_type: Option<String>,
    pub network_interface: Option<String>,
}

/// Represents a single hop in a network route
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteHop {
    pub hop_number: u32,
    pub address: Option<IpAddr>,
    pub hostname: Option<String>,
    pub response_time_ms: Option<f64>,
}

/// Configuration for the speed test
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConfig {
    pub server_url: String,
    pub test_size_mb: u64,
    pub timeout_seconds: u64,
    pub json_output: bool,
    pub animation_enabled: bool,
    pub detail_level: DetailLevel,
    pub max_servers: usize,
}

/// Level of detail for test output
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Display, EnumString)]
pub enum DetailLevel {
    #[strum(to_string = "Basic")]
    Basic,
    #[strum(to_string = "Standard")]
    Standard,
    #[strum(to_string = "Detailed")]
    Detailed,
    #[strum(to_string = "Debug")]
    Debug,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            server_url: "https://httpbin.org".to_string(),
            test_size_mb: 10,
            timeout_seconds: 30,
            json_output: false,
            animation_enabled: true,
            detail_level: DetailLevel::Standard,
            max_servers: 3,
        }
    }
}