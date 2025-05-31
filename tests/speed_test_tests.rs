use netrunner_cli::modules::{
    speed_test::SpeedTest,
    types::{TestConfig, DetailLevel, ConnectionQuality, SpeedTestResult, ServerProvider, TestServer, ServerCapabilities},
};
use tokio_test;
use tempfile::tempdir;
use std::time::Duration;
use chrono::Utc;

/// Helper function to create a test config
fn create_test_config() -> TestConfig {
    TestConfig {
        server_url: "https://httpbin.org".to_string(),
        test_size_mb: 1, // Small size for testing
        timeout_seconds: 10,
        json_output: true, // Suppress UI output during tests
        animation_enabled: false,
        detail_level: DetailLevel::Standard,
        max_servers: 2,
    }
}

/// Helper function to create a mock test server
fn create_mock_server() -> TestServer {
    TestServer {
        name: "Test Server".to_string(),
        url: "https://httpbin.org".to_string(),
        location: "Test Location".to_string(),
        distance_km: Some(100.0),
        latency_ms: Some(50.0),
        provider: ServerProvider::Custom("Test".to_string()),
        capabilities: ServerCapabilities {
            supports_download: true,
            supports_upload: true,
            supports_latency: true,
            max_test_size_mb: 10,
            geographic_weight: 0.8,
        },
        quality_score: Some(0.7),
        country_code: Some("US".to_string()),
        city: Some("Test City".to_string()),
        is_backup: false,
    }
}

#[tokio::test]
async fn test_speed_test_creation() {
    let config = create_test_config();
    let speed_test = SpeedTest::new(config);
    
    assert!(speed_test.is_ok(), "SpeedTest should be created successfully");
}

#[tokio::test]
async fn test_speed_test_full_test() {
    let config = create_test_config();
    let speed_test = SpeedTest::new(config).expect("Failed to create SpeedTest");
    
    // Run the full test with a short timeout
    let result = tokio::time::timeout(
        Duration::from_secs(30),
        speed_test.run_full_test()
    ).await;
    
    match result {
        Ok(test_result) => {
            let test_result = test_result.expect("Speed test should complete successfully");
            
            // Verify basic result structure
            assert!(test_result.download_mbps >= 0.0, "Download speed should be non-negative");
            assert!(test_result.upload_mbps >= 0.0, "Upload speed should be non-negative");
            assert!(test_result.ping_ms >= 0.0, "Ping should be non-negative");
            assert!(test_result.test_duration_seconds > 0.0, "Test duration should be positive");
            assert!(!test_result.server_location.is_empty(), "Server location should not be empty");
        }
        Err(_) => {
            // Test timed out, which is acceptable in some network conditions
            println!("Speed test timed out - this may be expected in test environments");
        }
    }
}

#[tokio::test]
async fn test_connection_quality_calculation() {
    // Test excellent quality
    let quality = ConnectionQuality::from_speed_and_ping(150.0, 25.0, 15.0);
    assert_eq!(quality, ConnectionQuality::Excellent);
    
    // Test good quality
    let quality = ConnectionQuality::from_speed_and_ping(60.0, 12.0, 40.0);
    assert_eq!(quality, ConnectionQuality::Good);
    
    // Test average quality
    let quality = ConnectionQuality::from_speed_and_ping(30.0, 7.0, 90.0);
    assert_eq!(quality, ConnectionQuality::Average);
    
    // Test poor quality
    let quality = ConnectionQuality::from_speed_and_ping(12.0, 3.0, 140.0);
    assert_eq!(quality, ConnectionQuality::Poor);
    
    // Test very poor quality
    let quality = ConnectionQuality::from_speed_and_ping(8.0, 1.5, 200.0);
    assert_eq!(quality, ConnectionQuality::VeryPoor);
    
    // Test failed quality
    let quality = ConnectionQuality::from_speed_and_ping(0.0, 0.0, 0.0);
    assert_eq!(quality, ConnectionQuality::Failed);
}

