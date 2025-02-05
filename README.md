# Rust task tracker

A simple task tracker CLI application

Project built as part of the educational content provided by [Roadmap.sh](https://roadmap.sh/projects/task-tracker)

## Installation

1. Clone the repository

 ```bash
 git clone <github-repo-url>
 ```

2. Build the application

```bash
cargo build --release
``` 

## Usage

### Adding a new task

 ```bash
 task-cli add "Buy groceries"
 # Output: Task added successfully (ID: 1)
 ```

### Updating and deleting tasks

 ```bash
 task-cli update 1 "Buy groceries and cook dinner"
 task-cli delete 1
 ```

### Marking a task as in progress or done

 ```bash
 task-cli mark-in-progress 1
 task-cli mark-done 1
 ```

### Listing all tasks

 ```bash
 task-cli list
 ```

### Listing tasks by status

 ```bash
 task-cli list done
 task-cli list todo
 task-cli list in-progress
 ```