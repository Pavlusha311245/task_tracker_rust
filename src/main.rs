use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: i32,
    description: String,
    status: String,
    created_at: String,
    updated_at: String,
}

fn main() -> () {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: task-cli <command> <options>");
    }

    if args[1].as_str() == String::from("list") {
        if args.len() < 3 {
            list_tasks(None);
        }

        match args[2].as_str() {
            "done" => list_tasks(Some(String::from("done"))),
            "todo" => list_tasks(Some(String::from("todo"))),
            "in-progress" => list_tasks(Some(String::from("in-progress"))),
            _ => list_tasks(None),
        }
    }

    if args[1].as_str() == String::from("add") {
        if args.len() < 3 {
            panic!("Missing description");
        }

        add_task(String::from(args[2].as_str()));
    }

    if args[1].as_str() == String::from("remove") {
        if args.len() < 3 {
            panic!("Missing id");
        }

        remove_task(args[2].parse::<i32>().unwrap());
    }

    if args[1].as_str() == String::from("update") {
        if args.len() < 4 {
            panic!("Missing id");
        }

        update_task_description(
            args[2].parse::<i32>().unwrap(),
            String::from(args[3].as_str()),
        );
    }
    
    let commands = vec!["mark-in-progress", "mark-done"];
    if commands.contains(&args[1].as_str()) {
        if args.len() < 3 {
            panic!("Missing id");
        }
        
        let status = match args[1].as_str() {
            "mark-in-progress" => "in-progress",
            "mark-done" => "done",
            _ => panic!("Invalid command")
        };
        
        change_task_status(args[2].parse::<i32>().unwrap(), status.to_string());
    }
}

fn change_task_status(id: i32, status: String) -> () {
    let mut tasks: Vec<Task> = get_tasks();

    let task_position = tasks.iter().position(|task| task.id == id);

    if task_position.is_none() {
        println!("Task not found");
        return;
    }

    if tasks[task_position.unwrap()].status == "done" {
        println!("Task is already done");
        return;
    }

    tasks[task_position.unwrap()].status = status;
    tasks[task_position.unwrap()].updated_at = chrono::offset::Utc::now().to_rfc3339();

    write_tasks(tasks).ok();

    let updated_task = get_tasks().into_iter().find(|task| task.id == id).unwrap();

    println!(
        "Task marked as {} successfully (ID: {})",
        updated_task.status, id
    );
}

fn add_task(description: String) {
    let mut tasks: Vec<Task> = get_tasks();

    let new_id = if let Some(max_id) = tasks.iter().map(|task| task.id).max() {
        max_id + 1
    } else {
        1
    };

    let date: String = chrono::offset::Utc::now().to_rfc3339();

    let task = Task {
        id: new_id,
        description,
        status: String::from("todo"),
        created_at: date.clone(),
        updated_at: date.clone(),
    };
    
    tasks.push(task);
    
    write_tasks(tasks).ok();
    
    let added_task = get_tasks().into_iter().find(|task| task.id == new_id).unwrap();

    println!("Task added successfully (ID: {})", added_task.id);
}

fn remove_task(id: i32) -> () {
    let tasks: Vec<Task> = get_tasks();

    let task_position = tasks.iter().position(|task| task.id == id);

    if task_position.is_none() {
        println!("Task not found");
        return;
    }

    write_tasks(tasks.into_iter().filter(|task| task.id != id).collect()).ok();

    println!("Task removed successfully (ID: {})", id);
}

fn update_task_description(id: i32, description: String) -> () {
    let mut tasks: Vec<Task> = get_tasks();

    let task_position = tasks.iter().position(|task| task.id == id);

    if task_position.is_none() {
        println!("Task not found");
        return;
    }

    tasks[task_position.unwrap()].description = description;
    tasks[task_position.unwrap()].updated_at = chrono::offset::Utc::now().to_rfc3339();

    write_tasks(tasks).ok();

    println!("Task updated successfully (ID: {})", id);
}

fn list_tasks(status: Option<String>) {
    let tasks = get_tasks();

    let filtered_tasks = match status {
        Some(status) => tasks
            .into_iter()
            .filter(|task| task.status == status)
            .collect(),
        None => tasks,
    };

    for task in filtered_tasks {
        println!(
            "ID: {},\nDescription: {},\nStatus: {},\nCreated At: {},\nUpdated At: {}\n",
            task.id, task.description, task.status, task.created_at, task.updated_at
        );
    }
}

fn get_tasks() -> Vec<Task> {
    let is_file_exists = Path::exists("tasks.json".as_ref());

    if !is_file_exists {
        File::create("tasks.json").unwrap();
    }

    let mut file: File = File::open("tasks.json").unwrap();
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();

    let tasks: Vec<Task> = serde_json::from_str(&file_content).unwrap_or_else(|_| Vec::new());

    tasks
}

fn write_tasks(tasks: Vec<Task>) -> Result<(), std::io::Error> {
    let json_string = serde_json::to_string_pretty(&tasks)?;

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("tasks.json")?;

    file.write_all(json_string.as_bytes())?;
    Ok(())
}
