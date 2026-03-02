use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "td",
    about = "Fast, offline-capable Todoist CLI",
    long_about = "A fast, offline-capable, feature-complete command-line interface for Todoist.\n\n\
        Built on the Todoist API v1, td provides instant reads from a local SQLite cache,\n\
        full resource coverage (tasks, projects, sections, labels, comments, reminders,\n\
        filters, activity), and both human-friendly table output and machine-readable JSON.",
    version,
    propagate_version = true,
    after_help = "Use 'td <command> --help' for more information about a specific command.\n\n\
        Documentation: https://github.com/osodevops/todoist-agent-cli"
)]
pub struct Cli {
    #[command(flatten)]
    pub global: GlobalArgs,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Args)]
pub struct GlobalArgs {
    /// Sync with Todoist before executing the command
    #[arg(long, short = 's', global = true)]
    pub sync: bool,

    /// Force JSON output (default when stdout is not a TTY)
    #[arg(long, short = 'j', global = true)]
    pub json: bool,

    /// Suppress output (errors only, IDs only on creation)
    #[arg(long, short = 'q', global = true)]
    pub quiet: bool,

    /// Enable debug logging
    #[arg(long, short = 'v', global = true)]
    pub verbose: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,

    /// Override API token for this invocation
    #[arg(long, global = true, env = "TODOIST_API_TOKEN")]
    pub token: Option<String>,

