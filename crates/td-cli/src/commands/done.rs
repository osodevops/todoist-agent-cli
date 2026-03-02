use anyhow::Result;

use td_cli::context::AppContext;

pub async fn execute(ctx: &AppContext, ids: &[String]) -> Result<()> {
    if ids.is_empty() {
        anyhow::bail!("No task IDs provided");
    }

    for id in ids {
        ctx.api.close_task(id).await?;
        ctx.cache.mark_task_completed(id)?;

        if !ctx.global.quiet {
            println!("Completed task {id}");
        }
    }

    Ok(())
}
