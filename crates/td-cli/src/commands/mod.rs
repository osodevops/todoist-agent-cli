mod activity;
mod add;
mod auth;
mod comment;
mod completions;
mod delete;
mod done;
mod edit;
mod filter_cmd;
mod inbox;
mod label;
mod list;
mod move_cmd;
mod project;
mod quick;
mod reminder;
mod reopen;
mod section;
mod show;
mod sync;
mod today;

use anyhow::Result;

use td_cli::cli::{Cli, Commands};
use td_cli::context::AppContext;

pub async fn execute(cli: Cli) -> Result<()> {
    // Commands that don't need AppContext
    match &cli.command {
        Commands::Auth { action } => return auth::execute(action).await,
        Commands::Completions { shell } => return completions::execute(*shell),
        _ => {}
    }

    let mut ctx = AppContext::new(cli.global)?;

    // Auto-sync before read commands if --sync flag is set
    if ctx.global.sync {
        sync::do_sync(&mut ctx).await?;
    }

    match cli.command {
        Commands::Sync { full, status } => {
            if status {
                sync::show_status(&ctx)?;
            } else if full {
                sync::do_full_sync(&mut ctx).await?;
            } else {
                sync::do_sync(&mut ctx).await?;
            }
        }
        Commands::List {
            project,
            label,
            section: _,
            sort: _,
            limit,
            tree: _,
            all,
        } => list::execute(&ctx, project.as_deref(), label.as_deref(), limit, all)?,
        Commands::Today { no_overdue } => today::execute(&ctx, no_overdue)?,
        Commands::Inbox => inbox::execute(&ctx)?,
        Commands::Add {
            content,
            project,
            section,
            priority,
            due,
            label,
            description,
            parent,
            duration,
            deadline,
        } => {
            add::execute(
                &ctx,
                &content,
                project,
                section,
                priority,
                due,
                label,
                description,
                parent,
                duration,
                deadline,
            )
            .await?
        }
        Commands::Quick { text } => quick::execute(&ctx, &text).await?,
        Commands::Done { ids } => done::execute(&ctx, &ids).await?,
        Commands::Delete { ids, yes } => delete::execute(&ctx, &ids, yes).await?,
        Commands::Show { id } => show::execute(&ctx, &id)?,
        Commands::Edit {
            id,
            content,
            due,
            priority,
            add_label,
            remove_label,
            description,
            no_due,
        } => {
            edit::execute(
                &ctx,
                &id,
                content,
                due,
                priority,
                add_label,
                remove_label,
                description,
                no_due,
            )
            .await?
        }
        Commands::Reopen { id } => reopen::execute(&ctx, &id).await?,
        Commands::Move {
            id,
            project,
            section,
            parent,
            no_parent,
        } => move_cmd::execute(&ctx, &id, project, section, parent, no_parent).await?,
        Commands::Project { action } => project::execute(&ctx, &action).await?,
        Commands::Section { action } => section::execute(&ctx, &action).await?,
        Commands::Label { action } => label::execute(&ctx, &action).await?,
        Commands::Comment { action } => comment::execute(&ctx, &action).await?,
        Commands::Reminder { action } => reminder::execute(&ctx, &action).await?,
        Commands::Filter { action } => filter_cmd::execute(&ctx, &action).await?,
        Commands::Activity {
            limit,
            event_type,
            project,
            since: _,
        } => activity::execute(&ctx, limit, event_type.as_deref(), project.as_deref()).await?,
        // Auth and Completions already handled above
        Commands::Auth { .. } | Commands::Completions { .. } => unreachable!(),
    }

    Ok(())
}
