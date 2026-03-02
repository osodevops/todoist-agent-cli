use anyhow::Result;
use td_api::requests::activity::GetActivityQuery;

use td_cli::context::AppContext;
use td_cli::output::json;

pub async fn execute(
    ctx: &AppContext,
    limit: usize,
    event_type: Option<&str>,
    project: Option<&str>,
) -> Result<()> {
    let query = GetActivityQuery {
        event_type: event_type.map(String::from),
        parent_project_id: project.map(String::from),
        limit: Some(limit),
        ..Default::default()
    };

    let events = ctx.api.get_activity(&query).await?;

    if ctx.use_json() {
        println!("{}", json::render_single_json(&events));
    } else {
        for event in &events {
            println!(
                "{}  {} {}  {}",
                event.event_date.as_deref().unwrap_or(""),
                event.event_type,
                event.object_type,
                event.object_id.as_deref().unwrap_or(""),
            );
        }
    }

    Ok(())
}
