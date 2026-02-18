mod models;
mod task_manager;
mod xml_parser;

use crate::models::Task;
use std::io::stdin;
use std::process;
use task_manager::TaskManager;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = "task_database.xml";

    let Some(command) = args.get(1) else {
        eprintln!("Usage: todo_cli <new|view|done>");
        process::exit(1);
    };

    let mut manager = TaskManager::new();

    match xml_parser::read(filename) {
        Ok(tasks) => manager.set_tasks(tasks),
        Err(e) => {
            if command != "new" {
                eprintln!("Could not read {filename}: {e}");
                process::exit(1);
            }
        }
    }

    match command.as_str() {
        "new" => match create_new_task() {
            Ok(task) => manager.add_task(task),
            Err(e) => {
                eprintln!("Error creating task: {e}");
                process::exit(1);
            }
        },
        "view" => {
            view_tasks(manager.fetch_tasks(), false);
        }
        "done" => {
            complete_task(&mut manager);
        }
        other => {
            eprintln!("Unknown command '{other}'. Valid commands: new, view, done");
            process::exit(1);
        }
    }

    if let Err(e) = manager.save_tasks(filename) {
        eprintln!("Error saving tasks: {e}");
        process::exit(1);
    }
}

fn create_new_task() -> Result<Task, std::io::Error> {
    println!("Enter task description: ");
    let description = read_trimmed_line()?;

    println!("Enter task due date: ");
    let due_date = read_trimmed_line()?;

    println!("Is this task important? (y/n): ");
    let input = read_trimmed_line()?;
    let important = input.trim().eq_ignore_ascii_case("y");

    Ok(Task {
        description,
        due_date,
        important,
    })
}

fn read_trimmed_line() -> std::io::Result<String> {
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn view_tasks(tasks: &[Task], show_task_id: bool) {
    for (i, task) in tasks.iter().enumerate() {
        if show_task_id {
            println!("Task ID: {}", i);
        }
        println!("{}\n", task);
    }
}

fn complete_task(manager: &mut TaskManager) {
    let tasks = manager.fetch_tasks();
    if tasks.is_empty() {
        println!("No tasks to complete.");
        return;
    }

    println!("Listed below are your tasks:");
    view_tasks(tasks, true);

    println!("Enter the task ID of the finished task: ");
    let input = match read_trimmed_line() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading input: {e}");
            return;
        }
    };

    let task_id: usize = match input.parse() {
        Ok(id) => id,
        Err(_) => {
            println!("Invalid input. Please enter a valid task ID.");
            return;
        }
    };

    if manager.remove_task(task_id).is_none() {
        println!(
            "Task ID {task_id} is out of range. Valid IDs: 0-{}",
            manager.fetch_tasks().len()
        );
    }
}
