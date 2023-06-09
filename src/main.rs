use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

struct Task {
    description: String,
    due_date: String,
    important: String,
}

fn read_xml(filename: &str) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let file = BufReader::new(file);
    let parser = EventReader::new(file);

    let mut task_list = Vec::new();
    let mut current_task: Option<Task> = None;

    for event in parser {
        match event? {
            XmlEvent::StartElement { name, .. } => match name.local_name.as_str() {
                "Task" => {
                    current_task = Some(Task {
                        description: String::new(),
                        due_date: String::new(),
                        important: String::new(),
                    });
                }
                _ => {}
            },
            XmlEvent::EndElement { name } => match name.local_name.as_str() {
                "Task" => {
                    if let Some(task) = current_task.take() {
                        task_list.push(task);
                    }
                }
                _ => {}
            },
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
    //Try reading tasks from sudo_task_list.xml
    match read_xml("sudo_task_list.xml") {
        Ok(tasks) => {
            for task in tasks {
                println!(
                    "Task Desc: {}\nTask due_date: {}\nTask important: {}",
                    task.description, task.due_date, task.important
                );
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
