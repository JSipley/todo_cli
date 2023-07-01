// models.rs contains all data models used in application

#[derive(Debug, PartialEq, Clone)]
pub struct Task {
    pub description: String,
    pub due_date: String,
    pub important: String,
}