use anyhow::Result;

use td_cli::context::AppContext;
use td_cli::output::{json, table};

pub fn execute(ctx: &AppContext) -> Result<()> {
    let tasks = ctx.cache.get_inbox_tasks()?;

    if ctx.use_json() {
        let synced_at = ctx.cache.get_last_sync_time()?;
        println!("{}", json::render_tasks_json(&tasks, synced_at));
    } else if tasks.is_empty() {
        println!("Inbox is empty.");
    } else {
        println!("{}", table::render_task_table(&tasks, ctx.global.no_color));
    }

    Ok(())
}
