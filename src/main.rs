use joa_lampela_todo_cli::menu::{add_task, clear_terminal, confirm_action, delete_task_options, edit_tasks_options, list_task_options, print_menu, read_line};
use joa_lampela_todo_cli::repository::{read_tasks_from_file};
use joa_lampela_todo_cli::task::Task;

fn main() -> () {
    clear_terminal();
    let mut tasks: Vec<Task> = read_tasks_from_file();
    let mut buffer: String;
    let mut id_counter: u32 = tasks.iter()
        .map(|task| task.id).max().unwrap_or(0);
    
    loop {
        println!("-----Options-----");
        print_menu();
        buffer = read_line("Please select an option");

        println!("-----Output-----");
        match buffer.trim() {
            "1" => add_task(&mut tasks, &mut id_counter),
            "2" => list_task_options(&tasks),
            "3" => edit_tasks_options(&mut tasks),
            "4" => delete_task_options(&mut tasks),
            "5" => if confirm_action() { break; },
            _ => println!("Invalid option: {}", buffer.trim())
        }

        _ = read_line("Press enter to continue...");
        clear_terminal();
    }
}