    /// Use a named profile from config
    #[arg(long, global = true, env = "TD_PROFILE")]
    pub profile: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Sync local cache with Todoist
    #[command(
        long_about = "Sync the local SQLite cache with Todoist.\n\n\
            By default performs a full sync, fetching all tasks, projects, sections, and labels\n\
            in parallel and replacing the local cache.",
        after_help = "Examples:\n  td sync              # Full sync\n  td sync --status     # Show last sync time and cache stats"
    )]
    Sync {
        /// Full sync (rebuild cache from scratch)
        #[arg(long)]
        full: bool,
        /// Show sync status (last sync time, cached resource counts)
        #[arg(long)]
        status: bool,
    },

    /// List tasks from the local cache
    #[command(
        long_about = "List tasks from the local cache with optional filtering, sorting, and limiting.\n\n\
            Reads are instant (no network required). Use --sync to refresh before listing.",
        after_help = "Examples:\n  td list                        # All active tasks\n  \
            td list --project Work         # Tasks in the Work project\n  \
            td list --label urgent         # Tasks with the 'urgent' label\n  \
            td list --limit 10 --sort due  # Top 10 by due date\n  \
            td list --all --tree           # All tasks as a subtask tree"
    )]
    List {
        /// Filter by project name or ID
        #[arg(long)]
        project: Option<String>,
        /// Filter by label name
        #[arg(long)]
        label: Option<String>,
        /// Filter by section name
        #[arg(long)]
        section: Option<String>,
        /// Sort by field (due, priority, created, updated)
        #[arg(long, default_value = "due")]
        sort: String,
        /// Limit number of results
        #[arg(long)]
        limit: Option<usize>,
        /// Show subtask hierarchy as a tree
        #[arg(long)]
        tree: bool,
        /// Show all tasks with no limit
        #[arg(long)]
        all: bool,
    },

    /// Show today's agenda (due today + overdue)
    #[command(
        long_about = "Show tasks that are due today and any overdue tasks.\n\n\
            Reads from the local cache. Use --sync to refresh first.",
        after_help = "Examples:\n  td today               # Today + overdue\n  td today --no-overdue  # Only tasks due today"
    )]
    Today {
        /// Exclude overdue tasks (show only tasks due today)
        #[arg(long)]
        no_overdue: bool,
    },

    /// Show tasks in the Inbox project
    #[command(long_about = "Show all tasks in the Inbox project.\n\n\
            The Inbox is the default project for tasks without an explicit project.")]
    Inbox,

    /// Add a new task
    #[command(
        long_about = "Create a new task via the Todoist API.\n\n\
            The task is created on the server and the local cache is updated with the response.\n\
            Supports project/section/label resolution by name.",
        after_help = "Examples:\n  td add \"Buy groceries\"\n  \
            td add \"Review PR\" -p Work -P 1 -d tomorrow\n  \
            td add \"Fix bug\" -S \"In Progress\" -l urgent -l backend\n  \
            td add \"Sub-item\" --parent <TASK_ID>\n  \
            td add \"Team meeting\" -d \"every monday at 10am\" --duration 60"
    )]
    Add {
        /// Task content (the title of the task)
        content: String,

        /// Project name or ID
        #[arg(long, short = 'p')]
        project: Option<String>,

        /// Section name or ID
        #[arg(long, short = 'S')]
        section: Option<String>,

        /// Priority: 1 (urgent), 2 (high), 3 (medium), 4 (default)
        #[arg(long, short = 'P', value_parser = clap::value_parser!(i32).range(1..=4))]
        priority: Option<i32>,

        /// Due date (natural language or ISO 8601, e.g. "tomorrow", "2026-03-15")
        #[arg(long, short = 'd')]
        due: Option<String>,

        /// Label names (repeatable, e.g. -l urgent -l backend)
        #[arg(long, short = 'l')]
        label: Vec<String>,

        /// Task description / notes
        #[arg(long, short = 'D')]
        description: Option<String>,

        /// Parent task ID to create a subtask
        #[arg(long)]
        parent: Option<String>,

        /// Duration in minutes
        #[arg(long)]
        duration: Option<u32>,

        /// Deadline date (ISO 8601)
        #[arg(long)]
        deadline: Option<String>,
    },

    /// Quick add a task using natural language
    #[command(
        long_about = "Create a task using Todoist's server-side natural language parsing.\n\n\
            Supports #Project, @Label, pN priority, /Section, and date expressions.\n\
            The server parses the text and extracts structured fields automatically.",
        after_help = "Examples:\n  td quick \"Call mom tomorrow at 5pm #Personal @important p1\"\n  \
            td quick \"Submit report every Friday #Work\"\n  \
            td quick \"Buy milk /Groceries\""
    )]
    Quick {
        /// Natural language task text
        text: String,
    },

    /// Complete one or more tasks
    #[command(
        long_about = "Mark tasks as complete via the Todoist API.\n\n\
            Supports batch completion by passing multiple IDs.",
        after_help = "Examples:\n  td done abc123\n  td done id1 id2 id3          # Batch complete"
    )]
    Done {
        /// Task IDs to complete
        #[arg(required = true)]
        ids: Vec<String>,
    },

    /// Delete one or more tasks
    #[command(
        long_about = "Permanently delete tasks via the Todoist API.\n\n\
            Prompts for confirmation unless --yes is passed. Supports batch deletion.",
        after_help = "Examples:\n  td delete abc123\n  td delete id1 id2 --yes     # Batch delete, skip confirmation"
    )]
    Delete {
        /// Task IDs to delete
        #[arg(required = true)]
        ids: Vec<String>,

        /// Skip confirmation prompt
        #[arg(long)]
        yes: bool,
    },

    /// Show full details for a task
    #[command(
        long_about = "Display detailed information for a single task.\n\n\
            Accepts a task ID or a Todoist task URL. Shows all fields including\n\
            description, labels, due date, project, section, and timestamps.",
        after_help = "Examples:\n  td show abc123\n  \
            td show https://app.todoist.com/app/task/buy-milk-8Jx4mVr72kPn3QwB"
    )]
    Show {
        /// Task ID or Todoist task URL
        id: String,
    },

    /// Edit an existing task
    #[command(
        long_about = "Update one or more fields on an existing task.\n\n\
            Only specified fields are updated; others remain unchanged.\n\
            Labels can be added or removed incrementally.",
        after_help = "Examples:\n  td edit abc123 --content \"Updated title\"\n  \
            td edit abc123 --due \"next week\" --priority 1\n  \
            td edit abc123 --add-label urgent --remove-label later\n  \
            td edit abc123 --no-due                    # Remove due date"
    )]
    Edit {
        /// Task ID to edit
        id: String,

        /// New task content
        #[arg(long)]
        content: Option<String>,

        /// New due date (natural language or ISO 8601)
        #[arg(long)]
        due: Option<String>,

        /// New priority (1-4)
        #[arg(long, value_parser = clap::value_parser!(i32).range(1..=4))]
        priority: Option<i32>,

        /// Add label(s) to the task (repeatable)
        #[arg(long)]
        add_label: Vec<String>,

        /// Remove label(s) from the task (repeatable)
        #[arg(long)]
        remove_label: Vec<String>,

        /// New description / notes
        #[arg(long)]
        description: Option<String>,

        /// Remove the due date
        #[arg(long)]
        no_due: bool,
    },

    /// Reopen a completed task
    #[command(
        long_about = "Reopen a previously completed task, making it active again.",
        after_help = "Examples:\n  td reopen abc123"
    )]
    Reopen {
        /// Task ID to reopen
        id: String,
    },

    /// Move a task to a different project, section, or parent
    #[command(
        long_about = "Move a task to a different project, section, or make it a subtask of another task.\n\n\
            Project and section names are resolved automatically from the cache.",
        after_help = "Examples:\n  td move abc123 --project Personal\n  \
            td move abc123 --section \"In Progress\"\n  \
            td move abc123 --parent other_id          # Make subtask\n  \
            td move abc123 --no-parent                # Promote to top-level"
    )]
    Move {
        /// Task ID to move
        id: String,

        /// Target project name or ID
        #[arg(long)]
        project: Option<String>,

        /// Target section name or ID
        #[arg(long)]
        section: Option<String>,

        /// Parent task ID (make subtask)
        #[arg(long)]
        parent: Option<String>,

        /// Remove parent (promote to top-level task)
        #[arg(long)]
        no_parent: bool,
    },

    /// Manage projects (list, show, add, edit, archive, delete)
    #[command(
        long_about = "Create, list, edit, archive, and delete Todoist projects.",
        after_help = "Examples:\n  td project list\n  td project add \"New Project\" --color blue\n  \
            td project edit <ID> --name \"Renamed\"\n  td project archive <ID>\n  \
            td project delete <ID> --yes"
    )]
    Project {
        #[command(subcommand)]
        action: ProjectAction,
    },

    /// Manage sections (list, add, edit, move, delete)
    #[command(
        long_about = "Create, list, edit, reorder, and delete sections within projects.",
        after_help = "Examples:\n  td section list --project Work\n  \
            td section add \"In Progress\" --project Work\n  \
            td section delete <ID> --yes"
    )]
    Section {
        #[command(subcommand)]
        action: SectionAction,
    },

    /// Manage labels (list, add, edit, delete)
    #[command(
        long_about = "Create, list, edit, and delete personal labels.\n\n\
            Labels are referenced by name in the API and can be applied to tasks.",
        after_help = "Examples:\n  td label list\n  td label add urgent --color red\n  \
            td label delete <ID> --yes"
    )]
    Label {
        #[command(subcommand)]
        action: LabelAction,
    },

    /// Manage comments on tasks and projects
    #[command(
        long_about = "List, add, edit, and delete comments on tasks or projects.",
        after_help = "Examples:\n  td comment list --task <TASK_ID>\n  \
            td comment add --task <TASK_ID> \"Looks good!\"\n  \
            td comment edit <COMMENT_ID> \"Updated text\"\n  \
            td comment delete <COMMENT_ID>"
    )]
    Comment {
        #[command(subcommand)]
        action: CommentAction,
    },

    /// Manage reminders for tasks
    #[command(
        long_about = "List, add, and delete reminders on tasks.",
        after_help = "Examples:\n  td reminder list --task <TASK_ID>\n  \
            td reminder add --task <TASK_ID> --due \"2026-03-15T09:00:00\"\n  \
            td reminder delete <REMINDER_ID>"
    )]
    Reminder {
        #[command(subcommand)]
        action: ReminderAction,
    },

    /// Manage saved filters
    #[command(
        long_about = "Create, list, edit, and delete saved filters.\n\n\
            Saved filters are stored on the Todoist server and can be used across all clients.",
        after_help = "Examples:\n  td filter list\n  \
            td filter add \"Work Today\" --query \"today & #Work\"\n  \
            td filter show <ID>"
    )]
    Filter {
        #[command(subcommand)]
        action: FilterAction,
    },

    /// View the activity log
    #[command(
        long_about = "View recent activity events from Todoist.\n\n\
            Shows actions like task completions, additions, and edits. Can be filtered\n\
            by event type, project, and date range.",
        after_help = "Examples:\n  td activity\n  td activity --limit 50\n  \
            td activity --event-type \"task:completed\"\n  \
            td activity --project <PROJECT_ID> --since 2026-03-01"
    )]
    Activity {
        /// Maximum number of events to show
        #[arg(long, default_value = "20")]
        limit: usize,

        /// Filter by event type (e.g. "task:completed", "task:added")
        #[arg(long)]
        event_type: Option<String>,

        /// Filter by project name or ID
        #[arg(long)]
        project: Option<String>,

        /// Show events since this date (ISO 8601)
        #[arg(long)]
        since: Option<String>,
    },

    /// Authentication commands (login, logout, status)
    #[command(
        long_about = "Manage Todoist API authentication.\n\n\
            Supports interactive login, direct token setting, multi-profile switching,\n\
            and credential management.",
        after_help = "Examples:\n  td auth login          # Interactive setup wizard\n  \
            td auth token <TOKEN>  # Set token directly\n  \
            td auth status         # Show current auth status\n  \
            td auth logout         # Remove stored credentials\n  \
            td auth switch work    # Switch to 'work' profile"
    )]
    Auth {
        #[command(subcommand)]
        action: AuthAction,
    },

    /// Generate shell completions for bash, zsh, fish, or powershell
    #[command(
        long_about = "Generate shell completion scripts for the specified shell.\n\n\
            Output the completion script to stdout; redirect to the appropriate file\n\
            for your shell.",
        after_help = "Examples:\n  td completions bash > ~/.local/share/bash-completion/completions/td\n  \
            td completions zsh > ~/.zfunc/_td\n  \
            td completions fish > ~/.config/fish/completions/td.fish\n  \
            td completions powershell > _td.ps1"
    )]
    Completions {
        /// Shell to generate completions for
        shell: clap_complete::Shell,
    },
}

