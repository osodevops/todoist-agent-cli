# td

A fast, offline-capable CLI for [Todoist](https://todoist.com/), built in Rust.

Manage your tasks, projects, labels, comments, reminders, and filters entirely from the terminal with instant offline reads, fuzzy name resolution, and both human-friendly and machine-readable output.

[![CI](https://github.com/osodevops/todoist-agent-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/osodevops/todoist-agent-cli/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## Install

```bash
# From source
cargo install --git https://github.com/osodevops/todoist-agent-cli --bin td

# Or clone and build
git clone https://github.com/osodevops/todoist-agent-cli
cd todoist-agent-cli
cargo build --release
# Binary at target/release/td
```

## Setup

Set your [Todoist API token](https://app.todoist.com/app/settings/integrations/developer) and perform an initial sync:

```bash
export TODOIST_API_TOKEN="your-token-here"
td sync
```

Alternatively, store the token in the config file:

```bash
td auth token <YOUR_API_TOKEN>
td sync
```

This creates a local SQLite cache at `~/.cache/td/cache-default.db`. All read commands work offline from this cache.

## Quick Start

```bash
# View today's agenda
td today

# List all tasks
td list

# Add a task
td add "Buy groceries" -p Personal -d tomorrow -P 1

# Quick add with natural language (server-side parsing)
td quick "Call mom tomorrow at 5pm #Personal @important p1"

# Complete a task
td done <TASK_ID>

# Show task details
td show <TASK_ID>

# Edit a task
td edit <TASK_ID> --due "next week" --add-label urgent

# Move a task to another project
td move <TASK_ID> --project Work --section "In Progress"

# JSON output for scripting
td list --json | jq '.tasks[].content'

# Sync before any read command
td list --sync
```

### Projects

```bash
td project list
td project add "New Project" --color blue --view board
td project edit <ID> --name "Renamed" --favorite
td project archive <ID>
td project delete <ID> --yes
td project collaborators <ID>
```

### Sections

```bash
td section list --project Work
td section add "In Progress" --project Work
td section edit <ID> --name "Done"
td section delete <ID> --yes
```

### Labels

```bash
td label list
td label add urgent --color red
td label edit <ID> --name important
td label delete <ID> --yes
```

### Comments

```bash
td comment list --task <TASK_ID>
td comment add --task <TASK_ID> "Looks good!"
td comment edit <COMMENT_ID> "Updated text"
td comment delete <COMMENT_ID>
```

### Reminders

```bash
td reminder list --task <TASK_ID>
td reminder add --task <TASK_ID> --due "2026-03-15T09:00:00"
td reminder add --task <TASK_ID> --relative 30    # 30 min before due
td reminder delete <REMINDER_ID>
```

### Saved Filters

```bash
td filter list
td filter add "Work Today" --query "today & #Work"
td filter show <ID>
td filter edit <ID> --name "Urgent" --query "overdue & p1"
td filter delete <ID>
```

### Activity Log

```bash
td activity
td activity --limit 50
td activity --event-type "task:completed"
td activity --project <PROJECT_ID> --since 2026-03-01
```

### Authentication

```bash
td auth login            # Interactive setup wizard
td auth token <TOKEN>    # Set token directly
td auth status           # Show current auth status
td auth logout           # Remove stored credentials
td auth switch work      # Switch to named profile
```

### Shell Completions

```bash
td completions bash > ~/.local/share/bash-completion/completions/td
td completions zsh > ~/.zfunc/_td
td completions fish > ~/.config/fish/completions/td.fish
td completions powershell > _td.ps1
```

## Features

- **Offline-first** — reads are instant from a local SQLite cache, no network required
- **22 commands** — full coverage of tasks, projects, sections, labels, comments, reminders, filters, activity
- **Fuzzy name resolution** — reference projects and labels by name with "Did you mean…?" suggestions
- **Dual output** — colored table for humans, structured JSON for scripts and AI agents
- **Auto-detect output** — table when TTY, JSON when piped
- **Color-coded priorities** — p1 red, p2 yellow, p3 blue, p4 default
- **Color-coded dates** — overdue red, today green, tomorrow yellow
- **Batch operations** — `td done id1 id2 id3`, `td delete id1 id2 --yes`
- **Natural language dates** — `td add "Task" --due "next friday"`
- **Quick add** — server-side NLP with `#Project`, `@Label`, `pN`, `/Section` syntax
- **Shell completions** — bash, zsh, fish, powershell
- **Man pages** — 56 man pages generated via `cargo run -p xtask`
- **Parallel sync** — fetches tasks, projects, sections, and labels concurrently
- **Retry with backoff** — automatic retry on 429/5xx with exponential backoff and jitter
- **Idempotent writes** — `X-Request-Id` header on all mutations
- **Multi-profile** — separate config and cache per profile
- **Cross-platform** — macOS (Apple Silicon + Intel), Linux, Windows
- **Zero runtime dependencies** — single statically-linked binary

## Configuration

Config file at `~/.config/td/config.toml`:

```toml
[default]
color = "auto"            # "auto" | "always" | "never"
date_format = "%Y-%m-%d"
default_output = "table"  # "table" | "json"
auto_sync_on_write = true
sync_timeout_secs = 30
# token = "your-token"    # Or use TODOIST_API_TOKEN env var

# Named profiles for multi-account
[profiles.work]
token = "work-token-here"

[profiles.personal]
token = "personal-token-here"
```

**Precedence:** `--token` flag > `TODOIST_API_TOKEN` env > config file token

```bash
# Use a named profile
td --profile work list
td --profile personal today

# Or via environment
export TD_PROFILE=work
td list
```

Each profile gets its own cache database to avoid cross-contamination.

## CLI Reference

```
td [OPTIONS] <COMMAND>

Commands:
  sync         Sync local cache with Todoist
  list         List tasks from the local cache
  today        Show today's agenda (due today + overdue)
  inbox        Show tasks in the Inbox project
  add          Add a new task
  quick        Quick add a task using natural language
  done         Complete one or more tasks
  delete       Delete one or more tasks
  show         Show full details for a task
  edit         Edit an existing task
  reopen       Reopen a completed task
  move         Move a task to a different project, section, or parent
  project      Manage projects (list, show, add, edit, archive, delete)
  section      Manage sections (list, add, edit, move, delete)
  label        Manage labels (list, add, edit, delete)
  comment      Manage comments on tasks and projects
  reminder     Manage reminders for tasks
  filter       Manage saved filters
  activity     View the activity log
  auth         Authentication commands (login, logout, status)
  completions  Generate shell completions

Global Options:
  -s, --sync               Sync before executing the command
  -j, --json               Force JSON output
  -q, --quiet              Suppress output (errors only)
  -v, --verbose            Enable debug logging
      --no-color           Disable colored output
      --token <TOKEN>      Override API token
      --profile <PROFILE>  Use a named profile
```

## Architecture

```
todoist-agent-cli/
├── crates/
│   ├── td-api/       # Todoist API v1 client (auth, pagination, retries)
│   ├── td-cache/     # SQLite cache (migrations, CRUD, sync state)
│   └── td-cli/       # CLI binary (clap v4, commands, output formatters)
└── xtask/            # Man page generation (clap_mangen)
```

| Crate | Purpose |
|-------|---------|
| `td-api` | HTTP client with Bearer auth, cursor-based pagination, exponential backoff, `X-Request-Id` idempotency |
| `td-cache` | SQLite via rusqlite (bundled), versioned migrations, JSON-in-column for nested objects |
| `td-cli` | 22 commands, table + JSON output, fuzzy name resolution, shell completions |
| `xtask` | Generates 56 man pages from clap definitions |

## Building from Source

```bash
git clone https://github.com/osodevops/todoist-agent-cli
cd todoist-agent-cli
cargo build --release
# Binary at target/release/td
```

**Requirements:** Rust 1.85+

```bash
# Run tests
cargo test --workspace

# Lint
cargo clippy --workspace --all-targets -- -D warnings

# Generate man pages
cargo run -p xtask
# Man pages at target/man/
```

## Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | General error |
| `2` | Auth error |
| `3` | Network error |
| `4` | Not found |
| `5` | Validation error |
| `10` | Cache error |

## Roadmap

- [x] **Phase 1: Core MVP** — tasks CRUD, sync, list/today/inbox, add/quick/done/delete/show/edit, table + JSON output, config, auth, shell completions
- [x] **Phase 2: Full Resource Coverage** — projects, sections, labels, comments, reminders, filters, activity log, move, reopen, man pages
- [ ] **Phase 3: Power Features** — filter expression engine, full-text search, interactive fuzzy picker, natural language date parsing (client-side), backup/export, CSV output, `--stdin` support
- [ ] **Phase 4: Agent & Ecosystem** — `--agent` mode with HATEOAS JSON, Claude Code skill file, MCP server wrapper, offline write queue, Homebrew tap, AUR/Nix packaging

## License

[MIT](LICENSE)
