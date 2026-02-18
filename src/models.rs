use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Task {
    pub description: String,
    pub due_date: String,
    pub important: bool,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let important_str = if self.important { "Yes" } else { "No" };
        write!(
            f,
            "Description: {}\nDue Date: {}\nImportant: {}",
            self.description, self.due_date, important_str
        )
    }
}
