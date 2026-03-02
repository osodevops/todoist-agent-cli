use anyhow::Result;

use td_cli::context::AppContext;
use td_cli::output::{json, table};

pub fn execute(
    ctx: &AppContext,
    project: Option<&str>,
    label: Option<&str>,
    limit: Option<usize>,
    all: bool,
) -> Result<()> {
    let mut tasks = if let Some(project_name) = project {
        // Try to resolve project name to ID
        if let Some(proj) = ctx.cache.find_project_by_name(project_name)? {
            ctx.cache.get_tasks_by_project(&proj.id)?
        } else {
            // Try as direct ID
            ctx.cache.get_tasks_by_project(project_name)?
        }
    } else if let Some(label_name) = label {
        ctx.cache.get_tasks_by_label(label_name)?
    } else {
        ctx.cache.get_all_cached_tasks()?
    };

    if !all {
        let max = limit.unwrap_or(50);
        tasks.truncate(max);
    }

    if ctx.use_json() {
        let synced_at = ctx.cache.get_last_sync_time()?;
        println!("{}", json::render_tasks_json(&tasks, synced_at));
    } else {
        println!("{}", table::render_task_table(&tasks, ctx.global.no_color));
    }

    Ok(())
}
