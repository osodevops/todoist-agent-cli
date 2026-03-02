use anyhow::Result;

use td_cli::context::AppContext;
use td_cli::output::{json, table};

pub fn execute(ctx: &AppContext, id: &str) -> Result<()> {
    // Try cache first, then extract ID from URL if needed
    let task_id = extract_task_id(id);
    let task = ctx.cache.get_task(&task_id)?;

    if ctx.use_json() {
        println!("{}", json::render_single_json(&task));
    } else {
        println!("{}", table::render_task_detail(&task, ctx.global.no_color));
    }

    Ok(())
}

fn extract_task_id(input: &str) -> String {
    // Handle Todoist URLs like https://app.todoist.com/app/task/buy-milk-8Jx4mVr72kPn3QwB
    if input.contains("todoist.com")
        && let Some(last_segment) = input.rsplit('/').next()
    {
        // The ID is the last part after the last hyphen
        if let Some(pos) = last_segment.rfind('-') {
            return last_segment[pos + 1..].to_string();
        }
        return last_segment.to_string();
    }

    // Handle id:PREFIX format
    if let Some(stripped) = input.strip_prefix("id:") {
        return stripped.to_string();
    }

    input.to_string()
}
