# Product Requirements Document: `td` — Todoist CLI in Rust

**Version:** 1.0  
**Date:** 2 March 2026  
**Author:** [Your Name]  
**Status:** Draft  

---

## 1. Executive Summary

`td` is a fast, offline-capable, feature-complete command-line interface for Todoist written in Rust. It targets the new unified Todoist API v1.0, replacing abandoned and deprecated CLI tools in the ecosystem. The tool is designed for power users, developers, and AI agents who want to manage their Todoist tasks entirely from the terminal with zero compromises on speed, reliability, or feature coverage.

### Why This Exists

The existing Todoist CLI landscape is fragmented and broken:

- **sachaos/todoist** (Go, 1.6k stars): Abandoned. Uses deprecated Sync API v8/v9. 64 open issues including crashes (SIGSEGV, panics), no section support, no comment support, broken on Windows. The old APIs were shut down in February 2026.
- **Doist/todoist-cli** (Official, TypeScript/Node.js): Active but requires Node.js runtime. No offline cache, no filter expressions, limited scriptability. Heavy dependency footprint.
- **chaosteil/doist** (Rust): Minimal feature set. Uses deprecated REST v2 API. No sections, labels, comments, reminders.
- **todoist-cli-rs** (Rust): Newer but still early, limited community adoption.

This project aims to be the definitive Todoist CLI — a single, statically-compiled binary with offline-first architecture, full API v1 coverage, and first-class support for both human operators and AI agents.

---

## 2. Goals & Non-Goals

### Goals

1. **Full Todoist API v1 coverage** — every resource the API exposes should be manageable from the CLI
2. **Offline-first architecture** — local cache for instant reads, explicit sync control
3. **Zero runtime dependencies** — single static binary, no Node.js/Python/Go required
4. **Scriptability** — JSON output, quiet mode, exit codes, stdin support for piping
5. **Human-friendly** — colored output, interactive fuzzy selection, natural language dates, shell completions
6. **Agent-friendly** — structured JSON output with optional HATEOAS-style `next_actions` hints for AI tooling
7. **Cross-platform** — macOS (ARM + Intel), Linux (x64, ARM), Windows (x64)
8. **Robust error handling** — no panics, no crashes, graceful degradation on network failure
9. **Secure credential storage** — OS keyring integration with fallbacks

### Non-Goals

- Full TUI application (this is a CLI, not a terminal UI like `taskwarrior-tui`)
- Mobile support
- Web interface or GUI
- Real-time push notifications (webhooks are for server-side integrations)
- Replacing the Todoist web/mobile app for non-technical users

---

## 3. Target Users

| Persona | Description | Key Need |
|---------|-------------|----------|
| **Power User** | Keyboard-driven developer who lives in the terminal | Speed, filter expressions, shell integration |
| **Scripter/Automator** | Writes shell scripts, cron jobs, CI/CD pipelines that interact with Todoist | JSON output, exit codes, quiet mode, stdin |
| **AI Agent** | Claude Code, Cursor, or custom LLM agent managing tasks programmatically | Structured JSON, predictable output, MCP skill file |
| **Multi-account User** | Uses Todoist for both work and personal | Profile switching, environment variable overrides |

---

## 4. Technical Architecture

### 4.1 Crate Structure (Cargo Workspace)

```
todoist-cli/
├── Cargo.toml              # Workspace root
├── crates/
│   ├── td-cli/             # Binary crate — CLI entry point, argument parsing, output formatting
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── commands/   # One module per command group (task, project, section, etc.)
│   │   │   ├── output/     # Formatters: table, json, csv
│   │   │   ├── interactive/ # Fuzzy picker, prompts
│   │   │   └── config.rs
│   │   └── Cargo.toml
│   ├── td-api/             # Library crate — Todoist API v1 client
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── client.rs   # HTTP client, auth, rate limiting, retries
│   │   │   ├── models/     # Serde models for all API resources
│   │   │   ├── endpoints/  # One module per API resource
│   │   │   └── error.rs    # Typed error hierarchy
│   │   └── Cargo.toml
│   └── td-cache/           # Library crate — Local cache, sync engine, filter parser
│       ├── src/
│       │   ├── lib.rs
│       │   ├── cache.rs    # SQLite-backed local cache
│       │   ├── sync.rs     # Incremental + full sync logic
│       │   ├── filter/     # Filter expression lexer, parser, evaluator
│       │   └── nlp.rs      # Natural language date parsing
│       └── Cargo.toml
├── skills/                 # AI agent skill files
│   └── todoist/
│       └── SKILL.md        # Claude Code / MCP skill definition
├── completions/            # Generated shell completion scripts
├── tests/                  # Integration tests
└── README.md
```

