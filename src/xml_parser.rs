//xml_parser.rs

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};
use xml::writer::{EmitterConfig, XmlEvent as XmlWriteEvent};

use crate::models::Task;

pub fn read(filename: &str) -> Result<Vec<Task>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let file = BufReader::new(file);
    let parser = EventReader::new(file);

    let mut task_list = Vec::new();
    let mut current_task: Option<Task> = None;

    for event in parser {
        match event? {
            XmlEvent::StartElement { name, .. } => {
                if name.local_name == "Task" {
                    current_task = Some(Task {
                        description: String::new(),
                        due_date: String::new(),
                        important: String::new(),
                    });
                }
            }
            XmlEvent::EndElement { name } => {
                if name.local_name == "Task" {
                    if let Some(task) = current_task.take() {
                        task_list.push(task);
                    }
                }
            }
            XmlEvent::Characters(text) => {
                if let Some(ref mut task) = current_task {
                    match task {
                        task if task.description.is_empty() => task.description = text,
                        task if task.due_date.is_empty() => task.due_date = text,
                        task if task.important.is_empty() => task.important = text,
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

        let mut write_field = | name: &str, data: &str | -> Result<(), Box<dyn Error>> {
            writer.write(XmlWriteEvent::start_element(name))?;
            writer.write(XmlWriteEvent::characters(data))?;
            writer.write(XmlWriteEvent::end_element())?;

            Ok(())
        };

        write_field("Description", &task.description)?;
        write_field("Due_Date", &task.due_date)?;
        write_field("Important", &task.important)?;

        writer.write(XmlWriteEvent::end_element())?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    const XML_TEST_FILE_PATH: &str = "xml_test_files/";

    //Test read functionality
    #[test]
    fn read_valid_task_data() {
        let expected_tasks: Vec<Task> = vec![
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

        let filename = XML_TEST_FILE_PATH.to_owned() + "valid_test_tasks.xml";
        let result_tasks = read(&filename).unwrap();

        assert_eq!(result_tasks, expected_tasks);
    }

    #[test]
    fn read_invalid_task_data() {
        let expected_tasks: Vec<Task> = vec![Task {
            description: " Example task one ".to_string(),
            due_date: " 1/25/2023 ".to_string(),
            important: " y ".to_string(),
        }];

        let filename = XML_TEST_FILE_PATH.to_owned() + "invalid_test_tasks.xml";
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
        // Create a temporary file for testing
        let filename = "test.xml";

        // Define some sample tasks
        let tasks = vec![
            Task {
                description: "Task 1".to_string(),
                due_date: "2023-06-10".to_string(),
                important: "n".to_string(),
            },
            Task {
                description: "Task 2".to_string(),
                due_date: "2023-06-15".to_string(),
                important: "y".to_string(),
            },
        ];

        // Invoke the write_xml function
        let result = write(filename, &tasks);

        // Assert that the function call succeeded
        assert!(result.is_ok());

        // Read the contents of the written file
        let mut file = File::open(filename).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        // Assert that the file contains the expected XML structure and data
        let expected_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<Task>
  <Description>Task 1</Description>
  <Due_Date>2023-06-10</Due_Date>
  <Important>n</Important>
</Task>
<Task>
  <Description>Task 2</Description>
  <Due_Date>2023-06-15</Due_Date>
  <Important>y</Important>
</Task>"#;

        assert_eq!(contents, expected_xml);

        // Clean up the temporary file
        std::fs::remove_file(filename).unwrap();
    }
}