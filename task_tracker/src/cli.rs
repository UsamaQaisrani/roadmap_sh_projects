use chrono::Utc;
use clap::{Parser, Subcommand};

use crate::tasks::{Task, TaskStatus};

#[derive(clap::Parser)]
#[command(name = "task-cli")]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add { description: String },
    Delete { id: u64 },
    Update { id: u64, description: String },
    List,
}

pub struct Cli {
    tasks: Vec<Task>,
}

impl Cli {
    pub fn new() -> Self {
        let tasks = match Cli::load_existing_tasks() {
            Ok(tasks) => tasks,
            Err(e) => {
                eprintln!("Unable to load tasks: {:?}", e);
                std::process::exit(1);
            }
        };
        Self { tasks }
    }

    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let args = Args::parse();

        match args.command {
            Commands::Add { description } => self.add_task(description)?,

            Commands::Update { id, description } => self.update_task(id, description)?,
            Commands::Delete { id } => {
                println!("Deleting task: {}", id);
            }

            Commands::List => {
                println!("Listing all tasks")
            }
        }

        Ok(())
    }

    fn add_task(&mut self, description: String) -> Result<(), Box<dyn std::error::Error>> {
        let max_id = self.get_max_id();
        let new_task = Task {
            id: max_id,
            description,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: TaskStatus::Todo,
        };
        self.tasks.push(new_task);
        self.save_tasks()?;
        eprintln!("Task Added Successfully (ID: {})", max_id);
        Ok(())
    }

    fn update_task(
        &mut self,
        id: u64,
        description: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.description = description;
            task.updated_at = Utc::now();
        }
        self.save_tasks()?;
        eprintln!("Task Updated Successfully (ID: {})", id);
        Ok(())
    }

    fn get_max_id(&self) -> u64 {
        self.tasks.iter().map(|task| task.id).max().unwrap_or(1_u64)
    }

    fn load_existing_tasks() -> Result<Vec<Task>, Box<dyn std::error::Error>> {
        let path = std::path::PathBuf::from("tasks.json");

        if !path.exists() {
            return Ok(Vec::new());
        }

        let str_content = std::fs::read_to_string(&path)?;
        let tasks: Vec<Task> = serde_json::from_str(&str_content)?;
        Ok(tasks)
    }

    fn save_tasks(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = std::path::PathBuf::from("tasks.json");

        let tasks_json = serde_json::to_string_pretty(&self.tasks)?;
        Ok(std::fs::write(&path, tasks_json)?)
    }
}
