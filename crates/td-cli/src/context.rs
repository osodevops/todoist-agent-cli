use anyhow::{Context, Result};
use td_api::TodoistClient;
use td_cache::CacheDb;

use crate::cli::GlobalArgs;
use crate::config::{AppConfig, resolve_token};

pub struct AppContext {
    pub api: TodoistClient,
    pub cache: CacheDb,
    #[allow(dead_code)]
    pub config: AppConfig,
    pub global: GlobalArgs,
}

impl AppContext {
    pub fn new(global: GlobalArgs) -> Result<Self> {
        let config = AppConfig::load()?;
        let profile_config = config.profile(global.profile.as_deref());

        let token = resolve_token(global.token.as_deref(), profile_config)?;
        let api = TodoistClient::new(&token).context("Failed to create API client")?;

        let cache_path = AppConfig::cache_path(global.profile.as_deref())?;
        let cache = CacheDb::open(&cache_path).context("Failed to open cache database")?;

        Ok(Self {
            api,
            cache,
            config,
            global,
        })
    }

    pub fn use_json(&self) -> bool {
        self.global.json || !atty::is(atty::Stream::Stdout)
    }
}
