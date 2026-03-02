use anyhow::Result;
use td_api::requests::filters::*;

use td_cli::cli::FilterAction;
use td_cli::context::AppContext;
use td_cli::output::json;

pub async fn execute(ctx: &AppContext, action: &FilterAction) -> Result<()> {
    match action {
        FilterAction::List => {
            let filters = ctx.cache.get_all_filters()?;
            if ctx.use_json() {
                println!("{}", json::render_single_json(&filters));
            } else {
                for f in &filters {
                    println!("{}  {} — {}", f.id, f.name, f.query);
                }
            }
        }
        FilterAction::Show { id } => {
            let filter = ctx.cache.get_filter(id)?;
            if ctx.use_json() {
                println!("{}", json::render_single_json(&filter));
            } else {
                println!("ID:       {}", filter.id);
                println!("Name:     {}", filter.name);
                println!("Query:    {}", filter.query);
                println!("Color:    {}", filter.color.as_deref().unwrap_or("-"));
                println!("Favorite: {}", filter.is_favorite);
            }
        }
        FilterAction::Add { name, query, color } => {
            let req = CreateFilterRequest {
                name: name.clone(),
                query: query.clone(),
                color: color.clone(),
                order: None,
                is_favorite: None,
            };
            let filter = ctx.api.add_filter(&req).await?;
            ctx.cache.upsert_filter(&filter)?;
            if ctx.global.quiet {
                println!("{}", filter.id);
            } else {
                println!("Created filter: {} ({})", filter.name, filter.id);
            }
        }
        FilterAction::Edit {
            id,
            name,
            query,
            color,
        } => {
            let req = UpdateFilterRequest {
                name: name.clone(),
                query: query.clone(),
                color: color.clone(),
                ..Default::default()
            };
            let filter = ctx.api.update_filter(id, &req).await?;
            ctx.cache.upsert_filter(&filter)?;
            println!("Updated filter {}", filter.id);
        }
        FilterAction::Delete { id } => {
            ctx.api.delete_filter(id).await?;
            ctx.cache.delete_cached_filter(id)?;
            println!("Deleted filter {id}");
        }
    }
    Ok(())
}