### 4.2 Key Dependencies (Suggested)

| Crate | Purpose |
|-------|---------|
| `clap` v4 | Argument parsing with derive macros, shell completions |
| `reqwest` | HTTP client (with rustls for no OpenSSL dependency) |
| `serde` / `serde_json` | Serialization |
| `rusqlite` | Local SQLite cache (bundled, no system dependency) |
| `keyring` | OS keyring integration (macOS Keychain, Windows Credential Manager, Linux Secret Service) |
| `chrono` | Date/time handling |
| `skim` or `nucleo` | Fuzzy finder for interactive selection |
| `comfy-table` or `tabled` | Terminal table rendering |
| `colored` / `owo-colors` | Terminal color output |
| `tokio` | Async runtime (single-threaded for CLI) |
| `indicatif` | Progress bars/spinners for sync operations |
| `directories` | XDG-compliant config/cache paths |
| `pest` or `winnow` | Filter expression parser |
| `thiserror` / `anyhow` | Error handling |
| `tracing` | Structured logging for debug mode |

### 4.3 Build & Distribution

- **Binary name:** `td`
- **MSRV:** Latest stable Rust (1.83+)
- **Static linking:** Use `musl` target for Linux to produce fully static binaries
- **Release artifacts:** GitHub Releases with pre-built binaries for all platforms
- **Package managers:** Homebrew tap, Cargo install, AUR package, Nix flake
- **CI/CD:** GitHub Actions for cross-compilation, testing, and release automation

---

## 5. Authentication & Configuration

### 5.1 Authentication Methods (Priority Order)

1. **Environment variable:** `TODOIST_API_TOKEN` — highest priority, for scripts/CI
2. **OS Keyring:** Secure storage via `keyring` crate — recommended for interactive use
3. **Config file:** `~/.config/td/config.toml` — fallback for systems without keyring
4. **CLI flag:** `--token <TOKEN>` — one-off override

### 5.2 Configuration File

Location: `$XDG_CONFIG_HOME/td/config.toml` (default: `~/.config/td/config.toml`)

```toml
# Default profile
[default]
token_source = "keyring"  # "keyring" | "config" | "env"
# token = "xxx"           # Only if token_source = "config"

# Display preferences
color = "auto"            # "auto" | "always" | "never"
date_format = "%Y-%m-%d"
time_format = "%H:%M"
default_project = "Inbox"
default_priority = 4

# Sync preferences
auto_sync_on_write = true     # Auto-sync cache after mutations
sync_timeout_secs = 30

# Output
default_output = "table"      # "table" | "json" | "csv"
table_columns = ["id", "priority", "content", "project", "due", "labels"]

# Named profiles for multi-account
[profiles.work]
token_source = "keyring"
keyring_entry = "td-work"

[profiles.personal]
token_source = "keyring"
keyring_entry = "td-personal"
```

### 5.3 First-Run Setup Wizard

On first invocation without a configured token, launch an interactive setup:

1. Prompt for API token (with link to Todoist settings page)
2. Validate token against the API
3. Ask preferred storage method (keyring / config file / env)
4. Store token securely
5. Perform initial full sync
6. Display task count summary

### 5.4 Auth Commands

```
td auth login           # Interactive setup wizard
td auth token <TOKEN>   # Set token directly
td auth status          # Show current auth status and user info
td auth logout          # Remove stored credentials
td auth switch <PROFILE> # Switch named profile
```

---

## 6. Command Reference

### 6.1 Global Flags

| Flag | Short | Description |
|------|-------|-------------|
| `--sync` | `-s` | Sync with Todoist before executing the command |
| `--json` | `-j` | Force JSON output |
| `--csv` | | Force CSV output |
| `--quiet` | `-q` | Suppress output (errors only), return only IDs on creation |
| `--verbose` | `-v` | Debug logging |
| `--no-color` | | Disable colored output |
| `--token <TOKEN>` | | Override API token for this invocation |
| `--profile <NAME>` | `-p` | Use named profile |
| `--accessible` | | Add text prefixes to color-coded elements for screen readers |

### 6.2 Task Commands

#### `td list` — List Tasks