// -- Project subcommands --

#[derive(Debug, Subcommand)]
pub enum ProjectAction {
    /// List all projects
    #[command(after_help = "Examples:\n  td project list\n  td project list --archived")]
    List {
        /// Include archived projects
        #[arg(long)]
        archived: bool,
    },

    /// Show details for a project
    #[command(after_help = "Examples:\n  td project show <PROJECT_ID>")]
    Show {
        /// Project ID
        id: String,
    },

    /// Create a new project
    #[command(after_help = "Examples:\n  td project add \"New Project\"\n  \
        td project add \"Sub\" --parent <PARENT_ID> --color blue --view board")]
    Add {
        /// Project name
        name: String,

        /// Parent project ID (create as sub-project)
        #[arg(long)]
        parent: Option<String>,

        /// Project color (e.g. red, blue, green)
        #[arg(long)]
        color: Option<String>,

        /// View style: list or board
        #[arg(long)]
        view: Option<String>,
    },

    /// Edit an existing project
    #[command(
        after_help = "Examples:\n  td project edit <ID> --name \"Renamed\" --color red\n  \
        td project edit <ID> --favorite"
    )]
    Edit {
        /// Project ID
        id: String,

        /// New project name
        #[arg(long)]
        name: Option<String>,

        /// New project color
        #[arg(long)]
        color: Option<String>,

        /// Toggle favorite status
        #[arg(long)]
        favorite: bool,
    },

    /// Archive a project
    #[command(after_help = "Examples:\n  td project archive <PROJECT_ID>")]
    Archive {
        /// Project ID
        id: String,
    },

    /// Unarchive a project
    #[command(after_help = "Examples:\n  td project unarchive <PROJECT_ID>")]
    Unarchive {
        /// Project ID
        id: String,
    },

    /// Delete a project permanently
    #[command(after_help = "Examples:\n  td project delete <PROJECT_ID>\n  \
        td project delete <PROJECT_ID> --yes")]
    Delete {
        /// Project ID
        id: String,

        /// Skip confirmation prompt
        #[arg(long)]
        yes: bool,
    },

    /// List collaborators on a shared project
    #[command(after_help = "Examples:\n  td project collaborators <PROJECT_ID>")]
    Collaborators {
        /// Project ID
        id: String,
    },
}

