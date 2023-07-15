// models.rs contains all data models used in application
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Task {
    pub description: String,
    pub due_date: String,
    pub important: String,
}

impl fmt::Display for Task{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Description: {}\nDue Date: {}\nImportant: {}", 
               self.description, self.due_date, self.important)
    }
}
