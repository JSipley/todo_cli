use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum Priority {
    Asap,
    Important,
    Medium,
    Minor,
    None,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Priority::Asap => write!(f, "ASAP"),
            Priority::Important => write!(f, "Important"),
            Priority::Medium => write!(f, "Medium"),
            Priority::Minor => write!(f, "Minor"),
            Priority::None => write!(f, "None"),
        }
    }
}

impl FromStr for Priority {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "asap" => Ok(Priority::Asap),
            "important" => Ok(Priority::Important),
            "medium" => Ok(Priority::Medium),
            "minor" => Ok(Priority::Minor),
            _ => Ok(Priority::None),
        }
    }
}

impl Priority {
    pub fn from_menu_number(n: u8) -> Priority {
        match n {
            1 => Priority::Asap,
            2 => Priority::Important,
            3 => Priority::Medium,
            4 => Priority::Minor,
            _ => Priority::None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Task {
    pub id: String,
    pub description: String,
    pub due_date: String,
    pub priority: Priority,
    pub notes: String,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sep = "─".repeat(40);
        write!(f, "{}\n", sep)?;
        if self.priority == Priority::None {
            writeln!(f, "{}", self.description)?;
        } else {
            writeln!(f, "{} ({})", self.description, self.priority)?;
        }
        if !self.notes.is_empty() {
            writeln!(f, "{}", self.notes)?;
        }
        writeln!(f, "{}", self.due_date)?;
        write!(f, "{}", sep)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn priority_display() {
        assert_eq!(format!("{}", Priority::Asap), "ASAP");
        assert_eq!(format!("{}", Priority::Important), "Important");
        assert_eq!(format!("{}", Priority::Medium), "Medium");
        assert_eq!(format!("{}", Priority::Minor), "Minor");
        assert_eq!(format!("{}", Priority::None), "None");
    }

    fn make_task(priority: Priority, notes: &str) -> Task {
        Task {
            id: "1A".to_string(),
            description: "Buy groceries".to_string(),
            due_date: "01/06/2026".to_string(),
            priority,
            notes: notes.to_string(),
        }
    }

    #[test]
    fn task_display_with_notes() {
        let task = make_task(Priority::Important, "Pick up milk");
        let output = format!("{}", task);
        let sep = "─".repeat(40);
        assert!(output.contains("Buy groceries (Important)"));
        assert!(output.contains("Pick up milk"));
        assert!(output.contains("01/06/2026"));
        assert_eq!(output.matches(&sep).count(), 2);
    }

    #[test]
    fn task_display_without_notes() {
        let task = make_task(Priority::Medium, "");
        let output = format!("{}", task);
        assert!(output.contains("Buy groceries (Medium)"));
        assert!(!output.contains("\n\n\n")); // no extra blank line from missing notes
        assert!(output.contains("01/06/2026"));
    }

    #[test]
    fn task_display_none_priority() {
        let task = make_task(Priority::None, "");
        let output = format!("{}", task);
        assert!(output.contains("Buy groceries\n"));
        assert!(!output.contains("(None)"));
    }
}
