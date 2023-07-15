//task_manager.rs

use crate::models::Task;
use crate::xml_parser::write as write_to_xml;
use crate::Error;

pub struct TaskManager {
    pub tasks: Vec<Task>,
}

impl TaskManager {
    pub fn set_tasks(&mut self, new_tasks: Vec<Task>) {
        self.tasks = new_tasks;
    }

    pub fn fetch_tasks(&self) -> &[Task] {
        &self.tasks
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn save_tasks(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let _ = write_to_xml(filename, &self.tasks)?;
        Ok(())
    }

    pub fn remove_task(&mut self, task_index: usize) {
        self.tasks.remove(task_index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static mut TASK_MANAGER: TaskManager = TaskManager { tasks: Vec::new() };

    #[test]
    fn test_task_manager() {
        let mut test_tasks: Vec<Task> = vec![
            Task {
                description: "Example task one".to_string(),
                due_date: "1/25/2023".to_string(),
                important: "y".to_string(),
            },
            Task {
                description: "Example task two".to_string(),
                due_date: "3/10/2023".to_string(),
                important: "n".to_string(),
            },
            Task {
                description: "Example task three".to_string(),
                due_date: "5/31/2023".to_string(),
                important: "n".to_string(),
            },
        ];

        unsafe {
            //Test that all tasks will be fetched from TASK_MANAGER once tasks are set through set_tasks()
            TASK_MANAGER.set_tasks(test_tasks.clone());
            assert_eq!(TASK_MANAGER.fetch_tasks(), test_tasks);

            //Now add a task to task manager
            let new_task = Task {
                description: String::from("New task"),
                due_date: String::from("6/20/2023"),
                important: String::from("n"),
            };
            //Test that all tests set and the one extra task added are received through fetch_tasks()
            test_tasks.push(new_task.clone());
            TASK_MANAGER.add_task(new_task);

            assert_eq!(TASK_MANAGER.fetch_tasks(), test_tasks);
        }
    }
}