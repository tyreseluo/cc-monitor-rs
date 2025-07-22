use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::i18n::Language;

#[derive(Debug, Serialize, Deserialize)]
pub struct Preferences {
    pub language: String,
}

impl Default for Preferences {
    fn default() -> Self {
        // Detect system language for default
        let default_lang = detect_system_language();
        Self {
            language: default_lang.code().to_string(),
        }
    }
}

fn detect_system_language() -> Language {
    // Try to get system language from environment variables
    let lang = std::env::var("LANG")
        .or_else(|_| std::env::var("LANGUAGE"))
        .or_else(|_| std::env::var("LC_ALL"))
        .or_else(|_| std::env::var("LC_MESSAGES"))
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

impl Preferences {
    pub fn load() -> Result<Self> {
        let path = Self::get_preferences_path()?;
        
        if !path.exists() {
            // Create default preferences file
            let prefs = Self::default();
            prefs.save()?;
            return Ok(prefs);
        }
        
        let content = fs::read_to_string(&path)?;
        let prefs: Preferences = serde_json::from_str(&content)?;
        Ok(prefs)
    }
    
    pub fn save(&self) -> Result<()> {
        let path = Self::get_preferences_path()?;
        
        // Create directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&path, content)?;
        Ok(())
    }
    
    fn get_preferences_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
        
        let app_dir = config_dir.join("cc-monitor-rs");
        Ok(app_dir.join("preferences.json"))
    }
    
    pub fn get_language(&self) -> Language {
        match self.language.as_str() {
            "en" => Language::English,
            "zh" => Language::Chinese,
            "ja" => Language::Japanese,
            _ => Language::Chinese, // Default to Chinese
        }
    }
    
    pub fn set_language(&mut self, lang: Language) {
        self.language = lang.code().to_string();
    }
}