```
td list                           # All active tasks (default limit: 50)
td list --all                     # All tasks, no limit
td list -f "today & p1"           # Filter expression
td list -f "overdue | today"      # Boolean filter
td list --project "Work"          # Filter by project name
td list --label "urgent"          # Filter by label
td list --section "In Progress"   # Filter by section
td list --assigned-to me          # Assigned to current user
td list --assigned-to "Alice"     # Assigned to collaborator
td list --search "keyword"        # Full-text search in task content and description
td list --sort due                # Sort by: due, priority, created, updated, manual
td list --group project           # Group by: project, section, priority, label, due
td list --tree                    # Show subtask hierarchy as tree
td list --limit 10                # Limit results
td list --columns id,content,due  # Custom column selection
```

**Output columns (configurable):** ID, Priority, Content, Project, Section, Due Date, Labels, Assignee, Created, Description (truncated)

#### `td today` — Today's Agenda

```
td today                          # Today + overdue
td today --no-overdue             # Just today
td today --upcoming 3             # Include next N days
td today --group priority         # Group by priority
```

#### `td inbox` — Inbox Tasks

```
td inbox                          # Tasks in the Inbox project
```

#### `td add` — Add Task

```
td add "Buy groceries"
td add "Review PR" --project "Work" --priority 1 --due "tomorrow"
td add "Fix bug" --section "In Progress" --label "urgent" --label "backend"
td add "Team meeting" --due "every monday at 10am" --duration 60
td add "Deploy v2" --deadline "2026-03-15"
td add "Pair review" --project "Shared" --assign "Alice"
td add "Research" --description "Look into the new API changes"
td add "Sub-item" --parent <TASK_ID>
td add "Read from stdin" --stdin                    # Read content from stdin
echo "Task from pipe" | td add --stdin
```

**Flags:**

| Flag | Short | Description |
|------|-------|-------------|
| `--project` | `-p` | Project name or ID |
| `--section` | `-S` | Section name or ID |
| `--priority` | `-P` | 1 (urgent) to 4 (default) |
| `--due` | `-d` | Due date (natural language or ISO 8601) |
| `--deadline` | | Deadline date |
| `--duration` | | Duration in minutes |
| `--label` | `-l` | Label name (repeatable) |
| `--assign` | `-a` | Assignee name |
| `--description` | `-D` | Task description/notes |
| `--parent` | | Parent task ID for subtasks |
| `--order` | | Position order within project/section |
| `--stdin` | | Read task content from stdin |

**On success:** Print the created task ID (in quiet mode) or full task details.

#### `td quick` — Quick Add (Natural Language)

```
td quick "Call mom tomorrow at 5pm #Personal @important p1"
td quick "Submit report every Friday #Work"
td quick "Buy milk /Groceries"    # /Section syntax
```

Uses Todoist's server-side natural language parsing. Supports `#Project`, `@Label`, `pN` priority, `/Section` and date expressions.

#### `td show` — Show Task Details

```
td show <TASK_ID>
td show <TASK_ID> --comments      # Include comments
td show <TASK_ID> --reminders     # Include reminders
td show <TASK_ID> --subtasks      # Include subtasks
td show <TASK_URL>                # Accept Todoist task URLs
```

#### `td edit` — Edit Task

```
td edit <TASK_ID> --content "New content"
td edit <TASK_ID> --due "next week"
td edit <TASK_ID> --priority 1
td edit <TASK_ID> --add-label "urgent"
td edit <TASK_ID> --remove-label "later"
td edit <TASK_ID> --project "Personal"      # Move to project
td edit <TASK_ID> --section "Done"          # Move to section
td edit <TASK_ID> --no-due                  # Remove due date
td edit <TASK_ID> --assign "Bob"            # Assign to collaborator
td edit <TASK_ID> --unassign                # Remove assignment
td edit <TASK_ID> --description "Updated"   # Set description
td edit <TASK_ID> --parent <OTHER_ID>       # Make subtask of another task
```

#### `td done` — Complete Tasks

```
td done <TASK_ID>
td done <ID1> <ID2> <ID3>                  # Batch complete
td done <TASK_ID> --all-occurrences         # Complete recurring task permanently
td list -f "overdue" --json | jq -r '.[].id' | xargs td done   # Pipe pattern
```

#### `td reopen` — Reopen Completed Tasks

```
td reopen <TASK_ID>
```

#### `td delete` — Delete Tasks

