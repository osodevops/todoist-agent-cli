use anyhow::Result;
use dialoguer::Confirm;
use td_api::requests::labels::*;

use td_cli::cli::LabelAction;
use td_cli::context::AppContext;
use td_cli::output::json;

pub async fn execute(ctx: &AppContext, action: &LabelAction) -> Result<()> {
    match action {
        LabelAction::List => {
            let labels = ctx.cache.get_all_labels()?;
            if ctx.use_json() {
                println!("{}", json::render_single_json(&labels));
            } else {
                for l in &labels {
                    let color = l.color.as_deref().unwrap_or("");
                    println!("{}  {} {}", l.id, l.name, color);
                }
            }
        }
        LabelAction::Add { name, color } => {
            let req = CreateLabelRequest {
                name: name.clone(),
                color: color.clone(),
                order: None,
                is_favorite: None,
            };
            let label = ctx.api.add_label(&req).await?;
            ctx.cache.upsert_label(&label)?;
            if ctx.global.quiet {
                println!("{}", label.id);
            } else {
                println!("Created label: {} ({})", label.name, label.id);
            }
        }
        LabelAction::Edit { id, name, color } => {
            let req = UpdateLabelRequest {
                name: name.clone(),
                color: color.clone(),
                ..Default::default()
            };
            let label = ctx.api.update_label(id, &req).await?;
            ctx.cache.upsert_label(&label)?;
            println!("Updated label {}", label.id);
        }
        LabelAction::Delete { id, yes } => {
            if !yes {
                let confirmed = Confirm::new()
                    .with_prompt(format!("Delete label {id}?"))
                    .default(false)
                    .interact()?;
                if !confirmed {
                    println!("Cancelled.");
                    return Ok(());
                }
            }
            ctx.api.delete_label(id).await?;
            ctx.cache.delete_cached_label(id)?;
            println!("Deleted label {id}");
        }
    }
    Ok(())
}
