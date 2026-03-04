mod models;
mod task_manager;
mod xml_parser;

use crate::models::{Priority, Task};
use std::io::stdin;
use std::process;
use task_manager::{generate_unique_id, TaskManager};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = "task_database.xml";

    let Some(command) = args.get(1) else {
        eprintln!("Usage: todo_cli <new|view|done|edit|help>");
        process::exit(1);
    };

    let mut manager = TaskManager::new();

    match xml_parser::read(filename) {
        Ok(mut tasks) => {
            if let Err(e) = assign_missing_ids(&mut tasks) {
                eprintln!("Error assigning task IDs: {e}");
                process::exit(1);
            }
            manager.set_tasks(tasks);
        }
        Err(e) => {
            if command != "new" {
                eprintln!("Could not read {filename}: {e}");
                process::exit(1);
            }
        }
    }

    match command.as_str() {
        "new" => {
            if let Err(e) = create_new_task(&mut manager) {
                eprintln!("Error creating task: {e}");
                process::exit(1);
            }
        }
        "view" => view_tasks(manager.fetch_tasks()),
        "done" => complete_task(&mut manager, args.get(2).map(String::as_str)),
        "edit" => edit_task(&mut manager, args.get(2).map(String::as_str)),
        "help" => print_help(),
        _ => print_help(),
    }

    if let Err(e) = manager.save_tasks(filename) {
        eprintln!("Error saving tasks: {e}");
        process::exit(1);
    }
}

fn assign_missing_ids(tasks: &mut Vec<Task>) -> Result<(), String> {
    let mut used_ids: std::collections::HashSet<String> = tasks
        .iter()
        .filter(|t| !t.id.is_empty())
        .map(|t| t.id.clone())
        .collect();

    for task in tasks.iter_mut() {
        if task.id.is_empty() {
            let existing: Vec<&str> = used_ids.iter().map(String::as_str).collect();
            let new_id = generate_unique_id(&existing)?;
            used_ids.insert(new_id.clone());
            task.id = new_id;
        }
    }
    Ok(())
}

fn parse_due_date(input: &str) -> Result<String, ()> {
    let normalized = if input.len() == 8 && input.chars().all(|c| c.is_ascii_digit()) {
        format!("{}/{}/{}", &input[0..2], &input[2..4], &input[4..8])
    } else {
        input.replace(['-', '.', ' '], "/")
    };

    let parts: Vec<&str> = normalized.split('/').collect();
    if parts.len() != 3 || parts[0].len() != 2 || parts[1].len() != 2 || parts[2].len() != 4 {
        return Err(());
    }

    let month: u32 = parts[0].parse().map_err(|_| ())?;
    let day: u32 = parts[1].parse().map_err(|_| ())?;

    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return Err(());
    }

    Ok(normalized)
}