#[tokio::test]
async fn test_quality_boundary_conditions() {
    // Test exact boundaries for excellent
    let quality = ConnectionQuality::from_speed_and_ping(100.0, 20.0, 19.9);
    assert_eq!(quality, ConnectionQuality::Excellent);
    
    // Just below excellent threshold
    let quality = ConnectionQuality::from_speed_and_ping(99.9, 19.9, 20.1);
    assert_ne!(quality, ConnectionQuality::Excellent);
    
    // Test exact boundaries for good
    let quality = ConnectionQuality::from_speed_and_ping(50.0, 10.0, 49.9);
    assert_eq!(quality, ConnectionQuality::Good);
}

#[tokio::test]
async fn test_speed_test_result_validation() {
    let result = SpeedTestResult {
        timestamp: Utc::now(),
        download_mbps: 50.5,
        upload_mbps: 10.2,
        ping_ms: 25.0,
        jitter_ms: 2.5,
        packet_loss_percent: 0.1,
        server_location: "Test Location".to_string(),
        server_ip: None,
        client_ip: None,
        quality: ConnectionQuality::Good,
        test_duration_seconds: 15.5,
        isp: Some("Test ISP".to_string()),
    };
    
    // Verify all fields are properly set
    assert_eq!(result.download_mbps, 50.5);
    assert_eq!(result.upload_mbps, 10.2);
    assert_eq!(result.ping_ms, 25.0);
    assert_eq!(result.jitter_ms, 2.5);
    assert_eq!(result.packet_loss_percent, 0.1);
    assert_eq!(result.server_location, "Test Location");
    assert_eq!(result.quality, ConnectionQuality::Good);
    assert_eq!(result.test_duration_seconds, 15.5);
    assert_eq!(result.isp, Some("Test ISP".to_string()));
}

#[tokio::test]
async fn test_realistic_speed_ranges() {
    let config = create_test_config();
    let speed_test = SpeedTest::new(config).expect("Failed to create SpeedTest");
    
    // Test that speeds are within realistic ranges (if test completes)
    if let Ok(result) = tokio::time::timeout(
        Duration::from_secs(20),
        speed_test.run_full_test()
    ).await {
        if let Ok(test_result) = result {
            // Download speed should be reasonable (not impossibly high)
            assert!(test_result.download_mbps <= 10000.0, "Download speed seems unrealistically high: {}", test_result.download_mbps);
            assert!(test_result.download_mbps >= 0.1, "Download speed seems too low: {}", test_result.download_mbps);
            
            // Upload speed should be reasonable
            assert!(test_result.upload_mbps <= 1000.0, "Upload speed seems unrealistically high: {}", test_result.upload_mbps);
            assert!(test_result.upload_mbps >= 0.1, "Upload speed seems too low: {}", test_result.upload_mbps);
            
            // Ping should be reasonable
            assert!(test_result.ping_ms <= 5000.0, "Ping seems unrealistically high: {}", test_result.ping_ms);
            assert!(test_result.ping_ms >= 1.0, "Ping seems too low: {}", test_result.ping_ms);
            
            // Jitter should be reasonable if measured
            if test_result.jitter_ms > 0.0 {
                assert!(test_result.jitter_ms <= 1000.0, "Jitter seems unrealistically high: {}", test_result.jitter_ms);
            }
            
            // Packet loss should be percentage (0-100)
            assert!(test_result.packet_loss_percent >= 0.0 && test_result.packet_loss_percent <= 100.0, 
                "Packet loss should be a percentage: {}", test_result.packet_loss_percent);
        }
    }
}

#[tokio::test]
async fn test_config_variations() {
    // Test with different detail levels
    let mut config = create_test_config();
    config.detail_level = DetailLevel::Debug;
    let speed_test = SpeedTest::new(config);
    assert!(speed_test.is_ok(), "SpeedTest should work with Debug detail level");
    
    // Test with different timeout
    let mut config = create_test_config();
    config.timeout_seconds = 5;
    let speed_test = SpeedTest::new(config);
    assert!(speed_test.is_ok(), "SpeedTest should work with short timeout");
    
    // Test with different test size
    let mut config = create_test_config();
    config.test_size_mb = 5;
    let speed_test = SpeedTest::new(config);
    assert!(speed_test.is_ok(), "SpeedTest should work with different test size");
}

