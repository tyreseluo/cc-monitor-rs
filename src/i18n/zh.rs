use super::Translations;

pub struct ChineseTranslations;

impl Translations for ChineseTranslations {
    fn get(&self, key: &str) -> String {
        match key {
            // Application
            "app.name" => "Claude Code 监测器",
            "app.version" => "Claude Code 网络监测器 v1.0",
            
            // Network status
            "network.title" => "🌐 网络连接状态",
            "network.status" => "状态",
            "network.connected" => "已连接",
            "network.disconnected" => "未连接",
            "network.detecting" => "检测中...",
            "network.speed" => "🚀 网速",
            "network.latency" => "延迟",
            "network.excellent" => "优秀",
            "network.good" => "良好",
            "network.fair" => "一般",
            "network.slow" => "较慢",
            
            // Usage status
            "usage.title" => "🤖 Claude Code 使用状态",
            "usage.session_start" => "📅 对话开始",
            "usage.time" => "⏱️  时间",
            "usage.remaining" => "⏰ 剩余",
            "usage.cost" => "💰 费用",
            "usage.model" => "🤖 模型",
            "usage.status" => "📍 状态",
            "usage.active" => "⚡ 活跃中",
            "usage.completed" => "✅ 已完成",
            "usage.running" => "🔄 运行中",
            "usage.inactive" => "⏸️  未活动",
            "usage.expired" => "已过期",
            "usage.reset" => "重置",
            
            // Historical statistics
            "history.title" => "📊 历史账单统计 (基于 Token 计算)",
            "history.total" => "💳 总计",
            "history.average" => "📊 平均",
            "history.sessions_total" => "🔢 总会话数",
            "history.sessions_active" => "⚡ 活跃会话",
            
            // Tray menu
            "tray.network" => "网络",
            "tray.usage" => "使用",
            "tray.cost" => "花费",
            "tray.model" => "模型",
            "tray.remaining" => "剩余时间",
            "tray.status" => "状态",
            "tray.quit" => "退出",
            "tray.checking" => "检测中...",
            "tray.network_connected" => "🟢 网络: 已连接",
            "tray.network_disconnected" => "🔴 网络: 未连接",
            "tray.status_active" => "✅ 状态: 活跃中",
            "tray.status_completed" => "⏸️ 状态: 已完成",
            
            // Notifications
            "notification.title" => "Claude Code 监测器",
            "notification.network_restored" => "🎉 网络连接已恢复",
            "notification.network_lost" => "🚨 网络连接中断",
            "notification.usage_status" => "Claude Code 使用状态",
            
            // Common
            "common.unknown" => "未知",
            "common.hour" => "时",
            "common.minute" => "分",
            "common.day" => "天",
            "common.last_update" => "🕐 最后更新",
            "common.press_to_quit" => "按 Cmd+Q 停止监控",
            
            _ => key, // Return key if translation not found
        }.to_string()
    }
}