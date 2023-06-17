use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug, PartialEq)]
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

fn read_xml(filename: &str) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
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

fn main() {
    let read_tasks =
        read_xml("sudo_task_list.xml").expect("Had trouble parsing tasks from xml file");

    //Basic usage with TaskManager
    unsafe {
        TASK_MANAGER.set_tasks(read_tasks);
        for current_task in TASK_MANAGER.fetch_tasks() {
            println!("{:?}", current_task);
        }

        let new_task = Task {
            description: String::from("new task"),
            due_date: String::from("1/1/2024"),
            important: String::from("y"),
        };
        TASK_MANAGER.add_task(new_task);

        let new_task_list = TASK_MANAGER.fetch_tasks();
        println!("Read tasks with new task added: {:?}", new_task_list);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn read_invalid_task_data() {}
    #[test]
    fn read_empty_xml_file() {}

    //test TASK_MANAGER functionality
    #[test]
    fn set_tasks_test() {}
    #[test]
    fn add_task_test() {}
}
