use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

mod en;
mod zh;
mod ja;

pub use self::en::EnglishTranslations;
pub use self::zh::ChineseTranslations;
pub use self::ja::JapaneseTranslations;

use crate::utils::preferences::Preferences;
use std::env;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    English,
    Chinese,
    Japanese,
}

impl Language {
    pub fn code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Chinese => "zh",
            Language::Japanese => "ja",
        }
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Chinese => "中文",
            Language::Japanese => "日本語",
        }
    }
}

fn detect_system_language() -> Language {
    // Try to get system language from environment variables
    let lang = env::var("LANG")
        .or_else(|_| env::var("LANGUAGE"))
        .or_else(|_| env::var("LC_ALL"))
        .or_else(|_| env::var("LC_MESSAGES"))
        .unwrap_or_else(|_| String::from("en_US"));
    
    // Parse the language code
    let lang_code = lang.split('.').next().unwrap_or(&lang);
    let lang_code = lang_code.split('_').next().unwrap_or(lang_code);
    
    match lang_code.to_lowercase().as_str() {
        "zh" | "zh-cn" | "zh-hans" => Language::Chinese,
        "ja" | "jp" => Language::Japanese,
        "en" | _ => Language::English, // Default to English for unknown languages
    }
}

pub trait Translations {
    fn get(&self, key: &str) -> String;
    
    fn format(&self, key: &str, args: &[&str]) -> String {
        let template = self.get(key);
        let mut result = template;
        
        for (i, arg) in args.iter().enumerate() {
            let placeholder = format!("{{{}}}", i);
            result = result.replace(&placeholder, arg);
        }
        
        result
    }
}

// Global language state
static CURRENT_LANGUAGE: Lazy<Arc<Mutex<Language>>> = Lazy::new(|| {
    // Load language from preferences
    let lang = if let Ok(prefs) = Preferences::load() {
        prefs.get_language()
    } else {
        // Default to system language
        detect_system_language()
    };
    Arc::new(Mutex::new(lang))
});

static TRANSLATIONS: Lazy<Arc<Mutex<Box<dyn Translations + Send>>>> = Lazy::new(|| {
    let lang = *CURRENT_LANGUAGE.lock().unwrap();
    let translations: Box<dyn Translations + Send> = match lang {
        Language::English => Box::new(EnglishTranslations),
        Language::Chinese => Box::new(ChineseTranslations),
        Language::Japanese => Box::new(JapaneseTranslations),
    };
    Arc::new(Mutex::new(translations))
});

pub fn set_language(lang: Language) {
    let mut current = CURRENT_LANGUAGE.lock().unwrap();
    *current = lang;
    
    let mut translations = TRANSLATIONS.lock().unwrap();
    *translations = match lang {
        Language::English => Box::new(EnglishTranslations),
        Language::Chinese => Box::new(ChineseTranslations),
        Language::Japanese => Box::new(JapaneseTranslations),
    };
    
    // Save preference
    if let Ok(mut prefs) = Preferences::load() {
        prefs.set_language(lang);
        let _ = prefs.save();
    }
}

pub fn get_language() -> Language {
    *CURRENT_LANGUAGE.lock().unwrap()
}

pub fn get(key: &str) -> String {
    TRANSLATIONS.lock().unwrap().get(key)
}

pub fn format(key: &str, args: &[&str]) -> String {
    TRANSLATIONS.lock().unwrap().format(key, args)
}

// Translation keys
pub mod keys {
    // Application
    pub const APP_NAME: &str = "app.name";
    pub const APP_VERSION: &str = "app.version";
    
    // Network status
    pub const NETWORK_TITLE: &str = "network.title";
    pub const NETWORK_STATUS: &str = "network.status";
    pub const NETWORK_CONNECTED: &str = "network.connected";
    pub const NETWORK_DISCONNECTED: &str = "network.disconnected";
    pub const NETWORK_DETECTING: &str = "network.detecting";
    pub const NETWORK_SPEED: &str = "network.speed";
    pub const NETWORK_LATENCY: &str = "network.latency";
    pub const NETWORK_EXCELLENT: &str = "network.excellent";
    pub const NETWORK_GOOD: &str = "network.good";
    pub const NETWORK_FAIR: &str = "network.fair";
    pub const NETWORK_SLOW: &str = "network.slow";
    
    // Usage status
    pub const USAGE_TITLE: &str = "usage.title";
    pub const USAGE_SESSION_START: &str = "usage.session_start";
    pub const USAGE_TIME: &str = "usage.time";
    pub const USAGE_REMAINING: &str = "usage.remaining";
    pub const USAGE_COST: &str = "usage.cost";
    pub const USAGE_MODEL: &str = "usage.model";
    pub const USAGE_STATUS: &str = "usage.status";
    pub const USAGE_ACTIVE: &str = "usage.active";
    pub const USAGE_COMPLETED: &str = "usage.completed";
    pub const USAGE_RUNNING: &str = "usage.running";
    pub const USAGE_INACTIVE: &str = "usage.inactive";
    pub const USAGE_EXPIRED: &str = "usage.expired";
    pub const USAGE_RESET: &str = "usage.reset";
    
    // Historical statistics
    pub const HISTORY_TITLE: &str = "history.title";
    pub const HISTORY_TOTAL: &str = "history.total";
    pub const HISTORY_AVERAGE: &str = "history.average";
    pub const HISTORY_SESSIONS_TOTAL: &str = "history.sessions_total";
    pub const HISTORY_SESSIONS_ACTIVE: &str = "history.sessions_active";
    
    // Tray menu
    pub const TRAY_NETWORK: &str = "tray.network";
    pub const TRAY_USAGE: &str = "tray.usage";
    pub const TRAY_COST: &str = "tray.cost";
    pub const TRAY_MODEL: &str = "tray.model";
    pub const TRAY_REMAINING: &str = "tray.remaining";
    pub const TRAY_STATUS: &str = "tray.status";
    pub const TRAY_QUIT: &str = "tray.quit";
    pub const TRAY_CHECKING: &str = "tray.checking";
    pub const TRAY_NETWORK_CONNECTED: &str = "tray.network_connected";
    pub const TRAY_NETWORK_DISCONNECTED: &str = "tray.network_disconnected";
    pub const TRAY_STATUS_ACTIVE: &str = "tray.status_active";
    pub const TRAY_STATUS_COMPLETED: &str = "tray.status_completed";
    
    // Notifications
    pub const NOTIF_TITLE: &str = "notification.title";
    pub const NOTIF_NETWORK_RESTORED: &str = "notification.network_restored";
    pub const NOTIF_NETWORK_LOST: &str = "notification.network_lost";
    pub const NOTIF_USAGE_STATUS: &str = "notification.usage_status";
    
    // Common
    pub const COMMON_UNKNOWN: &str = "common.unknown";
    pub const COMMON_HOUR: &str = "common.hour";
    pub const COMMON_MINUTE: &str = "common.minute";
    pub const COMMON_DAY: &str = "common.day";
    pub const COMMON_LAST_UPDATE: &str = "common.last_update";
    pub const COMMON_PRESS_TO_QUIT: &str = "common.press_to_quit";
    
    // Model Pricing
    pub const MODEL_PRICING_COMPARISON: &str = "model_pricing.comparison";
}