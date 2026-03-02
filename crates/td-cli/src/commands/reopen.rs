use anyhow::Result;

use td_cli::context::AppContext;

pub async fn execute(ctx: &AppContext, id: &str) -> Result<()> {
    ctx.api.reopen_task(id).await?;
    ctx.cache.mark_task_reopened(id)?;

    if !ctx.global.quiet {
        println!("Reopened task {id}");
    }

    Ok(())
}
