# Todo App

A simple command-line TODO application written in Rust using Clap for argument parsing and CSV file to store data.

## Features

- Add tasks
- Remove tasks
- List tasks
- List complete/incomplete tasks
- Mark tasks as complete/incomplete

## Installation

```bash
cargo install --path .
```

## Usage

```bash
todo-app [COMMAND] [OPTIONS]
```

### Commands
- `add <TASK>` or `a` - Add a new task
- `remove <ID>` or `r` - Remove a task
- `list` or `l` - Show all tasks
- `listComplete` or `lc` - Show all tasks
- `listIncomplete` or `li` - Show all tasks
- `complete <ID>` or `c` - Mark a task as complete
- `uncomplete <ID>` or `u` - Mark a task as complete

## License

MIT