```
td delete <TASK_ID>
td delete <ID1> <ID2> <ID3>               # Batch delete
td delete <TASK_ID> --yes                   # Skip confirmation prompt
```

#### `td move` — Move Tasks

```
td move <TASK_ID> --project "Personal"
td move <TASK_ID> --section "In Progress"
td move <TASK_ID> --parent <OTHER_ID>       # Make subtask
td move <TASK_ID> --no-parent               # Promote to top-level
```

### 6.3 Project Commands

```
td project list                             # List all projects
td project list --archived                  # Include archived projects
td project show <ID>                        # Show project details
td project add "New Project"                # Create project
td project add "Sub" --parent "Parent"      # Create sub-project
td project add "Work" --color blue --view board  # With options
td project edit <ID> --name "Renamed"
td project edit <ID> --color red
td project edit <ID> --favorite             # Toggle favorite
td project archive <ID>
td project unarchive <ID>
td project delete <ID> --yes
td project collaborators <ID>               # List collaborators
```

### 6.4 Section Commands

```
td section list                             # All sections
td section list --project "Work"            # Sections in project
td section add "In Progress" --project "Work"
td section edit <ID> --name "Done"
td section move <ID> --order 3              # Reorder
td section delete <ID> --yes
```

### 6.5 Label Commands

```
td label list
td label add "urgent"
td label add "context/home" --color red
td label edit <ID> --name "important"
td label delete <ID> --yes
```

### 6.6 Comment Commands

```
td comment list --task <TASK_ID>
td comment list --project <PROJECT_ID>
td comment add --task <TASK_ID> "Comment text"
td comment add --task <TASK_ID> --stdin       # Read from stdin (for piping)
td comment add --task <TASK_ID> --file /path/to/attachment
td comment edit <COMMENT_ID> "Updated text"
td comment delete <COMMENT_ID>
```

### 6.7 Reminder Commands

```
td reminder list --task <TASK_ID>
td reminder add --task <TASK_ID> --due "2026-03-15T09:00:00"
td reminder add --task <TASK_ID> --relative 30  # 30 minutes before due
td reminder delete <REMINDER_ID>
```

### 6.8 Activity Log

```
td activity                                  # Recent activity
td activity --limit 50
td activity --event-type "task:completed"
td activity --project <PROJECT_ID>
td activity --since "2026-03-01"
```

### 6.9 Saved Filters

```
td filter list
td filter add "Work Today" --query "today & #Work"
td filter show <ID>
td filter edit <ID> --name "New Name" --query "overdue & p1"
td filter delete <ID>
```

### 6.10 Sync Commands

```
td sync                                     # Incremental sync
td sync --full                              # Full sync (rebuild cache)
td sync --status                            # Show sync status (last sync time, cache stats)
```

### 6.11 Configuration Commands

```
td config show                              # Display current config
td config edit                              # Open in $EDITOR
td config set <KEY> <VALUE>                 # Set config value
td config path                              # Print config file path
td config init                              # Create default config file
```

### 6.12 Backup / Export

```
td backup                                   # Export all data as JSON
td backup --format json                     # JSON export (default)
td backup --format csv                      # CSV export
td backup --completed                       # Include completed tasks
td backup --output ./backup.json            # Specify output file
```

### 6.13 Shell Completions

```
td completions bash > ~/.local/share/bash-completion/completions/td
td completions zsh > ~/.zfunc/_td
td completions fish > ~/.config/fish/completions/td.fish
td completions powershell > _td.ps1
```

### 6.14 Interactive Mode

```
td pick                                     # Fuzzy-select a task, then choose action
td pick --filter "today"                    # Fuzzy-select from filtered tasks
td pick --project "Work"                    # Fuzzy-select from project
```

When a task is selected via fuzzy finder, present an action menu:
- View details
- Complete
- Edit content / due date / priority / labels
- Move to project/section
- Add comment
- Delete
- Open in browser

---

## 7. Filter Expression Engine

Implement a full filter expression parser compatible with Todoist's filter syntax. This was the most-requested missing feature across all existing CLI tools.

### 7.1 Supported Filters

#### Date Filters
| Expression | Description |
|-----------|-------------|
| `today` | Due today |
| `tomorrow` | Due tomorrow |
| `yesterday` | Due yesterday |
| `overdue` | Past due date |
| `N days` / `next N days` | Due within next N days |
| `no date` | No due date set |
| `Jan 15` / `January 15` | Specific date |
| `before:2026-03-15` | Due before date |
| `after:2026-03-01` | Due after date |
| `recurring` | Recurring tasks |

