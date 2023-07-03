// models.rs contains all data models used in application

#[derive(Debug, PartialEq, Clone)]
pub struct Task {
    pub description: String,
    pub due_date: String,
    pub important: String,
}

impl Task {
    pub fn print(&self) {
        println!("Description: {}", self.description);
        println!("Due Date: {}", self.due_date);
        println!("Important: {}", self.important);
    }
}
