pub mod types;
pub mod ui;
pub mod speed_test;
pub mod diagnostics;
pub mod history;

// Re-export common types for easier access
pub use types::{SpeedTestResult, TestConfig, ConnectionQuality, DetailLevel};