#### Priority Filters
| Expression | Description |
|-----------|-------------|
| `p1` | Priority 1 (urgent) |
| `p2` | Priority 2 (high) |
| `p3` | Priority 3 (medium) |
| `p4` | Priority 4 (default) |

#### Label Filters
| Expression | Description |
|-----------|-------------|
| `@label_name` | Has label |
| `no labels` | No labels assigned |

#### Project & Section Filters
| Expression | Description |
|-----------|-------------|
| `#Project` | Exact project match |
| `##Project` | Project and all sub-projects |
| `/Section` | Tasks in section |

#### Assignment Filters
| Expression | Description |
|-----------|-------------|
| `assigned to: me` | Assigned to current user |
| `assigned to: other` | Assigned to others |
| `assigned to: "Name"` | Assigned to specific person |
| `assigned by: me` | You assigned to someone |
| `assigned` | Has any assignee |
| `!assigned` | Unassigned |

#### Text Search
| Expression | Description |
|-----------|-------------|
| `search: keyword` | Full-text search in content and description |

#### Boolean Operators
| Operator | Description |
|----------|-------------|
| `&` | AND |
| `\|` | OR |
| `!` | NOT |
| `( )` | Grouping |

### 7.2 Examples

```
td list -f "p1 & (today | overdue)"
td list -f "#Work & no date"
td list -f "@urgent & !#Archive"
td list -f "7 days & ##Work"
td list -f "no labels & overdue"
td list -f "search: deploy & p1"
td list -f "assigned to: me & today"
td list -f "recurring & #Personal"
```

### 7.3 Implementation Notes

- Use `pest` or `winnow` for the parser
- Filters run against the local cache for instant evaluation
- Support both local-only evaluation and API-delegated filters where appropriate
- Provide helpful error messages on invalid filter syntax with suggestions

---

## 8. Sync & Cache Architecture

### 8.1 Cache Storage

Use SQLite (via `rusqlite` with bundled SQLite) for the local cache. This provides:
- Atomic operations
- Efficient queries for filter evaluation
- Schema migrations for upgrades
- Corruption resistance vs flat JSON files

**Cache location:** `$XDG_CACHE_HOME/td/cache.db` (default: `~/.cache/td/cache.db`)

### 8.2 Cache Schema

Tables for: `tasks`, `projects`, `sections`, `labels`, `reminders`, `collaborators`, `filters`, `sync_state`

The `sync_state` table stores the sync token for incremental sync.

### 8.3 Sync Behavior

| Operation | Network Required | Cache Updated |
|-----------|-----------------|---------------|
| `td list`, `td today`, `td show` | No (reads from cache) | No |
| `td list --sync`, `td today --sync` | Yes (sync first) | Yes |
| `td sync` | Yes | Yes |
| `td add`, `td edit`, `td done`, `td delete` | Yes (API call) | Yes (optimistic update) |
| `td quick` | Yes (server-side NLP) | Yes |

### 8.4 Sync Strategy

1. **Incremental sync (default):** Send sync token, receive only changes since last sync. Fast.
2. **Full sync:** Rebuild cache from scratch. Use when cache is corrupted or on explicit `--full`.
3. **Optimistic cache update:** After write operations, update local cache immediately with the API response rather than requiring a full re-sync.
4. **Conflict handling:** API is source of truth. On sync, remote state overwrites local cache.

### 8.5 Offline Resilience

- All read operations work offline using cached data
- Write operations queue locally if offline, with `td sync --push` to push pending changes
- Display a warning when showing potentially stale data (last sync > configurable threshold)
- Show `[offline]` indicator in output when network is unavailable

---

## 9. Output Formatting

### 9.1 Output Modes

| Mode | When | Description |
|------|------|-------------|
| **Table** | TTY (default) | Colored, aligned table with Unicode borders |
| **JSON** | Non-TTY (pipe) or `--json` | Machine-readable JSON array |
| **CSV** | `--csv` | RFC 4180 compliant CSV |
| **Quiet** | `--quiet` | IDs only on creation, nothing on success, errors on stderr |

### 9.2 Table Output Features

