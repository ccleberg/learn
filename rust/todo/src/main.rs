use clap::{Arg, Command};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

impl Task {
    fn to_string(&self) -> String {
        let status = if self.completed { "[X]" } else { "[ ]" };
        format!("{} {}", status, self.description)
    }
}

fn main() {
    let matches = Command::new("Task Echo")
        .version("1.0")
        .about("Echoes tasks to a file")
        .arg(
            Arg::new("task")
                .short('t')
                .long("task")
                .value_name("TASK")
                .help("Sets the task to be saved")
                .num_args(1..),
        )
        .arg(
            Arg::new("done")
                .short('d')
                .long("done")
                .value_name("TASK_ID")
                .help("Marks the specified task(s) as complete")
                .num_args(1..),
        )
        .get_matches();

    let file_path = "tasks.txt";

    if let Some(ids) = matches.get_many::<String>("done") {
        mark_done(file_path, ids);
    }

    if let Some(new_tasks) = matches.get_many::<String>("task") {
        add_tasks(file_path, new_tasks);
    }

    // Display saved tasks if no new tasks or done tasks were provided
    if !matches.contains_id("task") && !matches.contains_id("done") {
        display_tasks(file_path);
    }
}

fn mark_done<'a>(file_path: &str, ids: impl Iterator<Item = &'a String>) {
    let mut tasks = load_tasks(file_path);
    let mut missing = Vec::new();
    for id in ids {
        if let Ok(index) = id.parse::<usize>() {
            if index > 0 && index <= tasks.len() {
                tasks[index - 1].completed = true;
            } else {
                missing.push(id.clone());
            }
        }
    }
    save_tasks(file_path, &tasks);

    if missing.is_empty() {
        println!("Tasks marked as complete.");
    }
    for id in missing {
        println!("Task {} doesn't exist.", id);
    }

    display_tasks(file_path);
}

fn add_tasks<'a>(file_path: &str, new_tasks: impl Iterator<Item = &'a String>) {
    let mut tasks = load_tasks(file_path);
    let next_id = tasks.len() + 1;
    for task_desc in new_tasks {
        tasks.push(Task {
            id: next_id,
            description: task_desc.to_string(),
            completed: false,
        });
    }
    save_tasks(file_path, &tasks);
    println!("Tasks saved to {}", file_path);

    display_tasks(file_path);
}

fn load_tasks(file_path: &str) -> Vec<Task> {
    let mut tasks = Vec::new();
    if Path::new(file_path).exists() {
        let contents = fs::read_to_string(file_path).expect("Unable to read file");
        for (id, line) in contents.lines().enumerate() {
            let completed = line.starts_with("[X]");
            let description = line[4..].trim().to_string();
            tasks.push(Task {
                id: id + 1,
                description,
                completed,
            });
        }
    }
    tasks
}

fn save_tasks(file_path: &str, tasks: &[Task]) {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true) // Clear the file before writing
        .open(file_path)
        .expect("Unable to open file");

    for task in tasks {
        writeln!(file, "{}", task.to_string()).expect("Unable to write to file");
    }
}

fn display_tasks(file_path: &str) {
    let tasks = load_tasks(file_path);
    if tasks.is_empty() {
        println!("No tasks saved yet.");
    } else {
        println!("Saved tasks:");
        for task in tasks {
            // Pad the task ID to a width of 3 for alignment
            println!("{:>3}: {}", task.id, task.to_string()); // Display task ID
        }
    }
}
