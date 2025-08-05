mod models;
mod storage;
mod commands;

use clap::Parser;
use commands::{Cli, Commands};
use models::{Todo};
use storage::Storage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let storage = Storage::new("todo.json".to_string());
    let mut todo_list = storage.load()?;

    match cli.command {
        Commands::Add { title, description, priority } => {
            let priority = Commands::parse_priority(&priority);
            let todo = Todo::new(title, description, priority);
            let id = todo_list.add_todo(todo);
            println!("タスクを追加しました (ID: {})", id);
        }
        Commands::List { all } => {
            if all {
                for todo in todo_list.get_all_todos() {
                    println!("[{}] {} - {}",
                        if todo.completed { "✓" } else { " " },
                        todo.id, todo.title);
                }
            } else {
                for todo in todo_list.get_pending_todos() {
                    println!("[{}] {} - {}",
                        if todo.completed { "✓" } else { " " },
                        todo.id, todo.title);
                }
            }
        }
        Commands::Complete { id } => {
            if let Some(todo) = todo_list.get_todo_mut(id) {
                todo.mark_completed();
                println!("タスクを完了にしました (ID: {})", id);
            } else {
                println!("タスクが見つかりませんでした (ID: {})", id);
            }
        }
        Commands::Delete { id } => {
            if todo_list.remove_todo(id) {
                println!("タスクを削除しました (ID: {})", id);
            } else {
                println!("タスクが見つかりませんでした (ID: {})", id);
            }
        }
    }
    storage.save(&todo_list)?;
    Ok(())
}
