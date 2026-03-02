use anyhow::Result;
use td_api::requests::tasks::UpdateTaskRequest;

use td_cli::context::AppContext;
use td_cli::output::{json, table};

#[allow(clippy::too_many_arguments)]
pub async fn execute(
    ctx: &AppContext,
    id: &str,
    content: Option<String>,
    due: Option<String>,
    priority: Option<i32>,
    add_label: Vec<String>,
    remove_label: Vec<String>,
    description: Option<String>,
    no_due: bool,
) -> Result<()> {
    // Build labels list if modifying
    let labels = if !add_label.is_empty() || !remove_label.is_empty() {
        let task = ctx.cache.get_task(id)?;
        let mut current_labels = task.labels;
        for l in &add_label {
            if !current_labels.contains(l) {
                current_labels.push(l.clone());
            }
        }
        current_labels.retain(|l| !remove_label.contains(l));
        Some(current_labels)
    } else {
        None
    };

    let due_string = if no_due {
        Some(String::new()) // Empty string to clear due date
    } else {
        due
    };

    let req = UpdateTaskRequest {
        content,
        description,
        labels,
        priority,
        due_string,
        ..Default::default()
    };

    let task = ctx.api.update_task(id, &req).await?;
    ctx.cache.upsert_task(&task)?;

    if ctx.global.quiet {
        println!("{}", task.id);
    } else if ctx.use_json() {
        println!("{}", json::render_single_json(&task));
    } else {
        println!("Updated task {}", task.id);
        println!("{}", table::render_task_detail(&task, ctx.global.no_color));
    }

    Ok(())
}
