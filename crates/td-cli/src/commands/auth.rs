use anyhow::Result;

use td_cli::cli::AuthAction;
use td_cli::config::AppConfig;

pub async fn execute(action: &AuthAction) -> Result<()> {
    match action {
        AuthAction::Login => {
            eprintln!("Interactive login wizard.");
            eprintln!("1. Go to https://app.todoist.com/app/settings/integrations/developer");
            eprintln!("2. Copy your API token");
            eprintln!("3. Run: td auth token <YOUR_TOKEN>");
        }
        AuthAction::Token { token } => {
            let mut config = AppConfig::load()?;
            config.default.token = Some(token.clone());
            config.default.token_source = "config".into();
            config.save()?;
            println!("Token saved to config file.");
            println!("Run `td sync` to fetch your data.");
        }
        AuthAction::Status => {
            let config = AppConfig::load()?;
            if config.default.token.is_some() {
                println!("Authenticated (token stored in config)");
            } else {
                println!("Not authenticated. Run `td auth login` to set up.");
            }
        }
        AuthAction::Logout => {
            let mut config = AppConfig::load()?;
            config.default.token = None;
            config.save()?;
            println!("Logged out. Token removed from config.");
        }
        AuthAction::Switch { profile } => {
            println!("Switched to profile: {profile}");
            println!("Use --profile {profile} or set TD_PROFILE={profile}");
        }
    }

    Ok(())
}
