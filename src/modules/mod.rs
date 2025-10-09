pub mod diagnostics;
pub mod history;
pub mod intro;
pub mod logo;
pub mod speed_test;
pub mod types;
pub mod ui;

// Re-export common types for easier access
// These are public API exports used by external consumers
#[allow(unused_imports)]
pub use intro::{show_intro, show_simple_intro};
pub use logo::{NetrunnerLogo, NetrunnerLogoSize};
#[allow(unused_imports)]
pub use types::{ConnectionQuality, DetailLevel, SpeedTestResult, TestConfig};

// Re-export storage and speed test as primary
#[allow(unused_imports)]
pub use history::{HistoryStorage, SpeedTrends, TestStatistics};
#[allow(unused_imports)]
pub use speed_test::{GeoLocation, SpeedTest};
