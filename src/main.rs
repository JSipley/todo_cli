use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

struct Task {
    description: String,
    due_date: String,
    important: String,
}

fn read_xml(filename: &str) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let mut task_list: Vec<Task> = Vec::new();
    Ok(task_list)
}

fn main() {
    println!("Hello, world!");
}
