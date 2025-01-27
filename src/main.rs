use std::env;

struct Task {
    id: i32,
    description: String,
    status: String,
    created_at: String,
    updated_at: String,
}

fn main() {
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

    // println!("{:?}", args);

    // TODO: crud
    // TODO: Mark a task as in progress or done
    // TODO: List all tasks
    // TODO: List all tasks that are done
    // TODO: List all tasks that are not done
    // TODO: List all tasks that are in progress

    // task-cli add/delete/update <id> <options
    // task-cli mark-in-progress/done
    // task-cli list
    // TODO: task-cli list done
    // TODO: task-cli list todo
    // TODO: task-cli list in-progress
}

fn mark_in_progress(id: i32) -> () {
    //
}

fn mark_done(id: i32) -> () {
    //
}

fn add_task(description: String) -> () {
    // Task {
    //     name,
    //     status: String::from("todo"),
    // };
}

fn remove_task(id: i32) -> () {
    //
}

fn update_task(id: i32, description: String) -> () {
    //
}

fn list_tasks(status: Option<String>) -> () {
    let vector: Vec<Task> = Vec::new();

    println!("List of tasks with status: {:?}", status);
}
