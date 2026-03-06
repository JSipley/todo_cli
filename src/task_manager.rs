use crate::models::Task;
use crate::xml_parser::write as write_to_xml;
use std::collections::HashSet;
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
        write_to_xml(filename, &self.tasks)
    }

    pub fn find_task_by_id(&self, id: &str) -> Option<&Task> {
        self.tasks.iter().find(|t| t.id == id)
    }

    pub fn find_task_by_id_mut(&mut self, id: &str) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|t| t.id == id)
    }

    pub fn remove_task_by_id(&mut self, id: &str) -> Option<Task> {
        self.tasks.iter().position(|t| t.id == id)
            .map(|pos| self.tasks.remove(pos))
    }
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}

pub fn generate_unique_id(existing_ids: &[&str]) -> Result<String, String> {
    use rand::seq::SliceRandom;

    let used: HashSet<u8> = existing_ids
        .iter()
        .filter_map(|id| u8::from_str_radix(id, 16).ok())
        .collect();

    let available: Vec<u8> = (0u8..=255).filter(|n| !used.contains(n)).collect();

    if available.is_empty() {
        return Err("All 256 task IDs are in use. Cannot create a new task.".to_string());
    }

    let mut rng = rand::thread_rng();
    let chosen = available.choose(&mut rng).unwrap();
    Ok(format!("{:02X}", chosen))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Priority;

    fn sample_tasks() -> Vec<Task> {
        vec![
            Task {
                id: "1A".to_string(),
                description: "Example task one".to_string(),
                due_date: "1/25/2023".to_string(),
                priority: Priority::None,
                notes: "".to_string(),
            },
            Task {
                id: "2B".to_string(),
                description: "Example task two".to_string(),
                due_date: "3/10/2023".to_string(),
                priority: Priority::None,
                notes: "".to_string(),
            },
            Task {
                id: "3C".to_string(),
                description: "Example task three".to_string(),
                due_date: "5/31/2023".to_string(),
                priority: Priority::None,
                notes: "".to_string(),
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
            id: "FF".to_string(),
            description: "New task".to_string(),
            due_date: "6/20/2023".to_string(),
            priority: Priority::None,
            notes: "".to_string(),
        };
        manager.add_task(new_task.clone());

        assert_eq!(manager.fetch_tasks().len(), 4);
        assert_eq!(manager.fetch_tasks().last().unwrap(), &new_task);
    }

    #[test]
    fn test_remove_task_by_id_valid() {
        let mut manager = TaskManager::new();
        manager.set_tasks(sample_tasks());

        let removed = manager.remove_task_by_id("2B");
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().description, "Example task two");
        assert_eq!(manager.fetch_tasks().len(), 2);
    }

    #[test]
    fn test_remove_task_by_id_invalid() {
        let mut manager = TaskManager::new();
        manager.set_tasks(sample_tasks());

        let removed = manager.remove_task_by_id("ZZ");
        assert!(removed.is_none());
        assert_eq!(manager.fetch_tasks().len(), 3);
    }

    #[test]
    fn test_find_task_by_id() {
        let mut manager = TaskManager::new();
        manager.set_tasks(sample_tasks());

        let task = manager.find_task_by_id("2B");
        assert!(task.is_some());
        assert_eq!(task.unwrap().description, "Example task two");

        let missing = manager.find_task_by_id("ZZ");
        assert!(missing.is_none());
    }

    #[test]
    fn test_generate_unique_id_returns_hex() {
        let result = generate_unique_id(&[]).unwrap();
        assert_eq!(result.len(), 2);
        assert!(u8::from_str_radix(&result, 16).is_ok());
        assert_eq!(result, result.to_uppercase());
    }

    #[test]
    fn test_generate_unique_id_avoids_existing() {
        let existing = vec!["00", "1A", "FF"];
        let result = generate_unique_id(&existing).unwrap();
        assert!(!existing.contains(&result.as_str()));
    }

    #[test]
    fn test_generate_unique_id_exhausted() {
        let all_ids: Vec<String> = (0u8..=255).map(|n| format!("{:02X}", n)).collect();
        let all_ids_refs: Vec<&str> = all_ids.iter().map(String::as_str).collect();
        let result = generate_unique_id(&all_ids_refs);
        assert!(result.is_err());
    }
}
