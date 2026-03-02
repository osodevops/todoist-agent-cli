use anyhow::Result;

use td_cli::context::AppContext;
use td_cli::output::{json, table};

pub async fn execute(ctx: &AppContext, text: &str) -> Result<()> {
    let task = ctx.api.quick_add_task(text).await?;

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
