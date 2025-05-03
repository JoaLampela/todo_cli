use crate::task::{Printable, Task};
use crate::repository::{write_tasks_to_file};
use std::io;
use std::io::Write;

pub fn print_menu() -> () {
    println!("1. Add new task");
    println!("2. List tasks");
    println!("3. Edit tasks");
    println!("4. Delete tasks");
    println!("5. Exit");
}

pub fn clear_terminal() -> () {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

pub fn add_task(tasks: &mut Vec<Task>, id_counter: &mut u32) -> () {
    *id_counter += 1;
    println!("Adding a new task");
    let task: Task = Task::new(*id_counter, read_line("Description:"));
    println!("Added task {}", task.description);
    tasks.push(task);
    write_tasks_to_file(tasks);
}

pub fn list_task_options(tasks: &[Task]) -> () {
    println!("1. List all tasks");
    println!("2. List uncompleted tasks");
    println!("3. List completed tasks");
    println!("4. Cancel");

    let input = read_line("Select what to list:");

    match input.trim() {
        "1" => list_all_tasks(tasks),
        "2" => list_uncompleted_tasks(tasks),
        "3" => list_completed_tasks(tasks),
        "4" => return,
        _ => println!("Invalid option")
    }
}

fn list_all_tasks(tasks: &[Task]) -> () {
    println!("[id] [status] [description]");
    for task in tasks {
        task.print();
    }
}

fn list_uncompleted_tasks(tasks: &[Task]) -> () {
    println!("[id] [status] [description]");
    for task in tasks {
        if task.completed { continue; }
        task.print();
    }
}

fn list_completed_tasks(tasks: &[Task]) -> () {
    println!("[id] [status] [description]");
    for task in tasks {
        if !task.completed { continue; }
        task.print();
    }
}

pub fn edit_tasks_options(tasks: &mut Vec<Task>) -> () {
    println!("1. Edit task description by id");
    println!("2. Toggle task completion by id");
    println!("3. Cancel");

    let input = read_line("Select what to edit:");

    match input.trim() {
        "1" => edit_task_description_by_id(tasks),
        "2" => edit_task_status_by_id(tasks),
        "3" => return,
        _ => println!("Invalid option: {}", input.trim())
    }

    write_tasks_to_file(tasks);
}

fn edit_task_description_by_id(tasks: &mut Vec<Task>) -> () {
    list_all_tasks(tasks);

    let id: u32 = match read_parsed::<u32>("Enter task id:") {
        Some(id) => id,
        None => {
            println!("Invalid id");
            return;
        }
    };

    match tasks.iter().position(|task| task.id == id) {
        Some(index) => {
            let description: String = read_line("New description:");

            if confirm_action() {
                tasks[index] = Task {
                    id: tasks[index].id,
                    description,
                    completed: tasks[index].completed
                };
                println!("Successfully edited task with id {}", id);
            }
        },
        None => println!("Could not find task with id {}", id)
    };
}

fn edit_task_status_by_id(tasks: &mut Vec<Task>) -> () {
    list_all_tasks(tasks);

    let id: u32 = match read_parsed::<u32>("Enter task id:") {
        Some(id) => id,
        None => {
            println!("Invalid id");
            return;
        }
    };

    match tasks.iter().position(|task| task.id == id) {
        Some(index) => if confirm_action() {
            tasks[index].toggle_completed_status();
            println!("Successfully edited task with id {}", id);
        },
        None => println!("Could not find task with id {}", id)
    }
}

pub fn delete_task_options(tasks: &mut Vec<Task>) -> () {
    println!("1. Delete task by id");
    println!("2. Delete all tasks");
    println!("3. Delete uncompleted tasks");
    println!("4. Delete completed tasks");
    println!("5. Cancel");

    let input = read_line("Select what to delete:");

    match input.trim() {
        "1" => delete_task_by_id(tasks),
        "2" => if confirm_action() { tasks.clear(); },
        "3" => delete_uncompleted(tasks),
        "4" => delete_completed(tasks),
        "5" => return,
        _ => println!("Invalid option: {}", input.trim())
    }

    write_tasks_to_file(tasks);
}

fn delete_task_by_id(tasks: &mut Vec<Task>) -> () {
    list_all_tasks(tasks);

    let id: u32 = match read_parsed::<u32>("Enter task id:") {
        Some(id) => id,
        None => {
            println!("Invalid id");
            return;
        }
    };

    if confirm_action() {
        tasks.retain(|task| task.id != id);
        println!("Successfully deleted task with id {}", id);
    }
}

fn delete_uncompleted(tasks: &mut Vec<Task>) -> () {
    println!("Warning! You are about to delete the following task(s):");
    list_uncompleted_tasks(tasks);

    if confirm_action() {
        tasks.retain(|task| task.completed);
        println!("Successfully deleted all uncompleted tasks");
    }
}

fn delete_completed(tasks: &mut Vec<Task>) -> () {
    println!("Warning! You are about to delete the following task(s):");
    list_completed_tasks(tasks);

    if confirm_action() {
        tasks.retain(|task| !task.completed);
        println!("Successfully deleted all completed tasks");
    }
}

pub fn read_parsed<T: std::str::FromStr>(prompt: &str) -> Option<T> {
    let input = read_line(prompt);
    input.parse::<T>().ok()
}

pub fn read_line(prompt: &str) -> String {
    println!("{}", prompt);
    let mut buffer: String = String::new();
    if let Err(e) = io::stdin().read_line(&mut buffer) {
        eprintln!("Error when reading input: {}", e);
    }
    buffer.trim().to_string()
}

pub fn confirm_action() -> bool {
    let input = read_line("Are you sure? (y/N)");
    matches!(input.to_lowercase().trim(), "y" | "yes")
}
