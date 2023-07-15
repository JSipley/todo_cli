# Todo CLI (Command Line Interface)

Todo CLI is a simple command-line tool that allows you to manage your to-do list right from the terminal. This Rust-based project provides an efficient and straightforward way to keep track of tasks, add new ones, mark them as completed, and remove them when done, all without leaving the command-line environment.

## Features

- Create tasks with a description, due date, and important marker.
- View all tasks.
- Mark tasks as completed.
- Data is stored in an XML file for easy persistence.

## Prerequisites

- [Rust programming language](https://www.rust-lang.org/) installed on your system.

## Installation

1. Clone this repository to your local machine using the following command:

```bash
git clone https://github.com/JSipley/todo_cli.git
```

2. Change into the project directory:

```bash
cd todo_cli
```

3. Build the project using Cargo:

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

- `view`: View all tasks, both completed and incomplete.

- `done`: Mark a task as completed.

### Examples

1. Adding a new task:

```bash
./target/release/todo_cli new
```

You will be prompted to enter the task description, due date, and whether it is important. Below is an example of creating a new task.

```bash
Enter task description: 
Task 1
Enter task due date: 
7/15/2023
Is this task important? (y/n): 
n
```

2. Viewing all tasks:

```bash
./target/release/todo_cli view
```

This is what viewing tasks should look like.

```bash
Description: Task 1
Due Date: 7/15/2023
Important: n

Description: Task 2
Due Date: 8/16/2023
Important: n
```

3. Marking a task as completed:

```bash
./target/release/todo_cli done
```

You will be prompted to select the task ID to mark as completed.

```bash
Listed below are your tasks:
Task ID: 0
Description:  Example task one
Due Date:  1/25/2023
Important:  y
Task ID: 1
Description:  Example task two
Due Date:  3/10/2023
Important:  n
Task ID: 2
Description:  Example task three
Due Date:  5/31/2023
Important:  n
Task ID: 3
Description: Task 1
Due Date: 7/15/2023
Important: n
Enter the task ID of the finished task:
```

## File Storage

The to-do list data is stored in an XML file named `task_database.xml` in the project directory. When you add or complete tasks, the changes will be saved to this file automatically. In the current state of the application, it's important to run todo_cli from the directory that has `task_database.xml`.

## Contributing

Contributions to this project are welcome! If you find any issues or have suggestions for improvement, feel free to open an issue or submit a pull request.

When contributing code, please follow the existing code style and ensure that your changes do not introduce any new bugs. Also, include relevant unit tests with your code.

## License

This project is licensed under the [MIT License](LICENSE).

## Acknowledgments

Special thanks to [Your Name] for the inspiration and support throughout the development of this project.
