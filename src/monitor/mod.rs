pub mod network;
pub mod ccusage;
pub mod stats;

pub use network::{NetworkMonitor, SpeedLevel};
pub use ccusage::{CcusageMonitor, CcusageData};
pub use stats::DailyCost;

use chrono::{DateTime, Local};
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct MonitorData {
    // Network status
    pub network_connected: bool,
    pub latency: Option<String>,
    pub connection_speed: Option<SpeedLevel>,
    
    // Claude usage data
    pub ccusage_data: CcusageData,
    
    // Historical data
    pub daily_costs: HashMap<String, DailyCost>,
    
    // Update timestamp
    pub last_update: DateTime<Local>,
}