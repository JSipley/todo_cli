use std::clone::Clone;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};
use xml::writer::{EmitterConfig, XmlEvent as XmlWriteEvent};

#[derive(Debug, PartialEq, Clone)]
struct Task {
    description: String,
    due_date: String,
    important: String,
}

struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    fn set_tasks(&mut self, new_tasks: Vec<Task>) {
        self.tasks = new_tasks;
    }

    fn fetch_tasks(&self) -> &[Task] {
        &self.tasks
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }
}

static mut TASK_MANAGER: TaskManager = TaskManager { tasks: Vec::new() };

fn read_xml(filename: &str) -> Result<Vec<Task>, Box<dyn Error>> {
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

fn write_xml(filename: &str, tasks: Vec<Task>) -> Result<(), Box<dyn Error>> {
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

        writer.write(XmlWriteEvent::start_element("Description"))?;
        writer.write(XmlWriteEvent::characters(&task.description))?;
        writer.write(XmlWriteEvent::end_element())?;

        writer.write(XmlWriteEvent::start_element("Due_Date"))?;
        writer.write(XmlWriteEvent::characters(&task.due_date))?;
        writer.write(XmlWriteEvent::end_element())?;

        writer.write(XmlWriteEvent::start_element("Important"))?;
        writer.write(XmlWriteEvent::characters(&task.important))?;
        writer.write(XmlWriteEvent::end_element())?;

        writer.write(XmlWriteEvent::end_element())?;
    }

    Ok(())
}

fn main() {
    println!("Testing of writer_xml function");
    let test_tasks: Vec<Task> = read_xml("sudo_task_list.xml").unwrap();
    let _ = write_xml("test.xml", test_tasks);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    const XML_TEST_FILE_PATH: &str = "xml_test_files/";

    //Test read_xml functionality
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
        let result_tasks = read_xml(&filename).unwrap();

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
        let result_tasks = read_xml(&filename).unwrap();

        assert_eq!(expected_tasks, result_tasks);
    }

    #[test]
    fn read_empty_xml_file() {
        let filename = XML_TEST_FILE_PATH.to_owned() + "empty_file.xml";
        assert!(read_xml(&filename).is_err());
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
        let result = write_xml(filename, tasks);

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
    //test TASK_MANAGER functionality
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
