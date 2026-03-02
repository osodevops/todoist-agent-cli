use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    #[serde(default)]
    pub default: ProfileConfig,
    #[serde(default)]
    pub profiles: HashMap<String, ProfileConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileConfig {
    #[serde(default = "default_token_source")]
    pub token_source: String,
    pub token: Option<String>,
    #[serde(default = "default_color")]
    pub color: String,
    #[serde(default = "default_date_format")]
    pub date_format: String,
    #[serde(default = "default_output")]
    pub default_output: String,
    #[serde(default = "default_true")]
    pub auto_sync_on_write: bool,
    #[serde(default = "default_sync_timeout")]
    pub sync_timeout_secs: u64,
}

impl Default for ProfileConfig {
    fn default() -> Self {
        Self {
            token_source: default_token_source(),
            token: None,
            color: default_color(),
            date_format: default_date_format(),
            default_output: default_output(),
            auto_sync_on_write: true,
            sync_timeout_secs: default_sync_timeout(),
        }
    }
}

fn default_token_source() -> String {
    "env".into()
}
fn default_color() -> String {
    "auto".into()
}
fn default_date_format() -> String {
    "%Y-%m-%d".into()
}
fn default_output() -> String {
    "table".into()
}
fn default_true() -> bool {
    true
}
fn default_sync_timeout() -> u64 {
    30
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        if path.exists() {
            let contents = std::fs::read_to_string(&path)
                .with_context(|| format!("Failed to read config at {}", path.display()))?;
            let config: AppConfig = toml::from_str(&contents)
                .with_context(|| format!("Failed to parse config at {}", path.display()))?;
            Ok(config)
        } else {
            Ok(AppConfig::default())
        }
    }

    pub fn config_path() -> Result<PathBuf> {
        let dirs =
            ProjectDirs::from("", "", "td").context("Could not determine config directory")?;
        Ok(dirs.config_dir().join("config.toml"))
    }

    #[allow(dead_code)]
    pub fn config_dir() -> Result<PathBuf> {
        let dirs =
            ProjectDirs::from("", "", "td").context("Could not determine config directory")?;
        Ok(dirs.config_dir().to_path_buf())
    }

    pub fn cache_dir() -> Result<PathBuf> {
        let dirs =
            ProjectDirs::from("", "", "td").context("Could not determine cache directory")?;
        Ok(dirs.cache_dir().to_path_buf())
    }

    pub fn cache_path(profile: Option<&str>) -> Result<PathBuf> {
        let dir = Self::cache_dir()?;
        let filename = match profile {
            Some(p) => format!("cache-{p}.db"),
            None => "cache-default.db".to_string(),
        };
        Ok(dir.join(filename))
    }

    pub fn profile(&self, name: Option<&str>) -> &ProfileConfig {
        match name {
            Some(n) => self.profiles.get(n).unwrap_or(&self.default),
            None => &self.default,
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let contents = toml::to_string_pretty(self)?;
        std::fs::write(&path, contents)?;
        Ok(())
    }
}

/// Resolve the API token from multiple sources (flag > env > config).
pub fn resolve_token(flag_token: Option<&str>, profile_config: &ProfileConfig) -> Result<String> {
    // 1. CLI flag (also populated from TODOIST_API_TOKEN env via clap)
    if let Some(token) = flag_token
        && !token.is_empty()
    {
        return Ok(token.to_string());
    }

    // 2. Config file token
    if let Some(ref token) = profile_config.token
        && !token.is_empty()
    {
        return Ok(token.clone());
    }

    anyhow::bail!("No API token found. Set TODOIST_API_TOKEN, use --token, or run `td auth login`.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.default.default_output, "table");
        assert_eq!(config.default.color, "auto");
        assert!(config.default.auto_sync_on_write);
    }

    #[test]
    fn test_resolve_token_flag() {
        let profile = ProfileConfig::default();
        let token = resolve_token(Some("flag-token"), &profile).unwrap();
        assert_eq!(token, "flag-token");
    }

    #[test]
    fn test_resolve_token_config() {
        let profile = ProfileConfig {
            token: Some("config-token".into()),
            ..Default::default()
        };
        let token = resolve_token(None, &profile).unwrap();
        assert_eq!(token, "config-token");
    }

    #[test]
    fn test_resolve_token_missing() {
        let profile = ProfileConfig::default();
        let result = resolve_token(None, &profile);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_toml_config() {
        let toml_str = r#"
[default]
token = "abc123"
color = "always"
default_output = "json"
"#;
        let config: AppConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.default.token.unwrap(), "abc123");
        assert_eq!(config.default.color, "always");
        assert_eq!(config.default.default_output, "json");
    }
}
