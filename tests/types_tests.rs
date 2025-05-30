use netrunner_cli::modules::types::*;
use chrono::Utc;
use std::net::{IpAddr, Ipv4Addr};

#[test]
fn test_connection_quality_from_speed_and_ping() {
    // Test Excellent quality
    let quality = ConnectionQuality::from_speed_and_ping(150.0, 25.0, 15.0);
    assert_eq!(quality, ConnectionQuality::Excellent);
    
    // Test Good quality
    let quality = ConnectionQuality::from_speed_and_ping(60.0, 12.0, 40.0);
    assert_eq!(quality, ConnectionQuality::Good);
    
    // Test Average quality
    let quality = ConnectionQuality::from_speed_and_ping(30.0, 7.0, 90.0);
    assert_eq!(quality, ConnectionQuality::Average);
    
    // Test Poor quality
    let quality = ConnectionQuality::from_speed_and_ping(12.0, 3.0, 140.0);
    assert_eq!(quality, ConnectionQuality::Poor);
    
    // Test Very Poor quality
    let quality = ConnectionQuality::from_speed_and_ping(8.0, 1.5, 200.0);
    assert_eq!(quality, ConnectionQuality::VeryPoor);
    
    // Test Failed quality
    let quality = ConnectionQuality::from_speed_and_ping(0.0, 0.0, 0.0);
    assert_eq!(quality, ConnectionQuality::Failed);
}



#[test]
fn test_speed_test_result_default() {
    let result = SpeedTestResult::default();
    
    assert_eq!(result.download_mbps, 0.0);
    assert_eq!(result.upload_mbps, 0.0);
    assert_eq!(result.ping_ms, 0.0);
    assert_eq!(result.jitter_ms, 0.0);
    assert_eq!(result.packet_loss_percent, 0.0);
    assert_eq!(result.server_location, "Unknown");
    assert_eq!(result.server_ip, None);
    assert_eq!(result.client_ip, None);
    assert_eq!(result.quality, ConnectionQuality::Failed);
    assert_eq!(result.test_duration_seconds, 0.0);
    assert_eq!(result.isp, None);
    
    // Check that timestamp is recent (within last second)
    let now = Utc::now();
    let diff = now.signed_duration_since(result.timestamp);
    assert!(diff.num_seconds() < 1);
}

#[test]
fn test_speed_test_result_with_values() {
    let test_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1));
    let timestamp = Utc::now();
    
    let result = SpeedTestResult {
        timestamp,
        download_mbps: 75.5,
        upload_mbps: 15.2,
        ping_ms: 25.8,
        jitter_ms: 3.1,
        packet_loss_percent: 0.5,
        server_location: "Test Server".to_string(),
        server_ip: Some(test_ip),
        client_ip: Some(test_ip),
        quality: ConnectionQuality::Good,
        test_duration_seconds: 12.5,
        isp: Some("Test ISP".to_string()),
    };
    
    assert_eq!(result.download_mbps, 75.5);
    assert_eq!(result.upload_mbps, 15.2);
    assert_eq!(result.ping_ms, 25.8);
    assert_eq!(result.jitter_ms, 3.1);
    assert_eq!(result.packet_loss_percent, 0.5);
    assert_eq!(result.server_location, "Test Server");
    assert_eq!(result.server_ip, Some(test_ip));
    assert_eq!(result.client_ip, Some(test_ip));
    assert_eq!(result.quality, ConnectionQuality::Good);
    assert_eq!(result.test_duration_seconds, 12.5);
    assert_eq!(result.isp, Some("Test ISP".to_string()));
    assert_eq!(result.timestamp, timestamp);
}

#[test]
fn test_test_server_creation() {
    let server = TestServer {
        name: "Test Server".to_string(),
        url: "https://test.example.com".to_string(),
        location: "Test Location".to_string(),
        distance_km: Some(150.5),
        latency_ms: Some(25.0),
    };
    
    assert_eq!(server.name, "Test Server");
    assert_eq!(server.url, "https://test.example.com");
    assert_eq!(server.location, "Test Location");
    assert_eq!(server.distance_km, Some(150.5));
    assert_eq!(server.latency_ms, Some(25.0));
}

