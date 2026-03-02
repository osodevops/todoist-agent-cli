use anyhow::Result;
use td_api::requests::tasks::MoveTaskRequest;

use td_cli::context::AppContext;

pub async fn execute(
    ctx: &AppContext,
    id: &str,
    project: Option<String>,
    section: Option<String>,
    parent: Option<String>,
    no_parent: bool,
) -> Result<()> {
    // Resolve project name to ID
    let project_id = if let Some(ref name) = project {
        if let Some(proj) = ctx.cache.find_project_by_name(name)? {
            Some(proj.id)
        } else {
            Some(name.clone())
        }
    } else {
        None
    };

    // Resolve section name to ID
    let section_id = if let Some(ref name) = section {
        if let Some(sec) = ctx
            .cache
            .find_section_by_name(name, project_id.as_deref())?
        {
            Some(sec.id)
        } else {
            Some(name.clone())
        }
    } else {
        None
    };

    let parent_id = if no_parent {
        Some(String::new())
    } else {
        parent
    };

    let req = MoveTaskRequest {
        project_id,
        section_id,
        parent_id,
    };

    let task = ctx.api.move_task(id, &req).await?;
    ctx.cache.upsert_task(&task)?;

    if !ctx.global.quiet {
        println!("Moved task {id}");
    }

    Ok(())
}