- Color-coded priorities (p1=red, p2=orange, p3=yellow, p4=default)
- Color-coded due dates (overdue=red, today=green, tomorrow=yellow)
- Truncated content with `...` for long text
- Tree-style indentation for subtasks (when `--tree` flag is used)
- Nerd Font icon support (opt-in via config)
- Configurable column selection and ordering
- Accessible mode with text labels instead of color-only indicators

### 9.3 JSON Output Schema

```json
{
  "tasks": [
    {
      "id": "6X4Vw2Hfmg73Q2XR",
      "content": "Buy groceries",
      "description": "",
      "project": { "id": "abc", "name": "Personal" },
      "section": null,
      "priority": 4,
      "due": { "date": "2026-03-03", "is_recurring": false, "string": "tomorrow" },
      "labels": ["shopping"],
      "assignee": null,
      "parent_id": null,
      "order": 1,
      "created_at": "2026-03-02T13:00:00Z",
      "url": "https://app.todoist.com/app/task/6X4Vw2Hfmg73Q2XR"
    }
  ],
  "meta": {
    "total": 1,
    "synced_at": "2026-03-02T13:30:00Z"
  }
}
```

### 9.4 Agent Mode (Optional `--agent` flag)

When invoked with `--agent`, append HATEOAS-style `next_actions` hints to JSON output:

```json
{
  "tasks": [...],
  "next_actions": [
    { "action": "complete", "command": "td done <id>" },
    { "action": "edit", "command": "td edit <id> --due 'tomorrow'" },
    { "action": "add_comment", "command": "td comment add --task <id> 'text'" }
  ]
}
```

---

## 10. Error Handling & Resilience

### 10.1 Error Categories

| Category | Behavior |
|----------|----------|
| **Auth errors** (401) | Clear message: "Token invalid or expired. Run `td auth login` to re-authenticate." |
| **Rate limit** (429) | Automatic exponential backoff with retry. Display wait time. |
| **Network errors** | Graceful fallback to cache for reads. Queue writes for later sync. |
| **Not found** (404) | "Task/project not found. Run `td sync` to refresh." with fuzzy suggestions. |
| **Validation errors** | Show the specific field and constraint that failed. |
| **Cache corruption** | Auto-detect and prompt `td sync --full`. |

### 10.2 Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | General error |
| `2` | Auth error |
| `3` | Network error (offline) |
| `4` | Not found |
| `5` | Validation error |
| `10` | Cache error |

### 10.3 Smart Suggestions

When a user provides a project name, label name, or section name that doesn't match:
- Use Levenshtein distance to suggest closest matches
- Example: `Did you mean "Work" instead of "Wrk"?`
- Apply to all name-based lookups (projects, labels, sections, assignees)

---

## 11. Reference Resolution

Support multiple ways to reference Todoist objects, matching the official CLI's flexibility:

| Input | Resolution |
|-------|------------|
| `6X4Vw2Hfmg73Q2XR` | Direct ID |
| `id:6X4Vw2Hfmg73Q2XR` | Explicit ID prefix |
| `https://app.todoist.com/app/task/buy-milk-8Jx4mVr72kPn3QwB` | Todoist URL |
| `"Work"` | Name match (projects, labels, sections) |
| `#Work` | Project by name (in filter context) |
| `@urgent` | Label by name (in filter context) |

---

## 12. AI Agent Integration

### 12.1 Claude Code Skill File

Ship a `skills/todoist/SKILL.md` file that teaches Claude Code how to use the CLI:

```
skills/
└── todoist/
    └── SKILL.md    # Markdown skill definition
```

The skill file should document:
- All commands with examples
- JSON output schemas
- Common workflows (add task, complete task, review today, etc.)
- Error handling patterns

### 12.2 MCP Integration

Provide an optional MCP (Model Context Protocol) server wrapper:
- Expose CLI commands as MCP tools
- Handle auth via environment variables
- Return structured responses for LLM consumption

This can be a separate crate (`td-mcp`) or a feature flag on the main binary.

---

## 13. Natural Language Date Parsing

Support both server-side (via quick-add API) and client-side date parsing:

### Client-Side (for `--due` flag)

| Input | Parsed To |
|-------|-----------|
| `today` | Today's date |
| `tomorrow` / `tom` | Tomorrow |
| `monday` / `mon` | Next Monday |
| `next week` | Next Monday |
| `in 3 days` | 3 days from now |
| `2026-03-15` | ISO 8601 date |
| `Mar 15` | March 15 |
| `every monday` | Recurring: every Monday |
| `every 2 weeks` | Recurring: biweekly |
| `every month on the 1st` | Recurring: monthly |
| `5pm` | Today at 5pm |
| `tomorrow at 9:30` | Tomorrow at 9:30 |

