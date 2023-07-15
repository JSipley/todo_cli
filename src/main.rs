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
        view_tasks(false);
    } else if user_command == "done" {
        complete_task();
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

fn view_tasks(show_task_id: bool) {
    unsafe {
        let tasks = TASK_MANAGER.fetch_tasks();
        for i in 0..tasks.len(){
            if show_task_id {
                println!("Task ID: {}", i);
            }
            tasks[i].print();
            println!();
        }
    }
}

fn complete_task() {
    println!("Listed below are your tasks:");
    //Call view_tasks with the addition of displaying each task with an ID number
    view_tasks(true);
    println!("Enter the task ID of the finished task: ");
    let task_id: usize = match read_trimmed_line().unwrap().trim().parse::<usize>() {
        Ok(id) => id,
        Err(_) => {
            println!("Invalid input. Please enter a valid task ID.");
            return; // Early return if parsing fails
        }
    };
    unsafe { TASK_MANAGER.remove_task(task_id); }
}
