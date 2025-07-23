use super::Translations;

pub struct JapaneseTranslations;

impl Translations for JapaneseTranslations {
    fn get(&self, key: &str) -> String {
        match key {
            // Application
            "app.name" => "Claude Code モニター",
            "app.version" => "Claude Code ネットワークモニター v1.0",
            
            // Network status
            "network.title" => "🌐 ネットワーク接続状態",
            "network.status" => "状態",
            "network.connected" => "接続済み",
            "network.disconnected" => "未接続",
            "network.detecting" => "検出中...",
            "network.speed" => "🚀 速度",
            "network.latency" => "遅延",
            "network.excellent" => "優秀",
            "network.good" => "良好",
            "network.fair" => "普通",
            "network.slow" => "遅い",
            
            // Usage status
            "usage.title" => "🤖 Claude Code 使用状況",
            "usage.session_start" => "📅 セッション開始",
            "usage.time" => "⏱️  時間",
            "usage.remaining" => "⏰ 残り",
            "usage.cost" => "💰 コスト",
            "usage.model" => "🤖 モデル",
            "usage.status" => "📍 ステータス",
            "usage.active" => "⚡ アクティブ",
            "usage.completed" => "✅ 完了",
            "usage.running" => "🔄 実行中",
            "usage.inactive" => "⏸️  非アクティブ",
            "usage.expired" => "期限切れ",
            "usage.reset" => "リセット",
            
            // Historical statistics
            "history.title" => "📊 履歴請求統計（トークンベース）",
            "history.total" => "💳 合計",
            "history.average" => "📊 平均",
            "history.sessions_total" => "🔢 総セッション数",
            "history.sessions_active" => "⚡ アクティブセッション",
            
            // Tray menu
            "tray.network" => "ネットワーク",
            "tray.usage" => "使用量",
            "tray.cost" => "コスト",
            "tray.model" => "モデル",
            "tray.remaining" => "残り時間",
            "tray.status" => "ステータス",
            "tray.quit" => "終了",
            "tray.checking" => "確認中...",
            "tray.network_connected" => "🟢 ネットワーク: 接続済み",
            "tray.network_disconnected" => "🔴 ネットワーク: 未接続",
            "tray.status_active" => "✅ ステータス: アクティブ",
            "tray.status_completed" => "⏸️ ステータス: 完了",
            
            // Notifications
            "notification.title" => "Claude Code モニター",
            "notification.network_restored" => "🎉 ネットワーク接続が復元されました",
            "notification.network_lost" => "🚨 ネットワーク接続が切断されました",
            "notification.usage_status" => "Claude Code 使用状況",
            
            // Common
            "common.unknown" => "不明",
            "common.hour" => "時間",
            "common.minute" => "分",
            "common.day" => "日",
            "common.last_update" => "🕐 最終更新",
            "common.press_to_quit" => "Cmd+Q を押して監視を停止",
            
            _ => key, // Return key if translation not found
        }.to_string()
    }
}