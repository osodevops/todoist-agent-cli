use anyhow::Result;
use td_api::requests::comments::*;

use td_cli::cli::CommentAction;
use td_cli::context::AppContext;
use td_cli::output::json;

pub async fn execute(ctx: &AppContext, action: &CommentAction) -> Result<()> {
    match action {
        CommentAction::List { task, project } => {
            let comments = if let Some(tid) = task {
                ctx.api.get_comments_for_task(tid).await?
            } else if let Some(pid) = project {
                ctx.api.get_comments_for_project(pid).await?
            } else {
                anyhow::bail!("Provide --task or --project");
            };
            if ctx.use_json() {
                println!("{}", json::render_single_json(&comments));
            } else {
                for c in &comments {
                    println!(
                        "[{}] {} - {}",
                        c.id,
                        c.posted_at.as_deref().unwrap_or(""),
                        c.content
                    );
                }
            }
        }
        CommentAction::Add {
            task,
            project,
            content,
        } => {
            let req = CreateCommentRequest {
                task_id: task.clone(),
                project_id: project.clone(),
                content: content.clone(),
            };
            let comment = ctx.api.add_comment(&req).await?;
            ctx.cache.upsert_comment(&comment)?;
            if ctx.global.quiet {
                println!("{}", comment.id);
            } else {
                println!("Added comment {} on {}", comment.id, content);
            }
        }
        CommentAction::Edit { id, content } => {
            let req = UpdateCommentRequest {
                content: content.clone(),
            };
            let comment = ctx.api.update_comment(id, &req).await?;
            ctx.cache.upsert_comment(&comment)?;
            println!("Updated comment {}", comment.id);
        }
        CommentAction::Delete { id } => {
            ctx.api.delete_comment(id).await?;
            ctx.cache.delete_cached_comment(id)?;
            println!("Deleted comment {id}");
        }
    }
    Ok(())
}
