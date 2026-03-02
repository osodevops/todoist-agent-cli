use anyhow::Result;
use dialoguer::Confirm;
use td_api::requests::projects::*;

use td_cli::cli::ProjectAction;
use td_cli::context::AppContext;
use td_cli::output::json;

pub async fn execute(ctx: &AppContext, action: &ProjectAction) -> Result<()> {
    match action {
        ProjectAction::List { archived: _ } => {
            let projects = ctx.cache.get_all_projects()?;
            if ctx.use_json() {
                println!("{}", json::render_single_json(&projects));
            } else {
                for p in &projects {
                    let fav = if p.is_favorite { " *" } else { "" };
                    let inbox = if p.is_inbox_project { " (Inbox)" } else { "" };
                    println!("{}  {}{}{}", p.id, p.name, inbox, fav);
                }
            }
        }
        ProjectAction::Show { id } => {
            let project = ctx.cache.get_project(id)?;
            if ctx.use_json() {
                println!("{}", json::render_single_json(&project));
            } else {
                println!("ID:       {}", project.id);
                println!("Name:     {}", project.name);
                println!("Color:    {}", project.color.as_deref().unwrap_or("-"));
                println!("Favorite: {}", project.is_favorite);
                if let Some(ref url) = project.url {
                    println!("URL:      {url}");
                }
            }
        }
        ProjectAction::Add {
            name,
            parent,
            color,
            view,
        } => {
            let parent_id = if let Some(n) = parent {
                ctx.cache
                    .find_project_by_name(n)?
                    .map(|p| p.id)
                    .or_else(|| Some(n.clone()))
            } else {
                None
            };
            let req = CreateProjectRequest {
                name: name.clone(),
                parent_id,
                color: color.clone(),
                view_style: view.clone(),
                ..Default::default()
            };
            let project = ctx.api.add_project(&req).await?;
            ctx.cache.upsert_project(&project)?;
            if ctx.global.quiet {
                println!("{}", project.id);
            } else {
                println!("Created project: {} ({})", project.name, project.id);
            }
        }
        ProjectAction::Edit {
            id,
            name,
            color,
            favorite,
        } => {
            let req = UpdateProjectRequest {
                name: name.clone(),
                color: color.clone(),
                is_favorite: if *favorite { Some(true) } else { None },
                ..Default::default()
            };
            let project = ctx.api.update_project(id, &req).await?;
            ctx.cache.upsert_project(&project)?;
            println!("Updated project {}", project.id);
        }
        ProjectAction::Archive { id } => {
            ctx.api.archive_project(id).await?;
            println!("Archived project {id}");
        }
        ProjectAction::Unarchive { id } => {
            ctx.api.unarchive_project(id).await?;
            println!("Unarchived project {id}");
        }
        ProjectAction::Delete { id, yes } => {
            if !yes {
                let confirmed = Confirm::new()
                    .with_prompt(format!("Delete project {id}?"))
                    .default(false)
                    .interact()?;
                if !confirmed {
                    println!("Cancelled.");
                    return Ok(());
                }
            }
            ctx.api.delete_project(id).await?;
            ctx.cache.delete_cached_project(id)?;
            println!("Deleted project {id}");
        }
        ProjectAction::Collaborators { id } => {
            let collabs = ctx.api.get_project_collaborators(id).await?;
            if ctx.use_json() {
                println!("{}", json::render_single_json(&collabs));
            } else {
                for c in &collabs {
                    println!("{}  {} <{}>", c.id, c.name, c.email);
                }
            }
        }
    }
    Ok(())
}