// -- Section subcommands --

#[derive(Debug, Subcommand)]
pub enum SectionAction {
    /// List sections
    #[command(after_help = "Examples:\n  td section list\n  td section list --project Work")]
    List {
        /// Filter by project name or ID
        #[arg(long)]
        project: Option<String>,
    },

    /// Create a new section in a project
    #[command(after_help = "Examples:\n  td section add \"In Progress\" --project Work")]
    Add {
        /// Section name
        name: String,

        /// Project name or ID (required)
        #[arg(long, required = true)]
        project: String,
    },

    /// Rename a section
    #[command(after_help = "Examples:\n  td section edit <ID> --name \"Done\"")]
    Edit {
        /// Section ID
        id: String,

        /// New section name
        #[arg(long)]
        name: Option<String>,
    },

    /// Reorder a section within its project
    #[command(after_help = "Examples:\n  td section move <ID> --order 3")]
    Move {
        /// Section ID
        id: String,

        /// New position order
        #[arg(long)]
        order: i32,
    },

    /// Delete a section permanently
    #[command(after_help = "Examples:\n  td section delete <ID> --yes")]
    Delete {
        /// Section ID
        id: String,

        /// Skip confirmation prompt
        #[arg(long)]
        yes: bool,
    },
}

// -- Label subcommands --