fn read_trimmed_line() -> std::io::Result<String> {
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn view_tasks(tasks: &[Task]) {
    for task in tasks {
        println!("{}", task);
    }
}

fn print_tasks_with_ids(tasks: &[Task]) {
    for task in tasks {
        println!("ID: {}", task.id);
        println!("{}", task);
    }
}

fn create_new_task(manager: &mut TaskManager) -> Result<(), String> {
    println!("Enter task description: ");
    let description = read_trimmed_line().map_err(|e| e.to_string())?;

    let due_date = loop {
        println!("Enter task due date (MM/DD/YYYY): ");
        let input = read_trimmed_line().map_err(|e| e.to_string())?;
        match parse_due_date(&input) {
            Ok(date) => break date,
            Err(()) => println!("Invalid date. Please use MM/DD/YYYY format."),
        }
    };

    println!("Select a priority:");
    println!("1. ASAP");
    println!("2. Important");
    println!("3. Medium");
    println!("4. Minor");
    println!("5. None");

    let priority = loop {
        let input = read_trimmed_line().map_err(|e| e.to_string())?;
        if input.is_empty() {
            break Priority::None;
        }
        match input.as_str() {
            "1" | "2" | "3" | "4" | "5" => {
                let n: u8 = input.parse().unwrap();
                break Priority::from_menu_number(n);
            }
            _ => println!("Please enter a number 1-5 or press Enter for None."),
        }
    };

    println!("Enter notes (optional): ");
    let notes = read_trimmed_line().map_err(|e| e.to_string())?;

    let existing_ids: Vec<&str> = manager.fetch_tasks().iter().map(|t| t.id.as_str()).collect();
    let id = generate_unique_id(&existing_ids)?;

    manager.add_task(Task {
        id,
        description,
        due_date,
        priority,
        notes,
    });

    Ok(())
}

fn complete_task(manager: &mut TaskManager, id_arg: Option<&str>) {
    let id = if let Some(id) = id_arg {
        id.to_uppercase()
    } else {
        let tasks = manager.fetch_tasks();
        if tasks.is_empty() {
            println!("No tasks to complete.");
            return;
        }
        print_tasks_with_ids(tasks);
        println!("Enter the task ID of the finished task: ");
        match read_trimmed_line() {
            Ok(s) => s.to_uppercase(),
            Err(e) => {
                eprintln!("Error reading input: {e}");
                return;
            }
        }
    };

    if manager.remove_task_by_id(&id).is_none() {
        println!("Task ID {id} not found.");
        print_tasks_with_ids(manager.fetch_tasks());
    }
}

fn edit_task(manager: &mut TaskManager, id_arg: Option<&str>) {
    let id = match id_arg {
        None => {
            if manager.fetch_tasks().is_empty() {
                println!("No tasks to edit.");
            } else {
                print_tasks_with_ids(manager.fetch_tasks());
            }
            return;
        }
        Some(id) => id.to_uppercase(),
    };

    if manager.find_task_by_id(&id).is_none() {
        println!("Task ID {id} not found.");
        if !manager.fetch_tasks().is_empty() {
            print_tasks_with_ids(manager.fetch_tasks());
        }
        return;
    }

    let current = manager.find_task_by_id(&id).unwrap().clone();

    println!("Description [{}]: ", current.description);
    let input = read_trimmed_line().unwrap_or_default();
    let new_description = if input.is_empty() {
        current.description.clone()
    } else {
        input
    };

    let new_due_date = loop {
        println!("Due date [{}]: ", current.due_date);
        let input = read_trimmed_line().unwrap_or_default();
        if input.is_empty() {
            break current.due_date.clone();
        }
        match parse_due_date(&input) {
            Ok(date) => break date,
            Err(()) => println!("Invalid date. Please use MM/DD/YYYY format."),
        }
    };

    let new_priority = loop {
        println!("Priority [{}]:", current.priority);
        println!("  1. ASAP\n  2. Important\n  3. Medium\n  4. Minor\n  5. None");
        println!("Enter 1-5 or press Enter to keep current: ");
        let input = read_trimmed_line().unwrap_or_default();
        if input.is_empty() {
            break current.priority.clone();
        }
        match input.as_str() {
            "1" => break Priority::Asap,
            "2" => break Priority::Important,
            "3" => break Priority::Medium,
            "4" => break Priority::Minor,
            "5" => break Priority::None,
            _ => println!("Please enter a number 1-5 or press Enter to keep current."),
        }
    };

    println!("Notes [{}]: ", current.notes);
    let input = read_trimmed_line().unwrap_or_default();
    let new_notes = if input.is_empty() {
        current.notes.clone()
    } else {
        input
    };

    if let Some(task) = manager.find_task_by_id_mut(&id) {
        task.description = new_description;
        task.due_date = new_due_date;
        task.priority = new_priority;
        task.notes = new_notes;

        println!("Task[{}] updated:\n{}", task.id, task);
    }
}

fn print_help() {
    println!("New:  Create a new task");
    println!("Edit: Modify an existing task");
    println!("Done: Mark a task as complete");
    println!("View: Display all tasks");
    println!("Help: Show this help message");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_due_date_valid() {
        assert_eq!(parse_due_date("01/06/2026"), Ok("01/06/2026".to_string()));
    }

    #[test]
    fn test_parse_due_date_dash_delimited() {
        assert_eq!(parse_due_date("01-06-2026"), Ok("01/06/2026".to_string()));
    }

    #[test]
    fn test_parse_due_date_eight_digit() {
        assert_eq!(parse_due_date("01062026"), Ok("01/06/2026".to_string()));
    }

    #[test]
    fn test_parse_due_date_invalid_month() {
        assert_eq!(parse_due_date("13/06/2026"), Err(()));
    }

    #[test]
    fn test_parse_due_date_invalid_day() {
        assert_eq!(parse_due_date("01/32/2026"), Err(()));
    }

    #[test]
    fn test_parse_due_date_short_digits() {
        assert_eq!(parse_due_date("0106202"), Err(()));
    }
}
