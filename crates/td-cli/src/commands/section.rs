use anyhow::Result;
use dialoguer::Confirm;
use td_api::requests::sections::*;

use td_cli::cli::SectionAction;
use td_cli::context::AppContext;
use td_cli::output::json;

pub async fn execute(ctx: &AppContext, action: &SectionAction) -> Result<()> {
    match action {
        SectionAction::List { project } => {
            let sections = if let Some(proj_name) = project {
                if let Some(proj) = ctx.cache.find_project_by_name(proj_name)? {
                    ctx.cache.get_sections_by_project(&proj.id)?
                } else {
                    ctx.cache.get_sections_by_project(proj_name)?
                }
            } else {
                ctx.cache.get_all_sections()?
            };
            if ctx.use_json() {
                println!("{}", json::render_single_json(&sections));
            } else {
                for s in &sections {
                    println!("{}  {} (project: {})", s.id, s.name, s.project_id);
                }
            }
        }
        SectionAction::Add { name, project } => {
            let project_id = if let Some(proj) = ctx.cache.find_project_by_name(project)? {
                proj.id
            } else {
                project.clone()
            };
            let req = CreateSectionRequest {
                name: name.clone(),
                project_id,
                order: None,
            };
            let section = ctx.api.add_section(&req).await?;
            ctx.cache.upsert_section(&section)?;
            if ctx.global.quiet {
                println!("{}", section.id);
            } else {
                println!("Created section: {} ({})", section.name, section.id);
            }
        }
        SectionAction::Edit { id, name } => {
            let req = UpdateSectionRequest {
                name: name.clone(),
                ..Default::default()
            };
            let section = ctx.api.update_section(id, &req).await?;
            ctx.cache.upsert_section(&section)?;
            println!("Updated section {}", section.id);
        }
        SectionAction::Move { id, order } => {
            let req = UpdateSectionRequest {
                order: Some(*order),
                ..Default::default()
            };
            let section = ctx.api.update_section(id, &req).await?;
            ctx.cache.upsert_section(&section)?;
            println!("Moved section {}", section.id);
        }
        SectionAction::Delete { id, yes } => {
            if !yes {
                let confirmed = Confirm::new()
                    .with_prompt(format!("Delete section {id}?"))
                    .default(false)
                    .interact()?;
                if !confirmed {
                    println!("Cancelled.");
                    return Ok(());
                }
            }
            ctx.api.delete_section(id).await?;
            ctx.cache.delete_cached_section(id)?;
            println!("Deleted section {id}");
        }
    }
    Ok(())
}