#[derive(Debug, Subcommand)]
pub enum LabelAction {
    /// List all labels
    List,

    /// Create a new label
    #[command(
        after_help = "Examples:\n  td label add urgent\n  td label add \"context/home\" --color red"
    )]
    Add {
        /// Label name
        name: String,

        /// Label color
        #[arg(long)]
        color: Option<String>,
    },

    /// Edit an existing label
    #[command(after_help = "Examples:\n  td label edit <ID> --name important --color blue")]
    Edit {
        /// Label ID
        id: String,

        /// New label name
        #[arg(long)]
        name: Option<String>,

        /// New label color
        #[arg(long)]
        color: Option<String>,
    },

    /// Delete a label permanently
    #[command(after_help = "Examples:\n  td label delete <ID> --yes")]
    Delete {
        /// Label ID
        id: String,

        /// Skip confirmation prompt
        #[arg(long)]
        yes: bool,
    },
}

// -- Comment subcommands --

#[derive(Debug, Subcommand)]
pub enum CommentAction {
    /// List comments on a task or project
    #[command(
        long_about = "List comments. Provide either --task or --project to filter.",
        after_help = "Examples:\n  td comment list --task <TASK_ID>\n  td comment list --project <PROJECT_ID>"
    )]
    List {
        /// Filter by task ID
        #[arg(long)]
        task: Option<String>,

        /// Filter by project ID
        #[arg(long)]
        project: Option<String>,
    },

    /// Add a comment to a task or project
    #[command(
        after_help = "Examples:\n  td comment add --task <TASK_ID> \"Looks good!\"\n  \
        td comment add --project <PROJECT_ID> \"Project note\""
    )]
    Add {
        /// Task ID to comment on
        #[arg(long)]
        task: Option<String>,

        /// Project ID to comment on
        #[arg(long)]
        project: Option<String>,

        /// Comment text
        content: String,
    },

    /// Edit a comment
    #[command(after_help = "Examples:\n  td comment edit <COMMENT_ID> \"Updated text\"")]
    Edit {
        /// Comment ID
        id: String,

        /// New comment text
        content: String,
    },

    /// Delete a comment
    #[command(after_help = "Examples:\n  td comment delete <COMMENT_ID>")]
    Delete {
        /// Comment ID
        id: String,
    },
}