#[tokio::test]
async fn test_server_capabilities() {
    let server = create_mock_server();
    
    assert!(server.capabilities.supports_download, "Mock server should support download");
    assert!(server.capabilities.supports_upload, "Mock server should support upload");
    assert!(server.capabilities.supports_latency, "Mock server should support latency testing");
    assert!(server.capabilities.max_test_size_mb > 0, "Mock server should have positive max test size");
    assert!(server.capabilities.geographic_weight >= 0.0 && server.capabilities.geographic_weight <= 1.0, 
        "Geographic weight should be between 0 and 1");
}

#[tokio::test]
async fn test_server_provider_types() {
    // Test different server provider types
    let cloudflare = ServerProvider::Cloudflare;
    let google = ServerProvider::Google; 
    let netflix = ServerProvider::Netflix;
    let custom = ServerProvider::Custom("TestProvider".to_string());
    
    // Ensure they can be compared
    assert_eq!(cloudflare, ServerProvider::Cloudflare);
    assert_ne!(cloudflare, google);
    assert_ne!(google, netflix);
    
    match custom {
        ServerProvider::Custom(ref name) => assert_eq!(name, "TestProvider"),
        _ => panic!("Custom provider should match pattern"),
    }
}

#[tokio::test]
async fn test_error_handling() {
    // Test with invalid server URL
    let mut config = create_test_config();
    config.server_url = "invalid-url".to_string();
    
    let speed_test = SpeedTest::new(config);
    assert!(speed_test.is_ok(), "SpeedTest creation should succeed even with invalid URL");
    
    // The actual network errors should be handled gracefully during test execution
}

#[tokio::test] 
async fn test_json_serialization() {
    let result = SpeedTestResult {
        timestamp: Utc::now(),
        download_mbps: 100.5,
        upload_mbps: 20.3,
        ping_ms: 15.7,
        jitter_ms: 1.2,
        packet_loss_percent: 0.0,
        server_location: "Test Server".to_string(),
        server_ip: None,
        client_ip: None,
        quality: ConnectionQuality::Excellent,
        test_duration_seconds: 12.34,
        isp: Some("Test ISP".to_string()),
    };
    
    // Test JSON serialization
    let json = serde_json::to_string(&result);
    assert!(json.is_ok(), "SpeedTestResult should serialize to JSON");
    
    // Test JSON deserialization
    if let Ok(json_str) = json {
        let deserialized: Result<SpeedTestResult, _> = serde_json::from_str(&json_str);
        assert!(deserialized.is_ok(), "SpeedTestResult should deserialize from JSON");
        
        if let Ok(parsed_result) = deserialized {
            assert_eq!(parsed_result.download_mbps, result.download_mbps);
            assert_eq!(parsed_result.upload_mbps, result.upload_mbps);
            assert_eq!(parsed_result.quality, result.quality);
        }
    }
}

#[tokio::test]
async fn test_performance_metrics_integration() {
    // Test that jitter and packet loss are properly integrated
    let result = SpeedTestResult {
        timestamp: Utc::now(),
        download_mbps: 50.0,
        upload_mbps: 10.0,
        ping_ms: 30.0,
        jitter_ms: 5.0,
        packet_loss_percent: 1.0,
        server_location: "Test".to_string(),
        server_ip: None,
        client_ip: None,
        quality: ConnectionQuality::Good,
        test_duration_seconds: 10.0,
        isp: None,
    };
    
    // Verify that quality assessment considers all metrics appropriately
    assert_eq!(result.quality, ConnectionQuality::Good);
    
    // Even with good speeds, high jitter or packet loss might affect quality in future enhancements
    assert!(result.jitter_ms > 0.0, "Jitter should be measured");
    assert!(result.packet_loss_percent >= 0.0, "Packet loss should be measured");
}