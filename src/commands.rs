use clap::{Parser, Subcommand};
use crate::models::Priority;

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple todo list CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// タスクの追加
    Add {
        /// タスクのタイトル
        title: String,
        /// タスクの説明
        #[arg(short, long)]
        description: Option<String>,
        /// 優先度
        #[arg(short, long, default_value = "medium")]
        priority: String,
    },
    /// タスクの表示
    List {
        /// 完了済みも表示
        #[arg(short, long)]
        all: bool,
    },
    /// タスクの完了
    Complete {
        /// タスクのID
        id: u64,
    },
    /// タスクの削除
    Delete {
        /// タスクのID
        id: u64,
    },
}

impl Commands {
    pub fn parse_priority(priority: &str) -> Priority {
        match priority.to_lowercase().as_str() {
            "low" => Priority::Low,
            "medium" => Priority::Medium,
            "high" => Priority::High,
            _ => Priority::Medium,
        }
    }
}