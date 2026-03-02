use anyhow::Result;
use dialoguer::Confirm;

use td_cli::context::AppContext;

pub async fn execute(ctx: &AppContext, ids: &[String], yes: bool) -> Result<()> {
    if ids.is_empty() {
        anyhow::bail!("No task IDs provided");
    }

    if !yes {
        let msg = if ids.len() == 1 {
            format!("Delete task {}?", ids[0])
        } else {
            format!("Delete {} tasks?", ids.len())
        };
        let confirmed = Confirm::new().with_prompt(&msg).default(false).interact()?;
        if !confirmed {
            println!("Cancelled.");
            return Ok(());
        }
    }

    for id in ids {
        ctx.api.delete_task(id).await?;
        ctx.cache.delete_cached_task(id)?;

        if !ctx.global.quiet {
            println!("Deleted task {id}");
        }
    }

    Ok(())
}