### Server-Side (via `td quick`)

Delegate full natural language parsing to Todoist's server for the quick-add command. This handles complex expressions like "Submit report every other Friday starting March 10".

---

## 14. Multi-Account / Profile Support

```toml
# In config.toml
[profiles.work]
token_source = "keyring"
keyring_entry = "td-work"
default_project = "Work Inbox"

[profiles.personal]
token_source = "keyring"
keyring_entry = "td-personal"
default_project = "Inbox"
```

```bash
td --profile work list               # Use work profile
td --profile personal today           # Use personal profile
td auth switch work                   # Set default profile
export TD_PROFILE=work                # Environment override
```

Each profile has its own cache database to avoid cross-contamination.

---

## 15. Testing Strategy

### 15.1 Unit Tests

- Filter expression parser: extensive test suite covering all filter types, edge cases, boolean combinations
- Date parser: cover all natural language variations
- Cache operations: CRUD against in-memory SQLite
- Output formatters: snapshot tests for table, JSON, CSV output

### 15.2 Integration Tests

- API client: mock server (using `wiremock-rs`) testing all endpoints
- Full command pipeline: test CLI input → API call → cache update → output
- Auth flow: test all credential storage backends
- Sync: test incremental sync, full sync, conflict resolution

### 15.3 End-to-End Tests

- Optional E2E test suite using a real Todoist API token (set via env var, skipped in CI by default)
- Test complete workflows: create project → add task → complete → verify

### 15.4 Property-Based Tests

- Filter parser: use `proptest` to generate random filter expressions and verify parsing doesn't panic
- Sync: test cache consistency under random operation sequences

---

## 16. Milestones & Prioritized Roadmap

### Phase 1: Core (MVP)
**Goal:** Functional CLI with basic task management

- [ ] Project structure, workspace setup, CI/CD pipeline
- [ ] API client: auth, tasks CRUD, projects list, labels list
- [ ] Local SQLite cache with incremental sync
- [ ] Commands: `auth`, `sync`, `list`, `today`, `add`, `quick`, `done`, `delete`, `show`, `edit`
- [ ] Table and JSON output formatting
- [ ] Config file and keyring storage
- [ ] Shell completions (bash, zsh, fish)
- [ ] First-run setup wizard
- [ ] Homebrew formula, cargo install, GitHub Release binaries
- [ ] README and basic documentation

### Phase 2: Full Resource Coverage
**Goal:** Support all Todoist resources

- [ ] Projects: full CRUD, archive/unarchive, favorites, sub-projects
- [ ] Sections: full CRUD, reorder
- [ ] Labels: full CRUD
- [ ] Comments: full CRUD, file attachments
- [ ] Reminders: CRUD
- [ ] Collaborators: list
- [ ] Saved filters: full CRUD
- [ ] Activity log
- [ ] `move` command for tasks
- [ ] Subtask tree view
- [ ] `inbox` command

### Phase 3: Power Features
**Goal:** Match and exceed all existing CLI tools

- [ ] Filter expression engine (full Todoist filter syntax)
- [ ] Full-text search (`search:` filter)
- [ ] Interactive fuzzy picker (`td pick`)
- [ ] Smart suggestions (did you mean?)
- [ ] Natural language date parsing (client-side)
- [ ] Backup/export command
- [ ] Accessible output mode
- [ ] CSV output format
- [ ] `--stdin` support for add/comment
- [ ] Batch operations (pipe multiple IDs)
- [ ] Custom column selection and sorting

### Phase 4: Agent & Ecosystem
**Goal:** First-class AI agent support and ecosystem integration

- [ ] `--agent` mode with HATEOAS JSON
- [ ] Claude Code skill file
- [ ] MCP server wrapper (feature flag or separate crate)
- [ ] Multi-account profile support
- [ ] Offline write queue with `sync --push`
- [ ] Nerd Font icon support (opt-in)
- [ ] PowerShell completions
- [ ] AUR package, Nix flake
- [ ] Man page generation
- [ ] Comprehensive user guide

---

## 17. Competitive Differentiation

