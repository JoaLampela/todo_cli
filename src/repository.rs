use crate::task::Task;
use std::fs;
use std::path::PathBuf;

pub fn init_save_dir() -> Option<PathBuf> {
    let path: PathBuf = match dirs::data_dir() {
        Some(path) => path.join("todo_cli"),
        None => return None
    };

    match fs::create_dir_all(&path) {
        Ok(_) => Some(path),
        Err(e) => {
            eprintln!("Error when creating app directory: {}", e);
            None
        }
    }
}

pub fn write_tasks_to_file(data: &[Task]) -> () {
    let path: PathBuf = match init_save_dir() {
        Some(path) => path,
        None => return
    };

    let json: String = serde_json::to_string(&data).unwrap_or(String::new());

    if let Err(e) = fs::write(path.join("tasks.json"), &json) {
        eprintln!("Error when writing to save file: {}", e)
    };
}

pub fn read_tasks_from_file() -> Vec<Task> {
    let path: PathBuf = match init_save_dir() {
        Some(path) => path,
        None => return Vec::new()
    };

    let json: String = fs::read_to_string(path.join("tasks.json")).unwrap_or(String::new());

    serde_json::from_str(&json).unwrap_or(Vec::new())
}
