use anyhow::Result;
use chrono::Local;

use td_cli::context::AppContext;
use td_cli::output::{json, table};

pub fn execute(ctx: &AppContext, _no_overdue: bool) -> Result<()> {
    let today = Local::now().format("%Y-%m-%d").to_string();
    let tasks = ctx.cache.get_tasks_due_today_or_overdue(&today)?;

    if ctx.use_json() {
        let synced_at = ctx.cache.get_last_sync_time()?;
        println!("{}", json::render_tasks_json(&tasks, synced_at));
    } else if tasks.is_empty() {
        println!("No tasks due today. You're all caught up!");
    } else {
        println!("{}", table::render_task_table(&tasks, ctx.global.no_color));
    }

    Ok(())
}
