use serde::Serialize;
use td_api::models::Task;

#[derive(Serialize)]
pub struct TaskListOutput<'a> {
    pub tasks: &'a [Task],
    pub meta: Meta,
}

#[derive(Serialize)]
pub struct Meta {
    pub total: usize,
    pub synced_at: Option<String>,
}

pub fn render_tasks_json(tasks: &[Task], synced_at: Option<String>) -> String {
    let output = TaskListOutput {
        tasks,
        meta: Meta {
            total: tasks.len(),
            synced_at,
        },
    };
    serde_json::to_string_pretty(&output).unwrap_or_else(|_| "{}".to_string())
}

pub fn render_single_json<T: Serialize>(item: &T) -> String {
    serde_json::to_string_pretty(item).unwrap_or_else(|_| "{}".to_string())
}
