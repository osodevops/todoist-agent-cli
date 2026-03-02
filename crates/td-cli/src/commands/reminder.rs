use anyhow::Result;
use td_api::requests::reminders::*;

use td_cli::cli::ReminderAction;
use td_cli::context::AppContext;
use td_cli::output::json;

pub async fn execute(ctx: &AppContext, action: &ReminderAction) -> Result<()> {
    match action {
        ReminderAction::List { task } => {
            let reminders = ctx.cache.get_reminders_for_task(task)?;
            if ctx.use_json() {
                println!("{}", json::render_single_json(&reminders));
            } else {
                for r in &reminders {
                    let due_info = r
                        .due
                        .as_ref()
                        .map(|d| d.date.clone())
                        .or(r.minute_offset.map(|m| format!("{m} min before")))
                        .unwrap_or_default();
                    println!("{}  {}", r.id, due_info);
                }
            }
        }
        ReminderAction::Add {
            task,
            due,
            relative,
        } => {
            let req = CreateReminderRequest {
                item_id: task.clone(),
                due_string: due.clone(),
                due_date: None,
                due_datetime: None,
                minute_offset: *relative,
            };
            let reminder = ctx.api.add_reminder(&req).await?;
            ctx.cache.upsert_reminder(&reminder)?;
            if ctx.global.quiet {
                println!("{}", reminder.id);
            } else {
                println!("Created reminder {}", reminder.id);
            }
        }
        ReminderAction::Delete { id } => {
            ctx.api.delete_reminder(id).await?;
            ctx.cache.delete_cached_reminder(id)?;
            println!("Deleted reminder {id}");
        }
    }
    Ok(())
}
