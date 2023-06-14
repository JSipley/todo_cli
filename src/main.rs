use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug)]
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

static mut TASK_MANAGER: TaskManager = TaskManager {
    tasks: Vec::new(),
};

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
    let read_tasks = read_xml("sudo_task_list.xml")
        .expect("Had trouble parsing tasks from xml file");

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
