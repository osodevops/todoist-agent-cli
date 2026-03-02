use anyhow::Result;
use td_api::requests::tasks::CreateTaskRequest;

use td_cli::context::AppContext;
use td_cli::output::{json, table};

#[allow(clippy::too_many_arguments)]
pub async fn execute(
    ctx: &AppContext,
    content: &str,
    project: Option<String>,
    section: Option<String>,
    priority: Option<i32>,
    due: Option<String>,
    labels: Vec<String>,
    description: Option<String>,
    parent: Option<String>,
    duration: Option<u32>,
    deadline: Option<String>,
) -> Result<()> {
    // Resolve project name to ID if provided
    let project_id = if let Some(ref name) = project {
        if let Some(proj) = ctx.cache.find_project_by_name(name)? {
            Some(proj.id)
        } else {
            Some(name.clone()) // Assume it's already an ID
        }
    } else {
        None
    };

    // Resolve section name to ID if provided
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

    let req = CreateTaskRequest {
        content: content.to_string(),
        description,
        project_id,
        section_id,
        parent_id: parent,
        labels,
        priority,
        due_string: due,
        duration,
        duration_unit: duration.map(|_| "minute".to_string()),
        deadline_date: deadline,
        ..Default::default()
    };

    let task = ctx.api.add_task(&req).await?;

    // Optimistic cache update
    ctx.cache.upsert_task(&task)?;

    if ctx.global.quiet {
        println!("{}", task.id);
    } else if ctx.use_json() {
        println!("{}", json::render_single_json(&task));
    } else {
        println!("Created task: {} ({})", task.content, task.id);
        println!("{}", table::render_task_detail(&task, ctx.global.no_color));
    }

    Ok(())
}
