use crate::models::Task;
use crate::xml_parser::write as write_to_xml;
use std::error::Error;

pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager { tasks: Vec::new() }
    }

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
        write_to_xml(filename, &self.tasks)?;
        Ok(())
    }

    pub fn remove_task(&mut self, task_index: usize) -> Option<Task> {
        if task_index < self.tasks.len() {
            Some(self.tasks.remove(task_index))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_tasks() -> Vec<Task> {
        vec![
            Task {
                description: "Example task one".to_string(),
                due_date: "1/25/2023".to_string(),
                important: true,
            },
            Task {
                description: "Example task two".to_string(),
                due_date: "3/10/2023".to_string(),
                important: false,
            },
            Task {
                description: "Example task three".to_string(),
                due_date: "5/31/2023".to_string(),
                important: false,
            },
        ]
    }

    #[test]
    fn test_new_creates_empty_manager() {
        let manager = TaskManager::new();
        assert!(manager.fetch_tasks().is_empty());
    }

    #[test]
    fn test_set_and_fetch_tasks() {
        let mut manager = TaskManager::new();
        let tasks = sample_tasks();
        manager.set_tasks(tasks.clone());
        assert_eq!(manager.fetch_tasks(), tasks);
    }

    #[test]
    fn test_add_task() {
        let mut manager = TaskManager::new();
        manager.set_tasks(sample_tasks());

        let new_task = Task {
            description: "New task".to_string(),
            due_date: "6/20/2023".to_string(),
            important: false,
        };
        manager.add_task(new_task.clone());

        assert_eq!(manager.fetch_tasks().len(), 4);
        assert_eq!(manager.fetch_tasks().last().unwrap(), &new_task);
    }

    #[test]
    fn test_remove_task_valid_index() {
        let mut manager = TaskManager::new();
        manager.set_tasks(sample_tasks());

        let removed = manager.remove_task(1);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().description, "Example task two");
        assert_eq!(manager.fetch_tasks().len(), 2);
    }

    #[test]
    fn test_remove_task_out_of_bounds() {
        let mut manager = TaskManager::new();
        manager.set_tasks(sample_tasks());

        let removed = manager.remove_task(10);
        assert!(removed.is_none());
        assert_eq!(manager.fetch_tasks().len(), 3);
    }
}
