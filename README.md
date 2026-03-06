# Todo CLI (Command Line Interface)

Todo CLI is a simple command-line tool that allows you to manage your to-do list right from the terminal. This Rust-based project provides an efficient and straightforward way to keep track of tasks, add new ones, edit them, mark them as completed, and remove them when done, all without leaving the command-line environment.

## Features

- Create tasks with a description, due date, priority, and optional notes.
- View all tasks with a clean formatted display.
- Edit existing tasks in place.
- Mark tasks as completed.
- Tasks are identified by a unique 2-digit hex ID (e.g. `3D`).
- Data is stored in an XML file for easy persistence.

## Prerequisites

- [Rust programming language](https://www.rust-lang.org/) installed on your system.

## Installation

1. Clone this repository to your local machine using the following command:

```bash
git clone https://github.com/JSipley/todo_cli.git
```

1. Change into the project directory:

```bash
cd todo_cli
```

1. Build the project using Cargo:

```bash
cargo build --release
```

## Usage

To run the Todo CLI, open your terminal and navigate to the project directory (if you're not already there). Use the following command to execute the program:

```bash
./target/release/todo_cli <command>
```

### Commands

- `new`: Create a new task and add it to the to-do list.
- `view`: View all tasks.
- `done [ID]`: Mark a task as completed. Optionally pass the hex ID directly.
- `edit [ID]`: Modify an existing task. Optionally pass the hex ID directly.
- `help`: Show available commands.

### Examples

1. Adding a new task:

```bash
./target/release/todo_cli new
```

You will be prompted for the task description, due date, priority, and optional notes.

```text
Enter task description:
Buy groceries
Enter task due date (MM/DD/YYYY):
1/15/2026
Select a priority:
1. ASAP
2. Important
3. Medium
4. Minor
5. None
2
Enter notes (optional):
Don't forget milk
```

**Due date formats accepted:** `MM/DD/YYYY`, `M/D/YYYY`, `MM-DD-YYYY`, `MMDDYYYY`, `MM/DD/YY` (year expanded to `20YY`), and combinations thereof.

1. Viewing all tasks:

```bash
./target/release/todo_cli view
```

```text
────────────────────────────────────────
Buy groceries (Important)
Don't forget milk
01/15/2026
────────────────────────────────────────
────────────────────────────────────────
Walk the dog
01/16/2026
────────────────────────────────────────
```

1. Marking a task as completed:

```bash
./target/release/todo_cli done
```

If no ID is provided, your tasks are listed with their hex IDs and you are prompted to enter one:

```text
ID: 3D
────────────────────────────────────────
Buy groceries (Important)
Don't forget milk
01/15/2026
────────────────────────────────────────

Enter the task ID of the finished task:
3D
```

You can also pass the ID directly:

```bash
./target/release/todo_cli done 3D
```

1. Editing a task:

```bash
./target/release/todo_cli edit 3D
```

You are prompted for each field. Press Enter to keep the current value.

```text
Description [Buy groceries]:

Due date [01/15/2026]:
1/20/2026
Priority [Important]:
  1. ASAP  
  2. Important  
  3. Medium  
  4. Minor  
  5. None
Enter 1-5 or press Enter to keep current:
3
Notes [Don't forget milk]:

Task [3D] updated!:
────────────────────────────────────────
Buy groceries (Medium)
Test Note
1/20/2026
────────────────────────────────────────
```

1. Showing help:

```bash
./target/release/todo_cli help
```

```text
New:  Create a new task
Edit: Modify an existing task
Done: Mark a task as complete
View: Display all tasks
Help: Show this help message
```

## File Storage

The to-do list data is stored in an XML file named `task_database.xml` in the project directory. When you add, edit, or complete tasks, the changes will be saved to this file automatically. If the file doesn't exist yet, running the `new` command will create it. For `view`, `done`, and `edit`, the file must be present. Run todo_cli from the directory where you want `task_database.xml` to live.

## Contributing

Contributions to this project are welcome! If you find any issues or have suggestions for improvement, feel free to open an issue or submit a pull request.

When contributing code, please follow the existing code style and ensure that your changes do not introduce any new bugs. Also, include relevant unit tests with your code.

## License

This project is licensed under the [MIT License](LICENSE).
