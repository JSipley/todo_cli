use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};
use xml::writer::{EmitterConfig, XmlEvent as XmlWriteEvent};

use crate::models::{Priority, Task};

pub fn read(filename: &str) -> Result<Vec<Task>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let file = BufReader::new(file);
    let parser = EventReader::new(file);

    let mut task_list = Vec::new();
    let mut current_task: Option<Task> = None;
    let mut current_element: Option<String> = None;

    for event in parser {
        match event? {
            XmlEvent::StartElement { name, .. } => match name.local_name.as_str() {
                "Task" => {
                    current_task = Some(Task {
                        id: String::new(),
                        description: String::new(),
                        due_date: String::new(),
                        priority: Priority::None,
                        notes: String::new(),
                    });
                }
                other => {
                    current_element = Some(other.to_string());
                }
            },
            XmlEvent::EndElement { name } => {
                if name.local_name == "Task" {
                    if let Some(task) = current_task.take() {
                        task_list.push(task);
                    }
                }
                current_element = None;
            }
            XmlEvent::Characters(text) => {
                if let (Some(ref mut task), Some(ref elem)) = (&mut current_task, &current_element)
                {
                    match elem.as_str() {
                        "ID" => task.id = text,
                        "Description" => task.description = text,
                        "Due_Date" => task.due_date = text,
                        "Priority" => task.priority = text.parse().unwrap(),
                        "Notes" => task.notes = text,
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    Ok(task_list)
}

pub fn write(filename: &str, tasks: &[Task]) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(filename)?;
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(&mut file);

    writer.write(XmlWriteEvent::StartDocument {
        version: xml::common::XmlVersion::Version10,
        encoding: Some("UTF-8"),
        standalone: None,
    })?;

    for task in tasks {
        writer.write(XmlWriteEvent::start_element("Task"))?;

        let mut write_field = |name: &str, data: &str| -> Result<(), Box<dyn Error>> {
            writer.write(XmlWriteEvent::start_element(name))?;
            if !data.is_empty() {
                writer.write(XmlWriteEvent::characters(data))?;
            }
            writer.write(XmlWriteEvent::end_element())?;
            Ok(())
        };

        write_field("ID", &task.id)?;
        write_field("Description", &task.description)?;
        write_field("Due_Date", &task.due_date)?;
        write_field("Priority", &task.priority.to_string())?;
        write_field("Notes", &task.notes)?;

        writer.write(XmlWriteEvent::end_element())?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    const XML_TEST_FILE_PATH: &str = "xml_test_files/";

    #[test]
    fn read_valid_task_data() {
        let expected_tasks: Vec<Task> = vec![
            Task {
                id: "1A".to_string(),
                description: "Example task one".to_string(),
                due_date: "1/25/2023".to_string(),
                priority: Priority::Asap,
                notes: "Some notes here".to_string(),
            },
            Task {
                id: "2B".to_string(),
                description: "Example task two".to_string(),
                due_date: "3/10/2023".to_string(),
                priority: Priority::Medium,
                notes: "".to_string(),
            },
            Task {
                id: "3C".to_string(),
                description: "Example task three".to_string(),
                due_date: "5/31/2023".to_string(),
                priority: Priority::None,
                notes: "".to_string(),
            },
        ];

        let filename = XML_TEST_FILE_PATH.to_owned() + "valid_test_tasks.xml";
        let result_tasks = read(&filename).unwrap();

        assert_eq!(result_tasks, expected_tasks);
    }

    #[test]
    fn read_invalid_task_data() {
        let expected_tasks: Vec<Task> = vec![Task {
            id: "".to_string(),
            description: " Example task one ".to_string(),
            due_date: " 1/25/2023 ".to_string(),
            priority: Priority::None,
            notes: "".to_string(),
        }];

        let filename = XML_TEST_FILE_PATH.to_owned() + "invalid_test_tasks.xml";
        let result_tasks = read(&filename).unwrap();

        assert_eq!(expected_tasks, result_tasks);
    }

    #[test]
    fn read_missing_new_fields() {
        let expected_tasks: Vec<Task> = vec![Task {
            id: "".to_string(),
            description: "Legacy task".to_string(),
            due_date: "01/01/2025".to_string(),
            priority: Priority::None,
            notes: "".to_string(),
        }];

        let filename = XML_TEST_FILE_PATH.to_owned() + "legacy_tasks.xml";
        let result_tasks = read(&filename).unwrap();

        assert_eq!(expected_tasks, result_tasks);
    }

    #[test]
    fn read_empty_xml_file() {
        let filename = XML_TEST_FILE_PATH.to_owned() + "empty_file.xml";
        assert!(read(&filename).is_err());
    }

    #[test]
    fn test_write_xml() {
        let dir = std::env::temp_dir();
        let filename = dir.join("todo_cli_test_write.xml");
        let filename = filename.to_str().unwrap();

        let tasks = vec![
            Task {
                id: "A1".to_string(),
                description: "Task 1".to_string(),
                due_date: "2023-06-10".to_string(),
                priority: Priority::None,
                notes: "".to_string(),
            },
            Task {
                id: "B2".to_string(),
                description: "Task 2".to_string(),
                due_date: "2023-06-15".to_string(),
                priority: Priority::Important,
                notes: "some notes".to_string(),
            },
        ];

        let result = write(filename, &tasks);
        assert!(result.is_ok());

        let mut file = File::open(filename).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let expected_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<Task>
  <ID>A1</ID>
  <Description>Task 1</Description>
  <Due_Date>2023-06-10</Due_Date>
  <Priority>None</Priority>
  <Notes />
</Task>
<Task>
  <ID>B2</ID>
  <Description>Task 2</Description>
  <Due_Date>2023-06-15</Due_Date>
  <Priority>Important</Priority>
  <Notes>some notes</Notes>
</Task>"#;

        assert_eq!(contents, expected_xml);

        std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_round_trip() {
        let dir = std::env::temp_dir();
        let filename = dir.join("todo_cli_test_roundtrip.xml");
        let filename = filename.to_str().unwrap();

        let tasks = vec![
            Task {
                id: "AA".to_string(),
                description: "Buy groceries".to_string(),
                due_date: "2024-01-15".to_string(),
                priority: Priority::Asap,
                notes: "Don't forget milk".to_string(),
            },
            Task {
                id: "BB".to_string(),
                description: "Walk the dog".to_string(),
                due_date: "2024-01-16".to_string(),
                priority: Priority::None,
                notes: "".to_string(),
            },
        ];

        write(filename, &tasks).unwrap();
        let read_back = read(filename).unwrap();

        assert_eq!(tasks, read_back);

        std::fs::remove_file(filename).unwrap();
    }
}