#[test]
fn test_test_config_default() {
    let config = TestConfig::default();
    
    assert_eq!(config.server_url, "https://httpbin.org");
    assert_eq!(config.test_size_mb, 10);
    assert_eq!(config.timeout_seconds, 30);
    assert_eq!(config.json_output, false);
    assert_eq!(config.animation_enabled, true);
    assert_eq!(config.detail_level, DetailLevel::Standard);
    assert_eq!(config.max_servers, 3);
}

#[test]
fn test_test_config_custom() {
    let config = TestConfig {
        server_url: "https://custom.server.com".to_string(),
        test_size_mb: 50,
        timeout_seconds: 60,
        json_output: true,
        animation_enabled: false,
        detail_level: DetailLevel::Detailed,
        max_servers: 5,
    };
    
    assert_eq!(config.server_url, "https://custom.server.com");
    assert_eq!(config.test_size_mb, 50);
    assert_eq!(config.timeout_seconds, 60);
    assert_eq!(config.json_output, true);
    assert_eq!(config.animation_enabled, false);
    assert_eq!(config.detail_level, DetailLevel::Detailed);
    assert_eq!(config.max_servers, 5);
}

#[test]
fn test_detail_level_ordering() {
    // Test that DetailLevel implements PartialOrd correctly
    assert!(DetailLevel::Basic < DetailLevel::Standard);
    assert!(DetailLevel::Standard < DetailLevel::Detailed);
    assert!(DetailLevel::Detailed < DetailLevel::Debug);
    
    assert!(DetailLevel::Debug > DetailLevel::Detailed);
    assert!(DetailLevel::Detailed > DetailLevel::Standard);
    assert!(DetailLevel::Standard > DetailLevel::Basic);
}

#[test]
fn test_connection_quality_display() {
    assert_eq!(format!("{}", ConnectionQuality::Excellent), "Excellent");
    assert_eq!(format!("{}", ConnectionQuality::Good), "Good");
    assert_eq!(format!("{}", ConnectionQuality::Average), "Average");
    assert_eq!(format!("{}", ConnectionQuality::Poor), "Poor");
    assert_eq!(format!("{}", ConnectionQuality::VeryPoor), "Very Poor");
    assert_eq!(format!("{}", ConnectionQuality::Failed), "Failed");
}

#[test]
fn test_detail_level_display() {
    assert_eq!(format!("{}", DetailLevel::Basic), "Basic");
    assert_eq!(format!("{}", DetailLevel::Standard), "Standard");
    assert_eq!(format!("{}", DetailLevel::Detailed), "Detailed");
    assert_eq!(format!("{}", DetailLevel::Debug), "Debug");
}

#[test]
fn test_route_hop_creation() {
    let hop = RouteHop {
        hop_number: 5,
        address: Some(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1))),
        hostname: Some("gateway.example.com".to_string()),
        response_time_ms: Some(15.5),
    };
    
    assert_eq!(hop.hop_number, 5);
    assert_eq!(hop.address, Some(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1))));
    assert_eq!(hop.hostname, Some("gateway.example.com".to_string()));
    assert_eq!(hop.response_time_ms, Some(15.5));
}

#[test]
fn test_network_diagnostics_creation() {
    let diagnostics = NetworkDiagnostics {
        gateway_ip: Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))),
        dns_servers: vec![
            IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
            IpAddr::V4(Ipv4Addr::new(8, 8, 4, 4)),
        ],
        dns_response_time_ms: 25.0,
        route_hops: vec![],
        is_ipv6_available: true,
        connection_type: Some("Ethernet".to_string()),
        network_interface: Some("eth0".to_string()),
    };
    
    assert_eq!(diagnostics.gateway_ip, Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))));
    assert_eq!(diagnostics.dns_servers.len(), 2);
    assert_eq!(diagnostics.dns_response_time_ms, 25.0);
    assert_eq!(diagnostics.is_ipv6_available, true);
    assert_eq!(diagnostics.connection_type, Some("Ethernet".to_string()));
    assert_eq!(diagnostics.network_interface, Some("eth0".to_string()));
}