use anyhow::Result;
use chrono::Utc;
use tracing::info;

use td_cli::context::AppContext;

pub async fn do_sync(ctx: &mut AppContext) -> Result<()> {
    do_full_sync(ctx).await
}

pub async fn do_full_sync(ctx: &mut AppContext) -> Result<()> {
    if !ctx.global.quiet {
        eprintln!("Syncing...");
    }

    // Fetch all resources in parallel
    let (tasks, projects, sections, labels) = tokio::try_join!(
        ctx.api.get_all_tasks(),
        ctx.api
            .get_all_pages::<td_api::models::Project>("/projects"),
        ctx.api
            .get_all_pages::<td_api::models::Section>("/sections"),
        ctx.api.get_all_pages::<td_api::models::Label>("/labels"),
    )?;

    info!(
        tasks = tasks.len(),
        projects = projects.len(),
        sections = sections.len(),
        labels = labels.len(),
        "Fetched all resources"
    );

    // Replace all cached data in a transaction-like manner
    ctx.cache.replace_all_tasks(&tasks)?;
    ctx.cache.replace_all_projects(&projects)?;
    ctx.cache.replace_all_sections(&sections)?;
    ctx.cache.replace_all_labels(&labels)?;

    let now = Utc::now().to_rfc3339();
    ctx.cache.set_last_sync_time(&now)?;

    if !ctx.global.quiet {
        let stats = ctx.cache.get_cached_resource_counts()?;
        eprintln!(
            "Synced: {} tasks, {} projects, {} sections, {} labels",
            stats.tasks, stats.projects, stats.sections, stats.labels
        );
    }

    Ok(())
}

pub fn show_status(ctx: &AppContext) -> Result<()> {
    let last_sync = ctx.cache.get_last_sync_time()?;
    let stats = ctx.cache.get_cached_resource_counts()?;

    println!("Last sync:  {}", last_sync.as_deref().unwrap_or("never"));
    println!("Tasks:      {}", stats.tasks);
    println!("Projects:   {}", stats.projects);
    println!("Sections:   {}", stats.sections);
    println!("Labels:     {}", stats.labels);

    Ok(())
}
