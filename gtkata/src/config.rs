use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Configuration file structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConfigFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
}

/// Application configuration
#[derive(Debug, Clone)]
pub struct Config {
    pub base_url: String,
    pub api_token: Option<String>,
}

impl Config {
    /// Load configuration from file and environment
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_file = load_config_file()?;

        // Default base URL
        let mut base_url = String::from("http://localhost:4000/api");

        // Override with environment variable if set
        if let Ok(env_url) = std::env::var("KATANAUTE_API_URL") {
            base_url = env_url;
        } else if let Some(url) = config_file.base_url {
            base_url = url;
        }

        Ok(Self {
            base_url,
            api_token: config_file.api_token,
        })
    }

    /// Save API token to config file
    pub fn save_token(&self, token: String) -> Result<(), Box<dyn std::error::Error>> {
        let config_file = ConfigFile {
            api_token: Some(token),
            base_url: Some(self.base_url.clone()),
        };

        save_config_file(&config_file)
    }

    /// Clear API token from config file
    pub fn clear_token(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_file = ConfigFile {
            api_token: None,
            base_url: Some(self.base_url.clone()),
        };

        save_config_file(&config_file)
    }
}

/// Get the config directory path
fn get_config_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let proj_dirs =
        ProjectDirs::from("", "", "katanaute").ok_or("Failed to determine config directory")?;

    let config_dir = proj_dirs.config_dir();

    // Create directory if it doesn't exist
    fs::create_dir_all(config_dir)?;

    Ok(config_dir.to_path_buf())
}

/// Get the config file path
fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let config_dir = get_config_dir()?;
    Ok(config_dir.join("config.json"))
}

/// Load configuration from file
fn load_config_file() -> Result<ConfigFile, Box<dyn std::error::Error>> {
    let config_path = get_config_path()?;

    // If config doesn't exist, return empty config
    if !config_path.exists() {
        return Ok(ConfigFile::default());
    }

    let contents = fs::read_to_string(config_path)?;
    let config: ConfigFile = serde_json::from_str(&contents)?;

    Ok(config)
}

/// Save configuration to file
fn save_config_file(config: &ConfigFile) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = get_config_path()?;
    let contents = serde_json::to_string_pretty(config)?;

    fs::write(config_path, contents)?;

    Ok(())
}
