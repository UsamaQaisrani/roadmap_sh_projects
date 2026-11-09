# Task Tracker

A simple command-line task tracker written in Rust. Tasks are stored in a `tasks.json` file in the current directory.

Project from: https://roadmap.sh/projects/task-tracker

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024, so Rust 1.85 or newer)

## Build

```bash
cargo build --release
```

The binary will be at `target/release/task_tracker`.

## Usage

You can run commands with `cargo run -- <command>` or with the built binary directly.

```bash
# Add a new task
cargo run -- add "Buy groceries"

# Update a task's description
cargo run -- update 1 "Buy groceries and cook dinner"

# Delete a task
cargo run -- delete 1

# Mark a task's status (todo, in-progress, done, skip)
cargo run -- mark 1 in-progress
cargo run -- mark 1 done

# List all tasks
cargo run -- list

# List tasks by status
cargo run -- list todo
cargo run -- list in-progress
cargo run -- list done
cargo run -- list skip
```

Run `cargo run -- --help` or `cargo run -- <command> --help` to see all options.

## Data

Tasks are saved to `tasks.json` in the directory you run the command from. Each task has:

- `id`
- `description`
- `status` (`Todo`, `InProgress`, `Done`, `Skip`)
- `created_at`
- `updated_at`
