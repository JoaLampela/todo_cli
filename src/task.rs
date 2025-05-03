use serde::{Deserialize, Serialize};

pub trait Printable {
    fn print(&self) -> ();
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool
}

impl Task {
    pub fn new(id: u32, description: String) -> Task {
        Task {
            id,
            description,
            completed: false
        }
    }

    pub fn get_status(&self) -> &str {
        if self.completed { "X" }
        else { " " }
    }

    pub fn toggle_completed_status(&mut self) -> () {
        self.completed = !self.completed;
    }
}

impl Printable for Task {
    fn print(&self) -> () {
        println!("[{}] [{}] [{}]",
                 self.id,
                 self.get_status(),
                 self.description);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_task_new_initializes_fields() {
        let id = 1;
        let description = "Test".to_string();
        let task = Task::new(id, description.clone());
        assert_eq!(task.id, id);
        assert_eq!(task.description, description);
        assert!(!task.completed);
    }
    
    #[test]
    fn test_toggle_completed_status() {
        let mut task = Task::new(1, "Test".to_string());
        assert!(!task.completed);
        task.toggle_completed_status();
        assert!(task.completed);
        task.toggle_completed_status();
        assert!(!task.completed);
    }
}
