use super::Translations;

pub struct EnglishTranslations;

impl Translations for EnglishTranslations {
    fn get(&self, key: &str) -> String {
        match key {
            // Application
            "app.name" => "Claude Code Monitor",
            "app.version" => "Claude Code Network Monitor v1.0",
            
            // Network status
            "network.title" => "🌐 Network Connection Status",
            "network.status" => "Status",
            "network.connected" => "Connected",
            "network.disconnected" => "Disconnected",
            "network.detecting" => "Detecting...",
            "network.speed" => "🚀 Speed",
            "network.latency" => "Latency",
            "network.excellent" => "Excellent",
            "network.good" => "Good",
            "network.fair" => "Fair",
            "network.slow" => "Slow",
            
            // Usage status
            "usage.title" => "🤖 Claude Code Usage Status",
            "usage.session_start" => "📅 Session Start",
            "usage.time" => "⏱️  Time",
            "usage.remaining" => "⏰ Remaining",
            "usage.cost" => "💰 Cost",
            "usage.model" => "🤖 Model",
            "usage.status" => "📍 Status",
            "usage.active" => "⚡ Active",
            "usage.completed" => "✅ Completed",
            "usage.running" => "🔄 Running",
            "usage.inactive" => "⏸️  Inactive",
            "usage.expired" => "Expired",
            "usage.reset" => "Reset",
            
            // Historical statistics
            "history.title" => "📊 Historical Billing Statistics (Token Based)",
            "history.total" => "💳 Total",
            "history.average" => "📊 Average",
            "history.sessions_total" => "🔢 Total Sessions",
            "history.sessions_active" => "⚡ Active Sessions",
            
            // Tray menu
            "tray.network" => "Network",
            "tray.usage" => "Usage",
            "tray.cost" => "Cost",
            "tray.model" => "Model",
            "tray.remaining" => "Remaining Time",
            "tray.status" => "Status",
            "tray.quit" => "Quit",
            "tray.checking" => "Checking...",
            "tray.network_connected" => "🟢 Network: Connected",
            "tray.network_disconnected" => "🔴 Network: Disconnected",
            "tray.status_active" => "✅ Status: Active",
            "tray.status_completed" => "⏸️ Status: Completed",
            
            // Notifications
            "notification.title" => "Claude Code Monitor",
            "notification.network_restored" => "🎉 Network Connection Restored",
            "notification.network_lost" => "🚨 Network Connection Lost",
            "notification.usage_status" => "Claude Code Usage Status",
            
            // Common
            "common.unknown" => "Unknown",
            "common.hour" => "h",
            "common.minute" => "m",
            "common.day" => "day",
            "common.last_update" => "🕐 Last Update",
            "common.press_to_quit" => "Press Cmd+Q to Stop Monitoring",
            
            // Model Pricing
            "model_pricing.comparison" => "📊 Model Pricing Comparison",
            
            _ => key, // Return key if translation not found
        }.to_string()
    }
}