| Feature | sachaos/todoist | Doist CLI (official) | todoist-cli-rs | **This Project** |
|---------|:-:|:-:|:-:|:-:|
| API Version | ❌ Deprecated v8/v9 | ✅ v1 | ✅ Sync API | ✅ **v1 (unified)** |
| Language | Go | TypeScript | Rust | **Rust** |
| Runtime Deps | Go binary | Node.js | Rust binary | **None (static)** |
| Offline Cache | JSON file | ❌ | JSON file | **SQLite** |
| Filter Engine | Partial | ❌ | ✅ Full | ✅ **Full + search** |
| Sections | ❌ | ✅ | ✅ | ✅ |
| Comments | ❌ | ✅ | ✅ | ✅ |
| Reminders | ❌ | ✅ | ✅ | ✅ |
| Activity Log | ❌ | ✅ | ❌ | ✅ |
| Backup/Export | ❌ | ❌ | ❌ | ✅ |
| Multi-account | ❌ | ❌ | ❌ | ✅ |
| Agent Mode | ❌ | ❌ | ❌ | ✅ |
| Fuzzy Picker | External (peco/fzf) | ❌ | ❌ | ✅ **Built-in** |
| Smart Suggestions | ❌ | ❌ | ✅ | ✅ |
| Keyring Storage | ❌ | ❌ | ✅ | ✅ |
| Accessibility | ❌ | ✅ | ❌ | ✅ |
| Windows Support | ❌ | ✅ | ✅ | ✅ |
| Crash-Free | ❌ | ✅ | ✅ | ✅ |

---

## 18. API Endpoint Mapping

Reference for the `td-api` crate. All endpoints target `https://api.todoist.com/api/v1/`.

| Resource | Endpoints | HTTP Methods |
|----------|-----------|--------------|
| Tasks | `/tasks`, `/tasks/{id}`, `/tasks/{id}/close`, `/tasks/{id}/reopen` | GET, POST, PATCH, DELETE, POST, POST |
| Quick Add | `/tasks/quick` | POST |
| Projects | `/projects`, `/projects/{id}` | GET, POST, PATCH, DELETE |
| Sections | `/sections`, `/sections/{id}` | GET, POST, PATCH, DELETE |
| Labels | `/labels`, `/labels/{id}` | GET, POST, PATCH, DELETE |
| Comments | `/comments`, `/comments/{id}` | GET, POST, PATCH, DELETE |
| Reminders | `/reminders`, `/reminders/{id}` | GET, POST, DELETE |
| Activity | `/activity/events` | GET |
| Collaborators | `/projects/{id}/collaborators` | GET |
| Filters | `/filters`, `/filters/{id}` | GET, POST, PATCH, DELETE |
| Sync | `/sync` | POST |

**Pagination:** All list endpoints return paginated results using cursor-based pagination. The client must handle `next_cursor` tokens to fetch complete result sets.

**Rate Limits:** 1000 requests per 15 minutes per user for standard endpoints. Implement exponential backoff with jitter.

---

## 19. Appendix: Addressed Community Pain Points

This section maps specific community complaints (from GitHub issues, Reddit threads, and forum posts) to features in this PRD:

| Pain Point | Source | Resolution |
|-----------|--------|------------|
| Crashes (SIGSEGV, panics) on sachaos tool | GitHub #228, #232, #250, #254, #266 | Rust's safety guarantees; comprehensive error handling; no `unwrap()` on user data |
| Deprecated API endpoints | GitHub #222, #268; Reddit; n8n community | Built on Todoist API v1 from the start |
| No section support | sachaos missing feature | Full section CRUD |
| No comment support | GitHub #193 | Full comment CRUD with stdin and file attachments |
| No backup/export | GitHub #223 | `td backup` command |
| No keyword search | GitHub #191 | `search:` filter expression |
| No multi-account | GitHub #211 | Named profiles with separate caches |
| No incremental sync | GitHub #261 | Sync token-based incremental sync by default |
| Node.js dependency | Official CLI issue | Static Rust binary, zero runtime deps |
| No filter expressions | Official CLI gap | Full filter engine matching Todoist syntax |
| Broken on Windows | GitHub #269, #250 | Cross-platform CI with Windows builds |
| No `--stdin` support | Doist CLI #86 | `--stdin` flag on add and comment commands |
| No `--json` for mutations | Doist CLI #87 | `--json` flag works globally |
| No keyring storage | Doist CLI #83 | OS keyring as default storage method |
| Poor error messages | General feedback | Typed errors with actionable suggestions |
| No accessibility | sachaos/chaosteil | `--accessible` mode with text labels |

---

*End of PRD*
