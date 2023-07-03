mod models;
mod task_manager;
mod xml_parser;

use crate::models::Task;
use std::{env, error::Error, io::stdin};
use task_manager::TaskManager;
use xml_parser::read as read_from_xml;

static mut TASK_MANAGER: TaskManager = TaskManager { tasks: Vec::new() };

fn main() {
    let args: Vec<String> = env::args().collect();
    let user_command = &args[1];
    let filename = "task_database.xml";

    {
        let read_tasks = read_from_xml(filename).unwrap();
        unsafe {
            TASK_MANAGER.set_tasks(read_tasks);
        }
    }

    if user_command == "new" {
        let new_task = create_new_task().unwrap();
        unsafe { TASK_MANAGER.add_task(new_task) };
    } else if user_command == "view" {
        view_tasks();
    } else {
        eprintln!("Invalid arguments, try again");
    }

    //Once all user operations are done, write tasks to file
    unsafe {
        let _ = TASK_MANAGER.save_tasks(filename);
    }
}

fn create_new_task() -> Result<Task, Box<dyn Error>> {
    println!("Enter task description: ");
    let task_description = read_trimmed_line().unwrap();

    println!("Enter task due date: ");
    let task_due_date = read_trimmed_line().unwrap();

    println!("Is this task important? (y/n): ");
    let important_task = read_trimmed_line().unwrap();

    Ok(Task {
        description: task_description.to_string(),
        due_date: task_due_date.to_string(),
        important: important_task.to_string(),
    })
}

//Trim the newline ('\n') character off user input
fn read_trimmed_line() -> std::io::Result<String> {
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn view_tasks() {
    unsafe {
        let tasks = TASK_MANAGER.fetch_tasks();
        for task in tasks {
            task.print();
            println!();
        }
    }
}
