use chrono::NaiveDate;
use colored::Colorize;
use comfy_table::{Cell, ContentArrangement, Table};
use td_api::models::Task;

pub fn render_task_table(tasks: &[Task], no_color: bool) -> String {
    if tasks.is_empty() {
        return "No tasks found.".to_string();
    }

    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);

    if no_color {
        table.set_header(vec!["ID", "P", "Content", "Due", "Labels"]);
    } else {
        table.set_header(vec![
            Cell::new("ID"),
            Cell::new("P"),
            Cell::new("Content"),
            Cell::new("Due"),
            Cell::new("Labels"),
        ]);
    }

    let today = chrono::Local::now().date_naive();

    for task in tasks {
        let id = &task.id;
        let priority = format_priority(task.priority, no_color);
        let content = truncate(&task.content, 50);
        let due = format_due_date(&task.due, today, no_color);
        let labels = task.labels.join(", ");

        table.add_row(vec![
            Cell::new(id),
            Cell::new(&priority),
            Cell::new(&content),
            Cell::new(&due),
            Cell::new(&labels),
        ]);
    }

    table.to_string()
}

pub fn render_task_detail(task: &Task, no_color: bool) -> String {
    let mut lines = Vec::new();

    lines.push(format!("ID:          {}", task.id));
    lines.push(format!("Content:     {}", task.content));
    if !task.description.is_empty() {
        lines.push(format!("Description: {}", task.description));
    }
    lines.push(format!(
        "Priority:    {}",
        format_priority(task.priority, no_color)
    ));
    lines.push(format!("Project:     {}", task.project_id));
    if let Some(ref section) = task.section_id {
        lines.push(format!("Section:     {section}"));
    }
    if let Some(ref due) = task.due {
        lines.push(format!("Due:         {}", due.date));
        if let Some(ref s) = due.string {
            lines.push(format!("Due string:  {s}"));
        }
    }
    if !task.labels.is_empty() {
        lines.push(format!("Labels:      {}", task.labels.join(", ")));
    }
    if let Some(ref url) = task.url {
        lines.push(format!("URL:         {url}"));
    }

    lines.join("\n")
}

fn format_priority(priority: i32, no_color: bool) -> String {
    let label = match priority {
        1 => "p1",
        2 => "p2",
        3 => "p3",
        _ => "p4",
    };
    if no_color {
        return label.to_string();
    }
    match priority {
        1 => label.red().bold().to_string(),
        2 => label.yellow().to_string(),
        3 => label.blue().to_string(),
        _ => label.dimmed().to_string(),
    }
}

fn format_due_date(
    due: &Option<td_api::models::common::DueDate>,
    today: NaiveDate,
    no_color: bool,
) -> String {
    let Some(due) = due else {
        return String::new();
    };

    let date_str = &due.date;
    if no_color {
        return date_str.clone();
    }

    if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        let tomorrow = today + chrono::Duration::days(1);
        if date < today {
            return date_str.red().to_string(); // overdue
        } else if date == today {
            return date_str.green().to_string(); // today
        } else if date == tomorrow {
            return date_str.yellow().to_string(); // tomorrow
        }
    }

    date_str.clone()
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max - 3])
    }
}