// -- Reminder subcommands --

#[derive(Debug, Subcommand)]
pub enum ReminderAction {
    /// List reminders for a task
    #[command(after_help = "Examples:\n  td reminder list --task <TASK_ID>")]
    List {
        /// Task ID
        #[arg(long)]
        task: String,
    },

    /// Add a reminder to a task
    #[command(
        after_help = "Examples:\n  td reminder add --task <TASK_ID> --due \"2026-03-15T09:00:00\"\n  \
        td reminder add --task <TASK_ID> --relative 30"
    )]
    Add {
        /// Task ID
        #[arg(long)]
        task: String,

        /// Absolute due date/time (ISO 8601)
        #[arg(long)]
        due: Option<String>,

        /// Minutes before due date to trigger reminder
        #[arg(long)]
        relative: Option<i32>,
    },

    /// Delete a reminder
    #[command(after_help = "Examples:\n  td reminder delete <REMINDER_ID>")]
    Delete {
        /// Reminder ID
        id: String,
    },
}

// -- Filter subcommands --

#[derive(Debug, Subcommand)]
pub enum FilterAction {
    /// List all saved filters
    List,

    /// Show details for a saved filter
    #[command(after_help = "Examples:\n  td filter show <FILTER_ID>")]
    Show {
        /// Filter ID
        id: String,
    },

    /// Create a new saved filter
    #[command(
        after_help = "Examples:\n  td filter add \"Work Today\" --query \"today & #Work\" --color blue"
    )]
    Add {
        /// Filter name
        name: String,

        /// Filter query expression
        #[arg(long, required = true)]
        query: String,

        /// Filter color
        #[arg(long)]
        color: Option<String>,
    },

    /// Edit a saved filter
    #[command(
        after_help = "Examples:\n  td filter edit <ID> --name \"New Name\" --query \"overdue & p1\""
    )]
    Edit {
        /// Filter ID
        id: String,

        /// New filter name
        #[arg(long)]
        name: Option<String>,

        /// New filter query expression
        #[arg(long)]
        query: Option<String>,

        /// New filter color
        #[arg(long)]
        color: Option<String>,
    },

    /// Delete a saved filter
    #[command(after_help = "Examples:\n  td filter delete <FILTER_ID>")]
    Delete {
        /// Filter ID
        id: String,
    },
}

// -- Auth subcommands --

#[derive(Debug, Subcommand)]
pub enum AuthAction {
    /// Interactive setup wizard (validate token, store, initial sync)
    Login,

    /// Set API token directly
    #[command(after_help = "Examples:\n  td auth token <YOUR_API_TOKEN>")]
    Token {
        /// Todoist API token
        token: String,
    },

    /// Show current authentication status and user info
    Status,

    /// Remove stored credentials
    Logout,

    /// Switch the active named profile
    #[command(after_help = "Examples:\n  td auth switch work\n  td auth switch personal")]
    Switch {
        /// Profile name (as defined in config.toml)
        profile: String,
    },
}
