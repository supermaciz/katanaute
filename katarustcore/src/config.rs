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
    // Check if XDG_CONFIG_HOME is set for testing
    if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
        let config_dir = PathBuf::from(xdg_config).join("katanaute");
        fs::create_dir_all(&config_dir)?;
        return Ok(config_dir);
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::{Mutex, OnceLock};

    static ENV_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        ENV_MUTEX.get_or_init(|| Mutex::new(())).lock().unwrap()
    }

    #[test]
    fn load_prefers_env_over_file() -> Result<(), Box<dyn std::error::Error>> {
        let _guard = env_lock();
        let temp = tempfile::TempDir::new()?;
        unsafe { std::env::set_var("XDG_CONFIG_HOME", temp.path()) };

        // Write config file with a different URL
        let config_dir = temp.path().join("katanaute");
        fs::create_dir_all(&config_dir)?;
        let config_file = config_dir.join("config.json");
        let config_content = serde_json::json!({
            "base_url": "http://from-file"
        });
        fs::write(&config_file, serde_json::to_string_pretty(&config_content)?)?;

        // Set env var to override
        unsafe { std::env::set_var("KATANAUTE_API_URL", "http://from-env") };

        let cfg = Config::load()?;
        assert_eq!(cfg.base_url, "http://from-env");

        // Clean up env vars
        unsafe { std::env::remove_var("XDG_CONFIG_HOME") };
        unsafe { std::env::remove_var("KATANAUTE_API_URL") };

        Ok(())
    }

    #[test]
    fn load_uses_file_when_env_missing() -> Result<(), Box<dyn std::error::Error>> {
        let _guard = env_lock();
        let temp = tempfile::TempDir::new()?;
        unsafe { std::env::set_var("XDG_CONFIG_HOME", temp.path()) };

        // Write config file with a specific URL
        let config_dir = temp.path().join("katanaute");
        fs::create_dir_all(&config_dir)?;
        let config_file = config_dir.join("config.json");
        let config_content = serde_json::json!({
            "base_url": "http://from-file"
        });
        fs::write(&config_file, serde_json::to_string_pretty(&config_content)?)?;

        // Ensure env var is not set
        unsafe { std::env::remove_var("KATANAUTE_API_URL") };

        let cfg = Config::load()?;
        assert_eq!(cfg.base_url, "http://from-file");

        // Clean up env var
        unsafe { std::env::remove_var("XDG_CONFIG_HOME") };

        Ok(())
    }

    #[test]
    fn save_token_and_clear_token_round_trip() -> Result<(), Box<dyn std::error::Error>> {
        let _guard = env_lock();
        let temp = tempfile::TempDir::new()?;
        unsafe { std::env::set_var("XDG_CONFIG_HOME", temp.path()) };

        // Initial load should have no token
        let cfg = Config::load()?;
        assert!(cfg.api_token.is_none());

        // Save token
        cfg.save_token("secret-token".to_string())?;

        // Load again should have the token
        let cfg2 = Config::load()?;
        assert_eq!(cfg2.api_token, Some("secret-token".to_string()));

        // Clear token
        cfg2.clear_token()?;

        // Load again should have no token
        let cfg3 = Config::load()?;
        assert!(cfg3.api_token.is_none());

        // Clean up env var
        unsafe { std::env::remove_var("XDG_CONFIG_HOME") };

        Ok(())
    }
}
