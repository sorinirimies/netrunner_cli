use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Result as SqliteResult};
use std::fs;

use crate::modules::types::SpeedTestResult;

pub struct HistoryStorage {
    db_path: PathBuf,
}

impl HistoryStorage {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
        let data_dir = home_dir.join(".netrunner");
        
        // Create the data directory if it doesn't exist
        if !data_dir.exists() {
            fs::create_dir_all(&data_dir)?;
        }
        
        let db_path = data_dir.join("history.db");
        let storage = Self { db_path };
        
        // Initialize the database
        storage.init_db()?;
        
        Ok(storage)
    }
    
    fn init_db(&self) -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::open(&self.db_path)?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS speed_tests (
                id INTEGER PRIMARY KEY,
                timestamp TEXT NOT NULL,
                download_mbps REAL NOT NULL,
                upload_mbps REAL NOT NULL,
                ping_ms REAL NOT NULL,
                jitter_ms REAL NOT NULL,
                packet_loss_percent REAL,
                server_location TEXT NOT NULL,
                server_ip TEXT,
                client_ip TEXT,
                quality TEXT NOT NULL,
                test_duration_seconds REAL NOT NULL,
                isp TEXT
            )",
            [],
        )?;
        
        Ok(())
    }
    
    pub fn save_result(&self, result: &SpeedTestResult) -> Result<i64, Box<dyn std::error::Error>> {
        let conn = Connection::open(&self.db_path)?;
        
        conn.execute(
            "INSERT INTO speed_tests (
                timestamp, download_mbps, upload_mbps, ping_ms, jitter_ms,
                packet_loss_percent, server_location, server_ip, client_ip,
                quality, test_duration_seconds, isp
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12
            )",
            params![
                result.timestamp.to_rfc3339(),
                result.download_mbps,
                result.upload_mbps,
                result.ping_ms,
                result.jitter_ms,
                result.packet_loss_percent,
                result.server_location,
                result.server_ip.map(|ip| ip.to_string()),
                result.client_ip.map(|ip| ip.to_string()),
                result.quality.to_string(),
                result.test_duration_seconds,
                result.isp,
            ],
        )?;
        
        Ok(conn.last_insert_rowid())
    }
    
    pub fn get_all_results(&self) -> Result<Vec<SpeedTestResult>, Box<dyn std::error::Error>> {
        let conn = Connection::open(&self.db_path)?;
        
        let mut stmt = conn.prepare(
            "SELECT
                timestamp, download_mbps, upload_mbps, ping_ms, jitter_ms,
                packet_loss_percent, server_location, server_ip, client_ip,
                quality, test_duration_seconds, isp
             FROM speed_tests
             ORDER BY timestamp DESC"
        )?;
        
        let result_iter = stmt.query_map([], |row| {
            let timestamp_str: String = row.get(0)?;
            let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))?
                .with_timezone(&Utc);
                
            let server_ip_str: Option<String> = row.get(7)?;
            let server_ip = server_ip_str.and_then(|s| s.parse().ok());
            
            let client_ip_str: Option<String> = row.get(8)?;
            let client_ip = client_ip_str.and_then(|s| s.parse().ok());
            
            let quality_str: String = row.get(9)?;
            let quality = quality_str.parse()
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(9, rusqlite::types::Type::Text, Box::new(e)))?;
                
            Ok(SpeedTestResult {
                timestamp,
                download_mbps: row.get(1)?,
                upload_mbps: row.get(2)?,
                ping_ms: row.get(3)?,
                jitter_ms: row.get(4)?,
                packet_loss_percent: row.get(5)?,
                server_location: row.get(6)?,
                server_ip,
                client_ip,
                quality,
                test_duration_seconds: row.get(10)?,
                isp: row.get(11)?,
            })
        })?;
        
        let mut results = Vec::new();
        for result in result_iter {
            results.push(result?);
        }
        
        Ok(results)
    }
    
    pub fn get_recent_results(&self, limit: usize) -> Result<Vec<SpeedTestResult>, Box<dyn std::error::Error>> {
        let conn = Connection::open(&self.db_path)?;
        
        let mut stmt = conn.prepare(
            "SELECT
                timestamp, download_mbps, upload_mbps, ping_ms, jitter_ms,
                packet_loss_percent, server_location, server_ip, client_ip,
                quality, test_duration_seconds, isp
             FROM speed_tests
             ORDER BY timestamp DESC
             LIMIT ?"
        )?;
        
        let result_iter = stmt.query_map(params![limit as i64], |row| {
            let timestamp_str: String = row.get(0)?;
            let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))?
                .with_timezone(&Utc);
                
            let server_ip_str: Option<String> = row.get(7)?;
            let server_ip = server_ip_str.and_then(|s| s.parse().ok());
            
            let client_ip_str: Option<String> = row.get(8)?;
            let client_ip = client_ip_str.and_then(|s| s.parse().ok());
            
            let quality_str: String = row.get(9)?;
            let quality = quality_str.parse()
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(9, rusqlite::types::Type::Text, Box::new(e)))?;
                
            Ok(SpeedTestResult {
                timestamp,
                download_mbps: row.get(1)?,
                upload_mbps: row.get(2)?,
                ping_ms: row.get(3)?,
                jitter_ms: row.get(4)?,
                packet_loss_percent: row.get(5)?,
                server_location: row.get(6)?,
                server_ip,
                client_ip,
                quality,
                test_duration_seconds: row.get(10)?,
                isp: row.get(11)?,
            })
        })?;
        
        let mut results = Vec::new();
        for result in result_iter {
            results.push(result?);
        }
        
        Ok(results)
    }
    
    pub fn get_statistics(&self) -> Result<TestStatistics, Box<dyn std::error::Error>> {
        let conn = Connection::open(&self.db_path)?;
        
        let mut stmt = conn.prepare(
            "SELECT
                AVG(download_mbps) as avg_download,
                MAX(download_mbps) as max_download,
                MIN(download_mbps) as min_download,
                AVG(upload_mbps) as avg_upload,
                MAX(upload_mbps) as max_upload,
                MIN(upload_mbps) as min_upload,
                AVG(ping_ms) as avg_ping,
                MIN(ping_ms) as min_ping,
                MAX(ping_ms) as max_ping,
                COUNT(*) as test_count
             FROM speed_tests"
        )?;
        
        let stats = stmt.query_row([], |row| {
            Ok(TestStatistics {
                avg_download_mbps: row.get(0)?,
                max_download_mbps: row.get(1)?,
                min_download_mbps: row.get(2)?,
                avg_upload_mbps: row.get(3)?,
                max_upload_mbps: row.get(4)?,
                min_upload_mbps: row.get(5)?,
                avg_ping_ms: row.get(6)?,
                min_ping_ms: row.get(7)?,
                max_ping_ms: row.get(8)?,
                test_count: row.get(9)?,
            })
        })?;
        
        Ok(stats)
    }
    
    pub fn clear_history(&self) -> Result<(), Box<dyn std::error::Error>> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute("DELETE FROM speed_tests", [])?;
        Ok(())
    }
    
    pub fn get_db_path(&self) -> &Path {
        &self.db_path
    }
}

#[derive(Debug, Clone)]
pub struct TestStatistics {
    pub avg_download_mbps: f64,
    pub max_download_mbps: f64,
    pub min_download_mbps: f64,
    pub avg_upload_mbps: f64,
    pub max_upload_mbps: f64,
    pub min_upload_mbps: f64,
    pub avg_ping_ms: f64,
    pub min_ping_ms: f64,
    pub max_ping_ms: f64,
    pub test_count: i64,
}