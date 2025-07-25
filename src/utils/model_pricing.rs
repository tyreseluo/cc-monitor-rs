use std::fs;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use once_cell::sync::Lazy;
use std::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPricing {
    pub name: String,
    pub display_name: String,
    pub input_price_per_million: f64,
    pub output_price_per_million: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPricingConfig {
    pub models: Vec<ModelPricing>,
}

static MODEL_PRICING: Lazy<RwLock<ModelPricingConfig>> = Lazy::new(|| {
    RwLock::new(ModelPricingConfig::load_default())
});

impl ModelPricingConfig {
    pub fn load() -> Result<Self> {
        // Try to load from user config first
        if let Ok(user_config) = Self::load_user_config() {
            return Ok(user_config);
        }
        
        // Fall back to default config
        Ok(Self::load_default())
    }
    
    fn load_user_config() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
        
        let config_path = config_dir.join("cc-monitor-rs").join("model_pricing.json");
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let config: ModelPricingConfig = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            Err(anyhow::anyhow!("User config not found"))
        }
    }
    
    fn load_default() -> Self {
        // Load from embedded resource file
        let default_config = include_str!("../../resources/model_pricing.json");
        serde_json::from_str(default_config).unwrap_or_else(|_| {
            // Fallback if parsing fails
            ModelPricingConfig {
                models: vec![
                    ModelPricing {
                        name: "claude-3-5-sonnet-20241022".to_string(),
                        display_name: "Claude Sonnet 3.5".to_string(),
                        input_price_per_million: 3.0,
                        output_price_per_million: 15.0,
                    },
                    ModelPricing {
                        name: "gpt-4o".to_string(),
                        display_name: "GPT-4o".to_string(),
                        input_price_per_million: 5.0,
                        output_price_per_million: 15.0,
                    },
                    ModelPricing {
                        name: "qwen-coder".to_string(),
                        display_name: "Qwen Coder".to_string(),
                        input_price_per_million: 0.28,
                        output_price_per_million: 0.56,
                    },
                ],
            }
        })
    }
    
    pub fn save_user_config(&self) -> Result<()> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
        
        let app_dir = config_dir.join("cc-monitor-rs");
        fs::create_dir_all(&app_dir)?;
        
        let config_path = app_dir.join("model_pricing.json");
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
        Ok(())
    }
    
    pub fn get_model_pricing(&self, model_name: &str) -> Option<&ModelPricing> {
        self.models.iter().find(|m| m.name == model_name)
    }
    
    pub fn calculate_cost(&self, model_name: &str, input_tokens: u64, output_tokens: u64) -> Option<f64> {
        self.get_model_pricing(model_name).map(|pricing| {
            let input_cost = (input_tokens as f64 / 1_000_000.0) * pricing.input_price_per_million;
            let output_cost = (output_tokens as f64 / 1_000_000.0) * pricing.output_price_per_million;
            input_cost + output_cost
        })
    }
}

// Global functions for easy access
pub fn get_model_pricing_config() -> ModelPricingConfig {
    MODEL_PRICING.read().unwrap().clone()
}

pub fn reload_model_pricing() -> Result<()> {
    let new_config = ModelPricingConfig::load()?;
    *MODEL_PRICING.write().unwrap() = new_config;
    Ok(())
}

pub fn calculate_all_model_costs(input_tokens: u64, output_tokens: u64) -> Vec<(String, String, f64)> {
    let config = get_model_pricing_config();
    config.models.iter().map(|model| {
        let cost = config.calculate_cost(&model.name, input_tokens, output_tokens).unwrap_or(0.0);
        (model.name.clone(), model.display_name.clone(), cost)
    }).collect()
}